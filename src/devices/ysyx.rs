use std::time::{SystemTime, UNIX_EPOCH};

use crate::{device_atomic, hart::Hart, trap::Exception};

use super::{Device, bus::Bus};

pub(crate) const YSYX_START: u64 = 0x20000000;
pub(crate) const YSYX_END: u64 = YSYX_START + 0x01000000 - 1;

pub(crate) const YSYX_TIME: u64 = YSYX_START;

#[derive(Debug)]
pub(crate) struct Ysyx {}

impl Ysyx {
  pub(crate) fn new() -> Ysyx {
    Ysyx {}
  }
}

impl Device for Ysyx {
  device_atomic!();

  fn step(&mut self, _bus: &mut Bus, _hart: &mut Hart) {}

  fn read8(&mut self, address: u64) -> Result<u8, Exception> {
    Err(Exception::LoadAccessFault(address))
  }

  fn read16(&mut self, address: u64) -> Result<u16, Exception> {
    Err(Exception::LoadAccessFault(address))
  }

  fn read32(&mut self, address: u64) -> Result<u32, Exception> {
    Err(Exception::LoadAccessFault(address))
  }

  fn read64(&mut self, address: u64) -> Result<u64, Exception> {
    match address {
      YSYX_TIME => Ok(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()),
      _ => Err(Exception::LoadAccessFault(address))
    }
  }

  fn write8(&mut self, address: u64, _data: u8) -> Result<(), Exception> {
    Err(Exception::StoreAMOAccessFault(address))
  }

  fn write16(&mut self, address: u64, _data: u16) -> Result<(), Exception> {
    Err(Exception::StoreAMOAccessFault(address))
  }

  fn write32(&mut self, address: u64, _data: u32) -> Result<(), Exception> {
    Err(Exception::StoreAMOAccessFault(address))
  }

  fn write64(&mut self, address: u64, _data: u64) -> Result<(), Exception> {
    Err(Exception::StoreAMOAccessFault(address))
  }
}
