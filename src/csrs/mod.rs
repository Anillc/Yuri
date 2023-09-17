use std::array::from_fn;

use crate::cpu::{Exception, Cpu};

pub(crate) trait Csr {
  fn read(&self, cpu: &Cpu) -> u64;
  fn write(&mut self, cpu: &mut Cpu, data: u64);
}

pub(crate) struct CsrRegistry {
  pub(crate) csr: [Option<Box<dyn Csr>>; 4096],
}

impl CsrRegistry {
  pub(crate) fn new() -> CsrRegistry {
    let registry = CsrRegistry { csr: from_fn(|_| None) };
    registry
  }

  pub(crate) fn read(cpu: &Cpu, address: u16) -> Result<u64, Exception> {
    if address >> 8 & 0b11 <= cpu.mode.into_u16() {
      let csr = cpu.csr.borrow();
      let csr = csr.csr[address as usize].as_ref()
        .ok_or(Exception::IllegalInstruction)?;
      Ok(csr.read(cpu))
    } else {
      Err(Exception::IllegalInstruction)
    }
  }

  pub(crate) fn write(cpu: &mut Cpu, address: u16, data: u64) -> Result<(), Exception> {
    if address >> 10 & 0b11 == 0b11 {
      // read only
      return Err(Exception::IllegalInstruction);
    }
    if address >> 8 & 0b11 <= cpu.mode.into_u16() {
      let csr = cpu.csr.clone();
      let mut csr = csr.borrow_mut();
      let csr = csr.csr[address as usize].as_mut()
        .ok_or(Exception::IllegalInstruction)?;
      csr.write(cpu, data);
      Ok(())
    } else {
      Err(Exception::IllegalInstruction)
    }
  }
}
