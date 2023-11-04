use crate::{device_atomic, hart::Hart, trap::Exception};

use super::{Device, bus::Bus};

pub(crate) const PLIC_START: u64 = 0x0C000000;
pub(crate) const PLIC_END: u64 = PLIC_START + 0x3FFFFFF;

pub(crate) const PLIC_SOURCE_PRIORITY_START: u64 = PLIC_START + 0x000004;
pub(crate) const PLIC_SOURCE_PRIORITY_END: u64 = PLIC_START + 0x000FFF;

pub(crate) const PLIC_PENDING_START: u64 = PLIC_START + 0x001000;
pub(crate) const PLIC_PENDING_END: u64 = PLIC_START + 0x00107F;

pub(crate) const PLIC_SOURCE_ENABLE_START: u64 = PLIC_START + 0x002000;
pub(crate) const PLIC_SOURCE_ENABLE_END: u64 = PLIC_START + 0x1F1FFF;

pub(crate) const PLIC_THRESHOLD_CLIAM_COMPLETE_START: u64 = PLIC_START + 0x200000;
pub(crate) const PLIC_THRESHOLD_CLIAM_COMPLETE_END: u64 = PLIC_START + 0x3FFFFFF;

// TODO: hart count
const HART_COUNT: usize = 1;
const INTERRUPT_COUNT: usize = 64;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Pair<T: Clone + Copy> {
  machine: T,
  supervisor: T,
}

impl<T: Clone + Copy> Pair<T> {
  pub(crate) fn at(&self, index: usize) -> &T {
    match index {
      0 => &self.machine,
      1 => &self.supervisor,
      _ => unreachable!(),
    }
  }

  pub(crate) fn at_mut(&mut self, index: usize) -> &mut T {
    match index {
      0 => &mut self.machine,
      1 => &mut self.supervisor,
      _ => unreachable!(),
    }
  }
}

#[derive(Debug)]
pub(crate) struct Plic {
  priorities: [u32; 1023],
  pending: [u32; 32],
  enable: [Pair<[u32; 32]>; HART_COUNT * 2],
  threshold: [Pair<u32>; HART_COUNT * 2],
  claimed: [Pair<[bool; 1024]>; HART_COUNT * 2],
  update: bool,
}

impl Plic {
  pub(crate) fn new() -> Plic {
    Plic {
      priorities: [0; 1023],
      pending: [0; 32],
      enable: [Pair { machine: [0; 32], supervisor: [0; 32] }; HART_COUNT * 2],
      threshold: [Pair { machine: 0, supervisor: 0 }; HART_COUNT * 2],
      claimed: [Pair { machine: [false; 1024], supervisor: [false; 1024] }; HART_COUNT * 2],
      update: false,
    }
  }

  pub(crate) fn irq(&mut self, irq: u32, enable: bool) {
    let index = (irq / 32) as usize;
    let offset = irq % 32;
    self.pending[index] |= if enable { 1 } else { 0 } << offset;
    self.update = true;
  }

  fn complete(&mut self, context: usize, irq: u32) {
    self.claimed[context / 2].at_mut(context % 2)[irq as usize] = false;
  }

  fn claim(&mut self, context: usize) -> u32 {
    let irq = self.highest_irq(context);
    let index = (irq / 8) as usize;
    let offset = irq % 8;
    self.pending[index] &= !(1 << offset);
    self.claimed[context / 2].at_mut(context % 2)[irq as usize] = true;
    irq
  }

  fn highest_irq(&mut self, context: usize) -> u32 {
    let mut irq: u32 = 0;
    let mut priority = 0;
    for i in 1..INTERRUPT_COUNT {
      let index = i / 32;
      let offset = i % 32;
      let hart = context / 2;
      let mode = context % 2;
      if self.enable[hart].at(mode)[index] & (1 << offset) != 0
        && self.pending[index] & (1 << offset) != 0
        && !self.claimed[hart].at(mode)[index]
        && self.priorities[i] > *self.threshold[hart].at(mode)
        && self.priorities[i] > priority {
          irq = i as u32;
          priority = self.priorities[i];
      }
    }
    irq
  }
}

impl Device for Plic {
  device_atomic!();

