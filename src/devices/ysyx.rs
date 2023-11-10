use std::time::{SystemTime, UNIX_EPOCH};

use crate::{device_atomic, hart::Hart, trap::Exception};

use super::{Device, bus::Bus};

pub(crate) const YSYX_START: u64 = 0x20000000;
pub(crate) const YSYX_END: u64 = YSYX_START + 0x10000000 - 1;

const VGA_WIDTH: usize = 800;
const VGA_HEIGHT: usize = 600;


const YSYX_TIME: u64 = YSYX_START;

const YSYX_VGACTL_ADDR_LOW: u64 = YSYX_START + 0x100;
// sync
const YSYX_VGACTL_ADDR_HIGH: u64 = YSYX_START + 0x100 + 4;

const YSYX_FB_START: u64 = YSYX_START + 0x01000000;
const YSYX_FB_END: u64 = YSYX_FB_START + ((VGA_WIDTH * VGA_HEIGHT * 4) as u64) - 1;

#[derive(Debug)]
pub(crate) struct Ysyx {
  vgactl: [u32; 2],
  vmem: [u32; VGA_WIDTH * VGA_HEIGHT],
}

impl Ysyx {
  pub(crate) fn new() -> Ysyx {
    Ysyx {
      vgactl: [((VGA_WIDTH << 16) | VGA_HEIGHT) as u32, 0],
      vmem: [0; VGA_WIDTH * VGA_HEIGHT],
    }
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
    match address {
      YSYX_VGACTL_ADDR_LOW => Ok(self.vgactl[0]),
      YSYX_VGACTL_ADDR_HIGH => Ok(self.vgactl[1]),
      YSYX_FB_START..=YSYX_FB_END => Ok(self.vmem[((address - YSYX_FB_START) / 4) as usize]),
      _ => Err(Exception::LoadAccessFault(address)),
    }
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

  fn write32(&mut self, address: u64, data: u32) -> Result<(), Exception> {
    match address {
      YSYX_VGACTL_ADDR_HIGH => {
        // TODO
        dbg!("sync");
      },
      YSYX_FB_START..=YSYX_FB_END => self.vmem[((address - YSYX_FB_START) / 4) as usize] = data,
      _ => return Err(Exception::StoreAMOAccessFault(address)),
    }
    Ok(())
  }

  fn write64(&mut self, address: u64, _data: u64) -> Result<(), Exception> {
    Err(Exception::StoreAMOAccessFault(address))
  }
}
