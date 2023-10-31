use std::sync::{Arc, Mutex, atomic::Ordering};

use crate::{devices::{bus::Bus, Device}, hart::{Hart, Mode}, trap::Exception, instructions::InstructionWithType};

const PAGESIZE: u64 = 4096;
const LEVELS: usize = 3;
const PTESIZE: u64 = 8;

struct SATP {
  mode: u64,
  ppn: u64,
}

impl SATP {
  pub(crate) fn from_u64(data: u64) -> SATP {
    SATP {
      mode: data >> 60,
      ppn: data & 0b11111111111111111111111111111111111111111111,
    }
  }
}

struct VirtualAddress {
  invalid: bool,
  vpn: [u64; 3],
  page_offset: u64,
}

impl VirtualAddress {
  pub(crate) fn from_u64(data: u64) -> VirtualAddress {
    VirtualAddress {
      invalid: (data >> 39) != if (data >> 38) & 0b1 == 1 {
        0b1111111111111111111111111
      } else { 0 },
      vpn: [
        (data >> 12) & 0b111111111,
        (data >> 21) & 0b111111111,
        (data >> 30) & 0b111111111,
      ],
      page_offset: data & 0b111111111111,
    }
  }
}

struct PTE {
  invalid: bool,
  ppn: u64,
  ppns: [u64; 3],
  // rsw: u64,
  d: bool,
  a: bool,
  // g: bool,
  u: bool,
  x: bool,
  w: bool,
  r: bool,
  v: bool,
}

impl PTE {
  pub(crate) fn from_u64(data: u64) -> PTE {
    PTE {
      invalid: data >> 54 != 0,
      ppn: (data >> 10) & 0b11111111111111111111111111111111111111111111,
      ppns: [
        (data >> 10) & 0b111111111,
        (data >> 19) & 0b111111111,
        (data >> 28) & 0b11111111111111111111111111,
      ],
      // rsw: (data >> 8) & 0b11,
      d: (data >> 7) & 0b1 == 1,
      a: (data >> 6) & 0b1 == 1,
      // g: (data >> 5) & 0b1 == 1,
      u: (data >> 4) & 0b1 == 1,
      x: (data >> 3) & 0b1 == 1,
      w: (data >> 2) & 0b1 == 1,
      r: (data >> 1) & 0b1 == 1,
      v: data & 0b1 == 1,
    }
  }
}

#[derive(Debug, PartialEq, Eq)]
enum AccessType {
  // ReadWrite is for atomic functions
  Execute, Read, Write, ReadWrite,
}

#[derive(Debug, Clone)]
pub(crate) struct MMU {
  bus: Bus,
  reservation: Arc<Mutex<Vec<u64>>>,
}

impl MMU {
  pub(crate) fn new(bus: Bus) -> MMU {
    MMU {
      bus,
      reservation: Arc::new(Mutex::new(Vec::new())),
    }
  }

  pub(crate) fn lock_addr(&mut self, address: u64) {
    self.reservation.lock().unwrap().push(address);
  }

  // true -> exist
  // false -> non-exist
  pub(crate) fn unlock_addr(&mut self, address: u64) -> bool {
    let mut reservation = self.reservation.lock().unwrap();
    let res = reservation.contains(&address);
    reservation.clear();
    res
  }

