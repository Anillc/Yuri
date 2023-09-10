use crate::cpu::{Exception, Mode};

pub(crate) struct Csr {
  pub(crate) csr: [u64; 4096]
}

impl Csr {
  pub(crate) fn new() -> Csr {
    Csr { csr: [0; 4096] }
  }

  pub(crate) fn read(&self, mode: Mode, address: u16) -> Result<u64, Exception> {
    if address >> 8 & 0b11 <= mode.into_u16() {
      Ok(self.csr[address as usize])
    } else {
      Err(Exception::IllegalInstruction)
    }
  }

  pub(crate) fn write(&mut self, mode: Mode, address: u16, data: u64) -> Result<(), Exception> {
    if address >> 10 & 0b11 == 0b11 {
      // read only
      return Err(Exception::IllegalInstruction);
    }
    if address >> 8 & 0b11 <= mode.into_u16() {
      self.csr[address as usize] = data;
      Ok(())
    } else {
      Err(Exception::IllegalInstruction)
    }
  }
}