  fn step(&mut self, _bus: &mut Bus, hart: &mut Hart) {
    if self.update {
      self.update = false;
      for context in 0..HART_COUNT {
        let ip = if self.highest_irq(context) != 0 { 1 } else { 0 };
        hart.csr.write_mip_meip(ip);
        hart.csr.write_mip_seip(ip);
      }
    }
  }

  fn read8(&mut self, address: u64) -> Result<u8, Exception> {
    Err(Exception::LoadAccessFault(address))
  }
  fn read16(&mut self, address: u64) -> Result<u16, Exception> {
    Err(Exception::LoadAccessFault(address))
  }
  fn read64(&mut self, address: u64) -> Result<u64, Exception> {
    Ok((self.read32(address + 4)? as u64) | ((self.read32(address)? as u64) << 32))
  }
  fn write8(&mut self, address: u64, _data: u8) -> Result<(), Exception> {
    Err(Exception::StoreAMOAccessFault(address))
  }
  fn write16(&mut self, address: u64, _data: u16) -> Result<(), Exception> {
    Err(Exception::StoreAMOAccessFault(address))
  }
  fn write64(&mut self, address: u64, data: u64) -> Result<(), Exception> {
    self.write32(address, data as u32)?;
    self.write32(address + 4, (data >> 32) as u32)?;
    Ok(())
  }

  fn read32(&mut self, address: u64) -> Result<u32, Exception> {
    if address % 4 != 0 { return Err(Exception::LoadAddressMisaligned(address)); }
    match address {
      PLIC_SOURCE_PRIORITY_START..=PLIC_SOURCE_PRIORITY_END =>
        Ok(self.priorities[(address - PLIC_SOURCE_PRIORITY_START) as usize]),
      PLIC_PENDING_START..=PLIC_PENDING_END =>
        Ok(self.pending[(address - PLIC_PENDING_START) as usize]),
      PLIC_SOURCE_ENABLE_START..=PLIC_SOURCE_ENABLE_END => {
        let offset = (address - PLIC_SOURCE_ENABLE_START) as usize;
        let context = offset / 0x80;
        let item = offset % 0x80;
        Ok(self.enable[context / 2].at(context % 2)[item])
      },
      PLIC_THRESHOLD_CLIAM_COMPLETE_START..=PLIC_THRESHOLD_CLIAM_COMPLETE_END => {
        let offset = (address - PLIC_THRESHOLD_CLIAM_COMPLETE_START) as usize;
        let context = offset / 0x1000;
        let item = offset % 0x1000;
        match item {
          // threshold
          0 => Ok(*self.threshold[context / 2].at(context % 2)),
          // claim
          1 => {
            let irq = self.claim(context);
            self.update = true;
            Ok(irq)
          },
          _ => Err(Exception::LoadAccessFault(address)),
        }
      },
      _ => Err(Exception::LoadAccessFault(address)),
    }
  }

  fn write32(&mut self, address: u64, data: u32) -> Result<(), Exception> {
    if address % 4 != 0 { return Err(Exception::StoreAMOAddressMisaligned(address)); }
    match address {
      PLIC_SOURCE_PRIORITY_START..=PLIC_SOURCE_PRIORITY_END =>
        self.priorities[(address - PLIC_SOURCE_PRIORITY_START) as usize] = data,
      PLIC_SOURCE_ENABLE_START..=PLIC_SOURCE_ENABLE_END => {
        let offset = (address - PLIC_SOURCE_ENABLE_START) as usize;
        let context = offset / 0x80;
        let item = offset % 0x80;
        self.enable[context / 2].at_mut(context % 2)[item] = data;
      },
      PLIC_THRESHOLD_CLIAM_COMPLETE_START..=PLIC_THRESHOLD_CLIAM_COMPLETE_END => {
        let offset = (address - PLIC_THRESHOLD_CLIAM_COMPLETE_START) as usize;
        let context = offset / 0x1000;
        let item = offset % 0x1000;
        match item {
          // threshold
          0 => *self.threshold[context / 2].at_mut(context % 2) = data,
          // complete
          1 => self.complete(context, data),
          _ => return Err(Exception::LoadAccessFault(address)),
        };
      },
      _ => return Err(Exception::StoreAMOAccessFault(address)),
    };
    Ok(())
  }
}
