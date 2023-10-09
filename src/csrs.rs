use crate::cpu::{Exception, Cpu};

pub(crate) struct CsrRegistry {
  pub(crate) csr: [u64; 4096],
}

impl CsrRegistry {
  pub(crate) fn new() -> CsrRegistry {
    let registry = CsrRegistry { csr: [0; 4096] };
    registry
  }

  pub(crate) fn read(cpu: &Cpu, address: u16) -> Result<u64, Exception> {
    if address >> 8 & 0b11 <= cpu.mode.into_u16() {
      CsrRegistry::read_raw(cpu, address)
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
      CsrRegistry::write_raw(cpu, address, data)
    } else {
      Err(Exception::IllegalInstruction)
    }
  }

  fn read_raw(cpu: &Cpu, address: u16) -> Result<u64, Exception> {
    todo!()
  }

  fn write_raw(cpu: &mut Cpu, address: u16, data: u64) -> Result<(), Exception> {
    todo!()
  }
}