  fn translate(&self, address: u64, hart: &Hart, access: AccessType) -> Result<u64, Exception> {
    let satp = SATP::from_u64(hart.csr.read_satp());
    if satp.mode != 8 { return Ok(address); }
    let (mprv, mpp, sum, mxr) = hart.csr.read_mstatus_mprv_mpp_sum_mxr();
    let effective_mode = if mprv { mpp } else { hart.mode };
    if effective_mode == Mode::Machine { return Ok(address); }

    #[inline]
    fn fault(address: u64, access: AccessType) -> Result<u64, Exception> {
      match access {
          AccessType::Execute => Err(Exception::InstructionPageFault(address)),
          AccessType::Read => Err(Exception::LoadPageFault(address)),
          AccessType::Write
        | AccessType::ReadWrite => Err(Exception::StoreAMOPageFault(address)),
      }
    }

    let va = VirtualAddress::from_u64(address);
    if va.invalid { return fault(address, access) }

    let mut a = satp.ppn * PAGESIZE;
    let mut i = LEVELS - 1;

    let pte = loop {
      let pte = PTE::from_u64(self.bus.read64(a + va.vpn[i] * PTESIZE)?);
      if pte.invalid || !pte.v || (!pte.r && pte.w) { return fault(address, access); }
      if pte.r || pte.x {
        break pte;
      }
      if i == 0 { return fault(address, access); }
      i -= 1;
      a = pte.ppn * PAGESIZE;
    };

    let valid = match access {
      AccessType::Execute => pte.x,
      AccessType::Read => pte.r || (pte.x && mxr),
      AccessType::Write => pte.w,
      AccessType::ReadWrite => pte.r && pte.w,
    };
    if !valid { return fault(address, access); }

    if (effective_mode == Mode::User && !pte.u) ||
      (pte.u && effective_mode == Mode::Supervisor && !sum) {
        return fault(address, access);
    }

    if i > 0 {
      for j in 0..i {
        if pte.ppns[j] != 0 { return fault(address, access); }
      }
    }

    // TODO: implement A D
    if !pte.a ||
      ((access == AccessType::Write || access == AccessType::ReadWrite) && !pte.d) {
        return fault(address, access);
    }

    let mut pa: u64 = 0;
    if i > 0 {
      for j in 0..i {
        pa |= va.vpn[j] << (12 + j * 9);
      }
    }
    for j in i..LEVELS {
      pa |= pte.ppns[j] << (12 + j * 9)
    }
    pa |= va.page_offset;

    Ok(pa)
  }

  pub(crate) fn fetch(&self, hart: &Hart, address: u64) -> Result<InstructionWithType, Exception> {
    debug_assert!(address % 2 == 0);
    let address_low = self.translate(address, hart, AccessType::Execute)?;
    let instruction_low = self.bus.read16(address_low)?;
    if instruction_low & 0b11 != 0b11 {
      return Ok(InstructionWithType::L16(instruction_low));
    }
    if address % 4 == 0 {
      let instruction_high = self.bus.read16(address_low + 2)? as u32;
      return Ok(InstructionWithType::L32(instruction_high << 16 | instruction_low as u32));
    } else {
      let address_high = self.translate(address + 2, hart, AccessType::Execute)?;
      let instruction_high = self.bus.read16(address_high)? as u32;
      return Ok(InstructionWithType::L32(instruction_high << 16 | instruction_low as u32));
    }
  }

