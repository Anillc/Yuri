use crate::{cpu::Cpu, trap::Exception};

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

const MEPC: u16 = 0x341;
const MCAUSE: u16 = 0x342;
const MTVAL: u16 = 0x343;
const MIP: u16 = 0x344;

const SSTATUS: u16 = 0x100;
const SIE: u16 = 0x104;
const STVEC: u16 = 0x105;

const SEPC: u16 = 0x141;
const SCAUSE: u16 = 0x142;
const STVAL: u16 = 0x143;
const SIP: u16 = 0x144;

const SATP: u16 = 0x180;

// TODO: cycle

const MSTATUS_MASK: u64 = 0b1000000000000000000000000011111100000000011111111111111111101010;
const SSTATUS_MASK: u64 = 0b1000000000000000000000000000001100000000000011011110011101100010;
const MIE_MASK: u64 = 0b0000101010101010;
const MIP_MASK: u64 = 0b0000101010101010;
const SIE_MASK: u64 = 0b0000001000100010;
const SIP_MASK: u64 = 0b0000001000100010;

pub(crate) struct CsrRegistry {
  pub(crate) csr: [u64; 4096],
}

#[derive(Debug)]
pub(crate) struct MIEP {
  pub(crate) ss: bool,
  pub(crate) ms: bool,
  pub(crate) st: bool,
  pub(crate) mt: bool,
  pub(crate) se: bool,
  pub(crate) me: bool,
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
        MSTATUS => self.csr[MSTATUS as usize] = data & MSTATUS_MASK,
        MIE => self.csr[MIE as usize] = data & MIE_MASK,
        MIP => self.csr[MIP as usize] = data & MIP_MASK,
        SIE => self.csr[MIE as usize] =
          (self.csr[MIE as usize] & !SIE_MASK) | (data & SIE_MASK),
        SIP => self.csr[MIP as usize] =
          (self.csr[MIP as usize] & !SIP_MASK) | (data & SIP_MASK),
        SSTATUS => self.csr[MSTATUS as usize] =
          (self.csr[MSTATUS as usize] & !SSTATUS_MASK) | (data & SSTATUS_MASK),
        _ => self.csr[address as usize] = data,
    };
  }

  pub(crate) fn read_frm(&self) -> u8 {
    self.read_raw(FRM) as u8
  }

  pub(crate) fn write_fflags(&mut self, data: u64) {
    self.write_raw(FFLAGS, data);
  }

  pub(crate) fn read_mstatus_sie(&self) -> bool {
    let status = self.csr[MSTATUS as usize];
    (status >> 1) & 0b1 == 1
  }

  pub(crate) fn read_mstatus_mie(&self) -> bool {
    let status = self.csr[MSTATUS as usize];
    (status >> 3) & 0b1 == 1
  }

  pub(crate) fn read_mie(&self) -> MIEP {
    let mie = self.csr[MIE as usize];
    MIEP {
      ss: (mie >> 1)  & 0b1 == 1,
      ms: (mie >> 3)  & 0b1 == 1,
      st: (mie >> 5)  & 0b1 == 1,
      mt: (mie >> 7)  & 0b1 == 1,
      se: (mie >> 9)  & 0b1 == 1,
      me: (mie >> 11) & 0b1 == 1,
    }
  }

  pub(crate) fn read_mip(&self) -> MIEP {
    let mie = self.csr[MIP as usize];
    MIEP {
      ss: (mie >> 1)  & 0b1 == 1,
      ms: (mie >> 3)  & 0b1 == 1,
      st: (mie >> 5)  & 0b1 == 1,
      mt: (mie >> 7)  & 0b1 == 1,
      se: (mie >> 9)  & 0b1 == 1,
      me: (mie >> 11) & 0b1 == 1,
    }
  }
}
