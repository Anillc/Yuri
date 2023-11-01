use crate::{device_atomic, device_rw, hart::Hart, trap::Exception};

use super::{Device, bus::Bus};

pub(crate) const PLIC_START: u64 = 0x0c000000;
pub(crate) const PLIC_END: u64 = PLIC_START + 0x00000fff;

#[derive(Debug)]
pub(crate) struct Plic {}

impl Plic {
  pub(crate) fn new() -> Plic {
    Plic { }
  }
}

impl Device for Plic {
  device_atomic!();
  device_rw!();

  fn step(&mut self, bus: &mut Bus, hart: &mut Hart) {
    todo!()
  }

  fn read8(&self, address: u64) -> Result<u8, Exception> {
    todo!()
  }

  fn write8(&mut self, address: u64, data: u8) -> Result<(), Exception> {
    todo!()
  }
}
