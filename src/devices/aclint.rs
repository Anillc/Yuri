use crate::{device_atomic, device_rw, hart::Hart, trap::Exception};

use super::Device;

pub(crate) const ACLINT_START: u64 = 0x02000000;
pub(crate) const ACLINT_END: u64 = ACLINT_START + 0x0000bfff;

const MTIME_START: u64 = ACLINT_START + 0x0000bff8;
const MTIME_END: u64 = MTIME_START + 8 - 1;

const MTIMECMP0_START: u64 = ACLINT_START + 0x00004000;
const MTIMECMP0_END: u64 = MTIMECMP0_START + 8 - 1;

const MSIP0_START: u64 = ACLINT_START;
const MSIP0_END: u64 = MSIP0_START + 4 - 1;

#[derive(Debug)]
pub(crate) struct Aclint {
  mtime: u64,
  mtimecmp0: u64,
  msip0: u32,
  msip0_wrote: bool,
  // setssip0: u32,
}

impl Aclint {
  pub(crate) fn new() -> Aclint {
    Aclint {
      mtime: 0,
      mtimecmp0: 0,
      msip0: 0,
      msip0_wrote: false,
      // setssip0: 0,
    }
  }
}

impl Device for Aclint {
  device_atomic!();
  device_rw!();

  fn step(&mut self, hart: &mut Hart) {
    self.mtime = self.mtime.wrapping_add(1);
    hart.csr.write_mip_mtip(if self.mtime >= self.mtimecmp0 { 0 } else { 1 });

    if self.msip0_wrote {
      self.msip0_wrote = false;
      hart.csr.write_mip_msip(self.msip0 as u64 & 0b1);
    }

    // if self.setssip0 != 0 {
    //   self.setssip0 = 0;
    //   hart.csr.write_mip_ssip(1);
    // }
  }

  fn read8(&self, address: u64) -> Result<u8, Exception> {
    match address {
      MSIP0_START..=MSIP0_END => Ok(self.msip0.to_le_bytes()[(address - MSIP0_START) as usize]),
      MTIMECMP0_START..=MTIMECMP0_END => Ok(self.mtimecmp0.to_le_bytes()[(address - MTIMECMP0_START) as usize]),
      MTIME_START..=MTIME_END => Ok(self.mtime.to_le_bytes()[(address - MTIME_START) as usize]),
      _ => Err(Exception::LoadAccessFault(address))
    }
  }

  fn write8(&mut self, address: u64, data: u8) -> Result<(), Exception> {
    match address {
      MSIP0_START..=MSIP0_END => {
        let mut msip0 = self.msip0.to_le_bytes();
        msip0[(address - MSIP0_START) as usize] = data;
        self.msip0 = u32::from_le_bytes(msip0);
      },
      MTIMECMP0_START..=MTIMECMP0_END => {
        let mut mtimecmp0 = self.mtimecmp0.to_le_bytes();
        mtimecmp0[(address - MTIMECMP0_START) as usize] = data;
        self.mtimecmp0 = u64::from_le_bytes(mtimecmp0);
      },
      MTIME_START..=MTIME_END => {
        let mut mtime = self.mtime.to_le_bytes();
        mtime[(address - MTIME_START) as usize] = data;
        self.mtime = u64::from_le_bytes(mtime);
      }
      _ => return Err(Exception::LoadAccessFault(address))
      
    };
    Ok(())
  }
}
