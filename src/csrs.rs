use crate::cpu::{Exception, Cpu};

const FFLAGS: u16 = 0x001;
const FRM: u16 = 0x002;
const FCSR: u16 = 0x003;

const MVENDORID: u16 = 0xF11;
const MARCHID: u16 = 0xF12;
const MIMPID: u16 = 0xF13;
const MHARTID: u16 = 0xF14;

const MSTATUS: u16 = 0x300;
const MISA: u16 = 0x301;
const MEDELEG: u16 = 0x302;
const MIDELEG: u16 = 0x303;
const MIE: u16 = 0x304;
const MTVEC: u16 = 0x305;

const MSCRATCH: u16 = 0x340;
const MEPC: u16 = 0x341;
const MCAUSE: u16 = 0x342;
const MTVAL: u16 = 0x343;
const MIP: u16 = 0x344;

// TODO: cycle

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
        MISA => {
          let mxl = 2 << 62;
          let a = 1;
          let c = 1 << 2;
          let d = 1 << 3;
          let f = 1 << 5;
          let i = 1 << 8;
          let m = 1 << 12;
          let s = 1 << 18;
          let u = 1 << 20;
          mxl | i | m | a | f | d | c | s | u
        },
        MVENDORID => 0,
        MARCHID => 0,
        MIMPID => 0,
        MHARTID => 0,
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
        MISA => {},
        MVENDORID => {},
        MARCHID => {},
        MIMPID => {},
        MHARTID => {},
        MSTATUS => {
          let mask = 0b1000000000000000000000000011111100000000011111111111111111101010;
          self.csr[MSTATUS as usize] = data & mask;
        },
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
