use crate::{device_atomic, device_rw};

use super::Device;

pub(crate) const DEVICE_TREE_START: u64 = 0x00001000;
pub(crate) const DEVICE_TREE_END: u64 = DEVICE_TREE_START + 0xf000 - 1;

#[derive(Debug)]
pub(crate) struct DeviceTree {

}

impl DeviceTree {
  pub(crate) fn new() -> DeviceTree {
    DeviceTree {

    }
  }
}

impl Device for DeviceTree {
  device_atomic!();
  device_rw!();

  fn step(&mut self, bus: &mut super::bus::Bus, hart: &mut crate::hart::Hart) {
    todo!()
  }

  fn read8(&mut self, address: u64) -> Result<u8, crate::trap::Exception> {
    todo!()
  }

  fn write8(&mut self, address: u64, data: u8) -> Result<(), crate::trap::Exception> {
    todo!()
  }
}