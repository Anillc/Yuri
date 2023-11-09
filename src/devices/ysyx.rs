use crate::{device_atomic, device_rw, hart::Hart, trap::Exception};

use super::{Device, bus::Bus};

pub(crate) const YSYX_START: u64 = 0x20000000;
pub(crate) const YSYX_END: u64 = YSYX_START + 0x01000000 - 1;

#[derive(Debug)]
pub(crate) struct Ysyx {}

impl Ysyx {
  pub(crate) fn new() -> Ysyx {
    Ysyx {}
  }
}

impl Device for Ysyx {
  device_atomic!();
  device_rw!();

  fn step(&mut self, bus: &mut Bus, hart: &mut Hart) {
    todo!()
  }

  fn read8(&mut self, address: u64) -> Result<u8, Exception> {
    todo!()
  }

  fn write8(&mut self, address: u64, data: u8) -> Result<(), Exception> {
    todo!()
  }
}
