use crate::{device_atomic, hart::Hart, trap::Exception};

use super::{Device, bus::Bus};

pub(crate) const PLIC_START: u64 = 0x0C000000;
pub(crate) const PLIC_END: u64 = PLIC_START + 0x3FFFFFF;

pub(crate) const PLIC_SOURCE_PRIORITY_START: u64 = PLIC_START + 0x000004;
pub(crate) const PLIC_SOURCE_PRIORITY_END: u64 = PLIC_START + 0x000FFF;

pub(crate) const PLIC_PENDING_START: u64 = PLIC_START + 0x001000;
pub(crate) const PLIC_PENDING_END: u64 = PLIC_START + 0x00107F;

pub(crate) const PLIC_SOURCE_ENABLE_0_START: u64 = PLIC_START + 0x002000;
pub(crate) const PLIC_SOURCE_ENABLE_0_END: u64 = PLIC_SOURCE_ENABLE_0_START + 128 - 1;

pub(crate) const PLIC_SOURCE_ENABLE_1_START: u64 = PLIC_START + 0x002080;
pub(crate) const PLIC_SOURCE_ENABLE_1_END: u64 = PLIC_SOURCE_ENABLE_1_START + 128 - 1;

pub(crate) const PLIC_THRESHOLD0_START: u64 = PLIC_START + 0x200000;
pub(crate) const PLIC_THRESHOLD0_END: u64 = PLIC_THRESHOLD0_START + 4 - 1;

pub(crate) const PLIC_CLIAMCOMPLETE0_START: u64 = PLIC_START + 0x200004;
pub(crate) const PLIC_CLIAMCOMPLETE0_END: u64 = PLIC_CLIAMCOMPLETE0_START + 4 - 1;

pub(crate) const PLIC_THRESHOLD1_START: u64 = PLIC_START + 0x201000;
pub(crate) const PLIC_THRESHOLD1_END: u64 = PLIC_THRESHOLD1_START + 4 - 1;

pub(crate) const PLIC_CLIAMCOMPLETE1_START: u64 = PLIC_START + 0x201004;
pub(crate) const PLIC_CLIAMCOMPLETE1_END: u64 = PLIC_CLIAMCOMPLETE1_START + 4 - 1;

const CONTEXT_LENGTH: usize = 2;

#[derive(Debug)]
pub(crate) struct Plic {
  priorities: [u32; 1023],
  pending: [u32; 32],
  // TODO: if supports multiple harts, make sure there are hart * 2 contexts
  enable: [[u32; 32]; CONTEXT_LENGTH],
  threshold: [u32; CONTEXT_LENGTH],
  claimed: [[bool; 1024]; CONTEXT_LENGTH],
  update: bool,
}

impl Plic {
  pub(crate) fn new() -> Plic {
    Plic {
      priorities: [0; 1023],
      pending: [0; 32],
      enable: [[0; 32]; CONTEXT_LENGTH],
      threshold: [0; CONTEXT_LENGTH],
      claimed: [[false; 1024]; CONTEXT_LENGTH],
      update: false,
    }
  }

  pub(crate) fn irq(&mut self, irq: u32) {
    let index = (irq / 32) as usize;
    let offset = irq % 32;
    self.pending[index] |= 1 << offset;
    self.update = true;
  }

  fn complete(&mut self, context: usize, irq: u32) {
    self.claimed[context][irq as usize] = false;
  }

  fn claim(&mut self, context: usize) -> u32 {
    let irq = self.highest_irq(context);
    let index = (irq / 8) as usize;
    let offset = irq % 8;
    self.pending[index] &= !(1 << offset);
    self.claimed[context][irq as usize] = true;
    irq
  }

  fn highest_irq(&mut self, context: usize) -> u32 {
    let mut irq: u32 = 0;
    let mut priority = 0;
    for i in 1..1024 {
      let index = i / 32;
      let offset = i % 32;
      if self.enable[context][index] & (1 << offset) != 0
        && self.pending[index] & (1 << offset) != 0
        && !self.claimed[context][index]
        && self.priorities[i] > self.threshold[context]
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
    for context in 0..CONTEXT_LENGTH {
      let ip = if self.highest_irq(context) != 0 { 1 } else { 0 };
      if context % 2 == 0 {
        // M-mode
        hart.csr.write_mip_meip(ip);
      } else {
        // S-mode
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
    match address {
      PLIC_SOURCE_PRIORITY_START..=PLIC_SOURCE_PRIORITY_END =>
        Ok(self.priorities[(address - PLIC_SOURCE_PRIORITY_START) as usize]),
      PLIC_PENDING_START..=PLIC_PENDING_END =>
        Ok(self.pending[(address - PLIC_PENDING_START) as usize]),
      PLIC_SOURCE_ENABLE_0_START..=PLIC_SOURCE_ENABLE_0_END =>
        Ok(self.enable[0][(address - PLIC_SOURCE_ENABLE_0_START) as usize]),
      PLIC_SOURCE_ENABLE_1_START..=PLIC_SOURCE_ENABLE_1_END =>
        Ok(self.enable[1][(address - PLIC_SOURCE_ENABLE_1_START) as usize]),
      PLIC_THRESHOLD0_START..=PLIC_THRESHOLD0_END => Ok(self.threshold[0]),
      PLIC_THRESHOLD1_START..=PLIC_THRESHOLD1_END => Ok(self.threshold[1]),
      PLIC_CLIAMCOMPLETE0_START..=PLIC_CLIAMCOMPLETE0_END => {
        let irq = self.claim(0);
        self.update = true;
        Ok(irq)
      },
      PLIC_CLIAMCOMPLETE1_START..=PLIC_CLIAMCOMPLETE1_END => {
        let irq = self.claim(1);
        self.update = true;
        Ok(irq)
      },
      _ => Err(Exception::LoadAccessFault(address)),
    }
  }

  fn write32(&mut self, address: u64, data: u32) -> Result<(), Exception> {
    match address {
      PLIC_SOURCE_PRIORITY_START..=PLIC_SOURCE_PRIORITY_END =>
        self.priorities[(address - PLIC_SOURCE_PRIORITY_START) as usize] = data,
      PLIC_SOURCE_ENABLE_0_START..=PLIC_SOURCE_ENABLE_0_END =>
        self.enable[0][(address - PLIC_SOURCE_ENABLE_0_START) as usize] = data,
      PLIC_SOURCE_ENABLE_1_START..=PLIC_SOURCE_ENABLE_1_END =>
        self.enable[1][(address - PLIC_SOURCE_ENABLE_1_START) as usize] = data,
      PLIC_THRESHOLD0_START..=PLIC_THRESHOLD0_END => {
        self.threshold[0] = data;
        self.update = true;
      },
      PLIC_THRESHOLD1_START..=PLIC_THRESHOLD1_END => {
        self.threshold[1] = data;
        self.update = true;
      },
      PLIC_CLIAMCOMPLETE0_START..=PLIC_CLIAMCOMPLETE0_END => {
        self.complete(0, data);
        self.update = true;
      },
      PLIC_CLIAMCOMPLETE1_START..=PLIC_CLIAMCOMPLETE1_END => {
        self.complete(1, data);
        self.update = true;
      },
      _ => return Err(Exception::StoreAMOAccessFault(address)),
    };
    Ok(())
  }
}
