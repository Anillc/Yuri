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

  fn translate(&mut self, address: u64, hart: &Hart, access: AccessType) -> Result<u64, Exception> {
    let satp = SATP::from_u64(hart.csr.read_satp());
    if satp.mode != 8 { return Ok(address); }
    let (mprv, mpp, sum, mxr) = hart.csr.read_mstatus_mprv_mpp_sum_mxr();
    // MPRV only affects load and store
    let effective_mode = if mprv && access != AccessType::Execute {
      mpp
    } else {
      hart.mode
    };
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

  fn misaligned_read<const LEN: usize>(&mut self, hart: &Hart, address: u64) -> Result<[u8; LEN], Exception> {
    let address_low = self.translate(address, hart, AccessType::Read)?;
    // next page
    let address_high = self.translate(address + LEN as u64, hart, AccessType::Read)?;
    let high_len = (address % LEN as u64) as usize;
    let low_len = LEN - high_len;
    let mut bytes: [u8; LEN] = [0; LEN];
    for i in 0..LEN {
      if i < low_len {
        bytes[i] = self.bus.read8(address_low + i as u64)?;
      } else {
        bytes[i] = self.bus.read8(address_high + i as u64 - LEN as u64)?;
      }
    }
    Ok(bytes)
  }

  fn misaligned_write<const LEN: usize>(&mut self, hart: &Hart, address: u64, data: [u8; LEN]) -> Result<(), Exception> {
    let address_low = self.translate(address, hart, AccessType::Read)?;
    // next page
    let address_high = self.translate(address + LEN as u64, hart, AccessType::Read)?;
    let high_len = (address % LEN as u64) as usize;
    let low_len = LEN - high_len;
    for i in 0..LEN {
      if i < low_len {
        self.bus.write8(address_low + i as u64, data[i])?;
      } else {
        self.bus.write8(address_high + i as u64 - LEN as u64, data[i])?;
      }
    }
    Ok(())
  }

  pub(crate) fn fetch(&mut self, hart: &Hart, address: u64) -> Result<InstructionWithType, Exception> {
    debug_assert!(address % 2 == 0);
    let address_low = self.translate(address, hart, AccessType::Execute)?;
    let instruction_low = self.bus.read16(address_low)
      .map_err(|_| Exception::InstructionAccessFault(address))?;
    if instruction_low & 0b11 != 0b11 {
      return Ok(InstructionWithType::L16(instruction_low));
    }
    if address % 4 == 0 {
      let instruction_high = self.bus.read16(address_low + 2)
        .map_err(|_| Exception::InstructionAccessFault(address))? as u32;
      return Ok(InstructionWithType::L32(instruction_high << 16 | instruction_low as u32));
    } else {
      let address_high = self.translate(address + 2, hart, AccessType::Execute)?;
      let instruction_high = self.bus.read16(address_high)
        .map_err(|_| Exception::InstructionAccessFault(address))? as u32;
      return Ok(InstructionWithType::L32(instruction_high << 16 | instruction_low as u32));
    }
  }

  pub(crate) fn read8(&mut self, hart: &Hart, address: u64) -> Result<u8, Exception> {
    let address = self.translate(address, hart, AccessType::Read)?;
    Ok(self.bus.read8(address)?)
  }
  pub(crate) fn read16(&mut self, hart: &Hart, address: u64) -> Result<u16, Exception> {
    if address % 2 == 0 {
      let address = self.translate(address, hart, AccessType::Read)?;
      Ok(self.bus.read16(address)?)
    } else {
      Ok(u16::from_le_bytes(self.misaligned_read(hart, address)?))
    }
  }
  pub(crate) fn read32(&mut self, hart: &Hart, address: u64) -> Result<u32, Exception> {
    if address % 4 == 0 {
      let address = self.translate(address, hart, AccessType::Read)?;
      Ok(self.bus.read32(address)?)
    } else {
      Ok(u32::from_le_bytes(self.misaligned_read(hart, address)?))
    }
  }
  pub(crate) fn read64(&mut self, hart: &Hart, address: u64) -> Result<u64, Exception> {
    if address % 8 == 0 {
      let address = self.translate(address, hart, AccessType::Read)?;
      Ok(self.bus.read64(address)?)
    } else {
      Ok(u64::from_le_bytes(self.misaligned_read(hart, address)?))
    }
  }
  pub(crate) fn write8(&mut self, hart: &Hart, address: u64, data: u8) -> Result<(), Exception> {
    let address = self.translate(address, hart, AccessType::Write)?;
    self.bus.write8(address, data)?;
    Ok(())
  }
  pub(crate) fn write16(&mut self, hart: &Hart, address: u64, data: u16) -> Result<(), Exception> {
    if address % 2 == 0 {
      let address = self.translate(address, hart, AccessType::Write)?;
      self.bus.write16(address, data)?;
    } else {
      self.misaligned_write(hart, address, data.to_le_bytes())?;
    }
    Ok(())
  }
  pub(crate) fn write32(&mut self, hart: &Hart, address: u64, data: u32) -> Result<(), Exception> {
    if address % 4 == 0 {
      let address = self.translate(address, hart, AccessType::Write)?;
      self.bus.write32(address, data)?;
    } else {
      self.misaligned_write(hart, address, data.to_le_bytes())?;
    }
    Ok(())
  }
  pub(crate) fn write64(&mut self, hart: &Hart, address: u64, data: u64) -> Result<(), Exception> {
    if address % 8 == 0 {
      let address = self.translate(address, hart, AccessType::Write)?;
      self.bus.write64(address, data)?;
    } else {
      self.misaligned_write(hart, address, data.to_le_bytes())?;
    }
    Ok(())
  }
  pub(crate) fn atomic_swap32(&mut self, hart: &Hart, address: u64, val: u32, ordering: Ordering) -> Result<u32, Exception> {
    if address % 4 != 0 { return Err(Exception::StoreAMOAddressMisaligned(address)); }
    let address = self.translate(address, hart, AccessType::ReadWrite)?;
    Ok(self.bus.atomic_swap32(address, val, ordering)?)
  }
  pub(crate) fn atomic_swap64(&mut self, hart: &Hart, address: u64, val: u64, ordering: Ordering) -> Result<u64, Exception> {
    if address % 8 != 0 { return Err(Exception::StoreAMOAddressMisaligned(address)); }
    let address = self.translate(address, hart, AccessType::ReadWrite)?;
    Ok(self.bus.atomic_swap64(address, val, ordering)?)
  }
  pub(crate) fn atomic_add32(&mut self, hart: &Hart, address: u64, val: u32, ordering: Ordering) -> Result<u32, Exception> {
    if address % 4 != 0 { return Err(Exception::StoreAMOAddressMisaligned(address)); }
    let address = self.translate(address, hart, AccessType::ReadWrite)?;
    Ok(self.bus.atomic_add32(address, val, ordering)?)
  }
  pub(crate) fn atomic_add64(&mut self, hart: &Hart, address: u64, val: u64, ordering: Ordering) -> Result<u64, Exception> {
    if address % 8 != 0 { return Err(Exception::StoreAMOAddressMisaligned(address)); }
    let address = self.translate(address, hart, AccessType::ReadWrite)?;
    Ok(self.bus.atomic_add64(address, val, ordering)?)
  }
  pub(crate) fn atomic_xor32(&mut self, hart: &Hart, address: u64, val: u32, ordering: Ordering) -> Result<u32, Exception> {
    if address % 4 != 0 { return Err(Exception::StoreAMOAddressMisaligned(address)); }
    let address = self.translate(address, hart, AccessType::ReadWrite)?;
    Ok(self.bus.atomic_xor32(address, val, ordering)?)
  }
  pub(crate) fn atomic_xor64(&mut self, hart: &Hart, address: u64, val: u64, ordering: Ordering) -> Result<u64, Exception> {
    if address % 8 != 0 { return Err(Exception::StoreAMOAddressMisaligned(address)); }
    let address = self.translate(address, hart, AccessType::ReadWrite)?;
    Ok(self.bus.atomic_xor64(address, val, ordering)?)
  }
  pub(crate) fn atomic_and32(&mut self, hart: &Hart, address: u64, val: u32, ordering: Ordering) -> Result<u32, Exception> {
    if address % 4 != 0 { return Err(Exception::StoreAMOAddressMisaligned(address)); }
    let address = self.translate(address, hart, AccessType::ReadWrite)?;
    Ok(self.bus.atomic_and32(address, val, ordering)?)
  }
  pub(crate) fn atomic_and64(&mut self, hart: &Hart, address: u64, val: u64, ordering: Ordering) -> Result<u64, Exception> {
    if address % 8 != 0 { return Err(Exception::StoreAMOAddressMisaligned(address)); }
    let address = self.translate(address, hart, AccessType::ReadWrite)?;
    Ok(self.bus.atomic_and64(address, val, ordering)?)
  }
  pub(crate) fn atomic_or32(&mut self, hart: &Hart, address: u64, val: u32, ordering: Ordering) -> Result<u32, Exception> {
    if address % 4 != 0 { return Err(Exception::StoreAMOAddressMisaligned(address)); }
    let address = self.translate(address, hart, AccessType::ReadWrite)?;
    Ok(self.bus.atomic_or32(address, val, ordering)?)
  }
  pub(crate) fn atomic_or64(&mut self, hart: &Hart, address: u64, val: u64, ordering: Ordering) -> Result<u64, Exception> {
    if address % 8 != 0 { return Err(Exception::StoreAMOAddressMisaligned(address)); }
    let address = self.translate(address, hart, AccessType::ReadWrite)?;
    Ok(self.bus.atomic_or64(address, val, ordering)?)
  }
  pub(crate) fn atomic_min_i32(&mut self, hart: &Hart, address: u64, val: i32, ordering: Ordering) -> Result<i32, Exception> {
    if address % 4 != 0 { return Err(Exception::StoreAMOAddressMisaligned(address)); }
    let address = self.translate(address, hart, AccessType::ReadWrite)?;
    Ok(self.bus.atomic_min_i32(address, val, ordering)?)
  }
  pub(crate) fn atomic_min_i64(&mut self, hart: &Hart, address: u64, val: i64, ordering: Ordering) -> Result<i64, Exception> {
    if address % 8 != 0 { return Err(Exception::StoreAMOAddressMisaligned(address)); }
    let address = self.translate(address, hart, AccessType::ReadWrite)?;
    Ok(self.bus.atomic_min_i64(address, val, ordering)?)
  }
  pub(crate) fn atomic_max_i32(&mut self, hart: &Hart, address: u64, val: i32, ordering: Ordering) -> Result<i32, Exception> {
    if address % 4 != 0 { return Err(Exception::StoreAMOAddressMisaligned(address)); }
    let address = self.translate(address, hart, AccessType::ReadWrite)?;
    Ok(self.bus.atomic_max_i32(address, val, ordering)?)
  }
  pub(crate) fn atomic_max_i64(&mut self, hart: &Hart, address: u64, val: i64, ordering: Ordering) -> Result<i64, Exception> {
    if address % 8 != 0 { return Err(Exception::StoreAMOAddressMisaligned(address)); }
    let address = self.translate(address, hart, AccessType::ReadWrite)?;
    Ok(self.bus.atomic_max_i64(address, val, ordering)?)
  }
  pub(crate) fn atomic_min_u32(&mut self, hart: &Hart, address: u64, val: u32, ordering: Ordering) -> Result<u32, Exception> {
    if address % 4 != 0 { return Err(Exception::StoreAMOAddressMisaligned(address)); }
    let address = self.translate(address, hart, AccessType::ReadWrite)?;
    Ok(self.bus.atomic_min_u32(address, val, ordering)?)
  }
  pub(crate) fn atomic_min_u64(&mut self, hart: &Hart, address: u64, val: u64, ordering: Ordering) -> Result<u64, Exception> {
    if address % 8 != 0 { return Err(Exception::StoreAMOAddressMisaligned(address)); }
    let address = self.translate(address, hart, AccessType::ReadWrite)?;
    Ok(self.bus.atomic_min_u64(address, val, ordering)?)
  }
  pub(crate) fn atomic_max_u32(&mut self, hart: &Hart, address: u64, val: u32, ordering: Ordering) -> Result<u32, Exception> {
    if address % 4 != 0 { return Err(Exception::StoreAMOAddressMisaligned(address)); }
    let address = self.translate(address, hart, AccessType::ReadWrite)?;
    Ok(self.bus.atomic_max_u32(address, val, ordering)?)
  }
  pub(crate) fn atomic_max_u64(&mut self, hart: &Hart, address: u64, val: u64, ordering: Ordering) -> Result<u64, Exception> {
    if address % 8 != 0 { return Err(Exception::StoreAMOAddressMisaligned(address)); }
    let address = self.translate(address, hart, AccessType::ReadWrite)?;
    Ok(self.bus.atomic_max_u64(address, val, ordering)?)
  }
}