  pub(crate) fn read8(&self, hart: &Hart, address: u64) -> Result<u8, Exception> {
    Ok(self.bus.read8(self.translate(address, hart, AccessType::Read)?)?)
  }
  pub(crate) fn read16(&self, hart: &Hart, address: u64) -> Result<u16, Exception> {
    if address % 2 != 0 { return Err(Exception::LoadAddressMisaligned(address)); }
    Ok(self.bus.read16(self.translate(address, hart, AccessType::Read)?)?)
  }
  pub(crate) fn read32(&self, hart: &Hart, address: u64) -> Result<u32, Exception> {
    if address % 4 != 0 { return Err(Exception::LoadAddressMisaligned(address)); }
    Ok(self.bus.read32(self.translate(address, hart, AccessType::Read)?)?)
  }
  pub(crate) fn read64(&self, hart: &Hart, address: u64) -> Result<u64, Exception> {
    if address % 8 != 0 { return Err(Exception::LoadAddressMisaligned(address)); }
    Ok(self.bus.read64(self.translate(address, hart, AccessType::Read)?)?)
  }
  pub(crate) fn write8(&mut self, hart: &Hart, address: u64, data: u8) -> Result<(), Exception> {
    self.bus.write8(self.translate(address, hart, AccessType::Write)?, data)?;
    Ok(())
  }
  pub(crate) fn write16(&mut self, hart: &Hart, address: u64, data: u16) -> Result<(), Exception> {
    if address % 2 != 0 { return Err(Exception::StoreAMOAddressMisaligned(address)); }
    self.bus.write16(self.translate(address, hart, AccessType::Write)?, data)?;
    Ok(())
  }
  pub(crate) fn write32(&mut self, hart: &Hart, address: u64, data: u32) -> Result<(), Exception> {
    if address % 4 != 0 { return Err(Exception::StoreAMOAddressMisaligned(address)); }
    self.bus.write32(self.translate(address, hart, AccessType::Write)?, data)?;
    Ok(())
  }
  pub(crate) fn write64(&mut self, hart: &Hart, address: u64, data: u64) -> Result<(), Exception> {
    if address % 8 != 0 { return Err(Exception::StoreAMOAddressMisaligned(address)); }
    self.bus.write64(self.translate(address, hart, AccessType::Write)?, data)?;
    Ok(())
  }
  pub(crate) fn atomic_swap32(&mut self, hart: &Hart, address: u64, val: u32, ordering: Ordering) -> Result<u32, Exception> {
    if address % 4 != 0 { return Err(Exception::StoreAMOAddressMisaligned(address)); }
    Ok(self.bus.atomic_swap32(self.translate(address, hart, AccessType::ReadWrite)?, val, ordering)?)
  }
  pub(crate) fn atomic_swap64(&mut self, hart: &Hart, address: u64, val: u64, ordering: Ordering) -> Result<u64, Exception> {
    if address % 8 != 0 { return Err(Exception::StoreAMOAddressMisaligned(address)); }
    Ok(self.bus.atomic_swap64(self.translate(address, hart, AccessType::ReadWrite)?, val, ordering)?)
  }
  pub(crate) fn atomic_add32(&mut self, hart: &Hart, address: u64, val: u32, ordering: Ordering) -> Result<u32, Exception> {
    if address % 4 != 0 { return Err(Exception::StoreAMOAddressMisaligned(address)); }
    Ok(self.bus.atomic_add32(self.translate(address, hart, AccessType::ReadWrite)?, val, ordering)?)
  }
  pub(crate) fn atomic_add64(&mut self, hart: &Hart, address: u64, val: u64, ordering: Ordering) -> Result<u64, Exception> {
    if address % 8 != 0 { return Err(Exception::StoreAMOAddressMisaligned(address)); }
    Ok(self.bus.atomic_add64(self.translate(address, hart, AccessType::ReadWrite)?, val, ordering)?)
  }
  pub(crate) fn atomic_xor32(&mut self, hart: &Hart, address: u64, val: u32, ordering: Ordering) -> Result<u32, Exception> {
    if address % 4 != 0 { return Err(Exception::StoreAMOAddressMisaligned(address)); }
    Ok(self.bus.atomic_xor32(self.translate(address, hart, AccessType::ReadWrite)?, val, ordering)?)
  }
  pub(crate) fn atomic_xor64(&mut self, hart: &Hart, address: u64, val: u64, ordering: Ordering) -> Result<u64, Exception> {
    if address % 8 != 0 { return Err(Exception::StoreAMOAddressMisaligned(address)); }
    Ok(self.bus.atomic_xor64(self.translate(address, hart, AccessType::ReadWrite)?, val, ordering)?)
  }
  pub(crate) fn atomic_and32(&mut self, hart: &Hart, address: u64, val: u32, ordering: Ordering) -> Result<u32, Exception> {
    if address % 4 != 0 { return Err(Exception::StoreAMOAddressMisaligned(address)); }
    Ok(self.bus.atomic_and32(self.translate(address, hart, AccessType::ReadWrite)?, val, ordering)?)
  }
  pub(crate) fn atomic_and64(&mut self, hart: &Hart, address: u64, val: u64, ordering: Ordering) -> Result<u64, Exception> {
    if address % 8 != 0 { return Err(Exception::StoreAMOAddressMisaligned(address)); }
    Ok(self.bus.atomic_and64(self.translate(address, hart, AccessType::ReadWrite)?, val, ordering)?)
  }
  pub(crate) fn atomic_or32(&mut self, hart: &Hart, address: u64, val: u32, ordering: Ordering) -> Result<u32, Exception> {
    if address % 4 != 0 { return Err(Exception::StoreAMOAddressMisaligned(address)); }
    Ok(self.bus.atomic_or32(self.translate(address, hart, AccessType::ReadWrite)?, val, ordering)?)
  }
  pub(crate) fn atomic_or64(&mut self, hart: &Hart, address: u64, val: u64, ordering: Ordering) -> Result<u64, Exception> {
    if address % 8 != 0 { return Err(Exception::StoreAMOAddressMisaligned(address)); }
    Ok(self.bus.atomic_or64(self.translate(address, hart, AccessType::ReadWrite)?, val, ordering)?)
  }
  pub(crate) fn atomic_min_i32(&mut self, hart: &Hart, address: u64, val: i32, ordering: Ordering) -> Result<i32, Exception> {
    if address % 4 != 0 { return Err(Exception::StoreAMOAddressMisaligned(address)); }
    Ok(self.bus.atomic_min_i32(self.translate(address, hart, AccessType::ReadWrite)?, val, ordering)?)
  }
  pub(crate) fn atomic_min_i64(&mut self, hart: &Hart, address: u64, val: i64, ordering: Ordering) -> Result<i64, Exception> {
    if address % 8 != 0 { return Err(Exception::StoreAMOAddressMisaligned(address)); }
    Ok(self.bus.atomic_min_i64(self.translate(address, hart, AccessType::ReadWrite)?, val, ordering)?)
  }
  pub(crate) fn atomic_max_i32(&mut self, hart: &Hart, address: u64, val: i32, ordering: Ordering) -> Result<i32, Exception> {
    if address % 4 != 0 { return Err(Exception::StoreAMOAddressMisaligned(address)); }
    Ok(self.bus.atomic_max_i32(self.translate(address, hart, AccessType::ReadWrite)?, val, ordering)?)
  }
  pub(crate) fn atomic_max_i64(&mut self, hart: &Hart, address: u64, val: i64, ordering: Ordering) -> Result<i64, Exception> {
    if address % 8 != 0 { return Err(Exception::StoreAMOAddressMisaligned(address)); }
    Ok(self.bus.atomic_max_i64(self.translate(address, hart, AccessType::ReadWrite)?, val, ordering)?)
  }
  pub(crate) fn atomic_min_u32(&mut self, hart: &Hart, address: u64, val: u32, ordering: Ordering) -> Result<u32, Exception> {
    if address % 4 != 0 { return Err(Exception::StoreAMOAddressMisaligned(address)); }
    Ok(self.bus.atomic_min_u32(self.translate(address, hart, AccessType::ReadWrite)?, val, ordering)?)
  }
  pub(crate) fn atomic_min_u64(&mut self, hart: &Hart, address: u64, val: u64, ordering: Ordering) -> Result<u64, Exception> {
    if address % 8 != 0 { return Err(Exception::StoreAMOAddressMisaligned(address)); }
    Ok(self.bus.atomic_min_u64(self.translate(address, hart, AccessType::ReadWrite)?, val, ordering)?)
  }
  pub(crate) fn atomic_max_u32(&mut self, hart: &Hart, address: u64, val: u32, ordering: Ordering) -> Result<u32, Exception> {
    if address % 4 != 0 { return Err(Exception::StoreAMOAddressMisaligned(address)); }
    Ok(self.bus.atomic_max_u32(self.translate(address, hart, AccessType::ReadWrite)?, val, ordering)?)
  }
  pub(crate) fn atomic_max_u64(&mut self, hart: &Hart, address: u64, val: u64, ordering: Ordering) -> Result<u64, Exception> {
    if address % 8 != 0 { return Err(Exception::StoreAMOAddressMisaligned(address)); }
    Ok(self.bus.atomic_max_u64(self.translate(address, hart, AccessType::ReadWrite)?, val, ordering)?)
  }
}
