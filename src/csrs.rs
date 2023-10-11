use crate::cpu::{Exception, Cpu};

const FFLAGS: u16 = 0x001;
const FRM: u16 = 0x002;
const FCSR: u16 = 0x003;

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
      Ok(CsrRegistry::read_raw(&cpu.csr, address))
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
      Ok(CsrRegistry::write_raw(&mut cpu.csr, address, data))
    } else {
      Err(Exception::IllegalInstruction)
    }
  }

  fn read_raw(&self, address: u16) -> u64 {
    match address {
        FFLAGS => self.csr[FCSR as usize] & 0b11111,
        FRM => (self.csr[FCSR as usize] >> 5) & 0b111,
        _ => self.csr[address as usize],
    }
  }

  fn write_raw(&mut self, address: u16, data: u64) {
    match address {
        FFLAGS => {
          let rest = self.csr[FCSR as usize] & !0b11111;
          self.csr[FCSR as usize] = rest | (data & 0b11111);
        },
        FRM => {
          let rest = self.csr[FCSR as usize] & !0b11100000;
          self.csr[FCSR as usize] = rest | ((data & 0b111) << 5)
        },
        FCSR => self.csr[FCSR as usize] = data & 0b11111111,
        _ => self.csr[address as usize] = data,
    };
  }

  pub(crate) fn read_frm(&self) -> u8 {
    self.read_raw(FRM) as u8
  }

  pub(crate) fn write_fflags(&mut self, data: u64) {
    self.write_raw(FFLAGS, data);
  }
}
