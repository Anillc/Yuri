use crate::{hart::{Hart, Mode}, trap::Exception};

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

const TRAP_INTO_MACHINE_MASK: u64 = 0b0001100010001000;
const TRAP_INTO_SUPERVISOR_MASK: u64 = 0b0000000100100010;
const MRET_MASK: u64 = 0b100001100010001000;
const SRET_MASK: u64 = 0b100000000100100010;

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
    CsrRegistry { csr: [0; 4096] }
  }

  pub(crate) fn read(hart: &Hart, address: u16) -> Result<u64, Exception> {
    if address >> 8 & 0b11 <= hart.mode.as_u8() {
      Ok(CsrRegistry::read_raw(&hart.csr, address))
    } else {
      Err(Exception::IllegalInstruction)
    }
  }

  pub(crate) fn write(hart: &mut Hart, address: u16, data: u64) -> Result<(), Exception> {
    if address >> 10 & 0b11 == 0b11 {
      // read only
      return Err(Exception::IllegalInstruction);
    }
    if address >> 8 & 0b11 <= hart.mode.as_u8() {
      CsrRegistry::write_raw(&mut hart.csr, address, data);
      Ok(())
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
        MTVEC | STVEC => {
          // ignore mode >= 2
          let mut mode = data & 0b11;
          if mode != 0 || mode != 1 { mode = 0; }
          self.csr[address as usize] = (data & !0b11) | mode;
        },
        _ => self.csr[address as usize] = data,
    };
  }

  pub(crate) fn trap_into_machine(&mut self, old: Mode) {
    let status = self.csr[MSTATUS as usize];
    self.csr[MSTATUS as usize] = (status & !TRAP_INTO_MACHINE_MASK) |
    //   MPIE                          MPP
      (((status >> 3) & 0b1) << 7) | ((old.as_u8() as u64) << 11);
  }

  pub(crate) fn trap_into_supervisor(&mut self, old: Mode) {
    let status = self.csr[MSTATUS as usize];
    debug_assert!(old.as_u8() <= 1);
    self.csr[MSTATUS as usize] = (status & !TRAP_INTO_SUPERVISOR_MASK) |
    //   SPIE                          SPP
      (((status >> 1) & 0b1) << 5) | ((old.as_u8() as u64) << 8);
  }

  pub(crate) fn sret(&mut self) -> (u64, Mode) {
    let status = self.csr[MSTATUS as usize];
    let spie = (status >> 5) & 0b1;
    let spp = Mode::from_u8(((status >> 8) & 0b1) as u8);
    let mprv = if spp == Mode::Machine { (status >> 17) & 0b1 } else { 0 };
    self.csr[MSTATUS as usize] = (status & !SRET_MASK) |
    // SIE           SPIE       MPRV         SPP(set to U which is 0)
      (spie << 1) | (1 << 5) | (mprv << 17);
    (self.csr[MEPC as usize], spp)
  }

  pub(crate) fn mret(&mut self) -> (u64, Mode) {
    let status = self.csr[MSTATUS as usize];
    let mpie = (status >> 7) & 0b1;
    let mpp = Mode::from_u8(((status >> 11) & 0b11) as u8);
    let mprv = if mpp == Mode::Machine { (status >> 17) & 0b1 } else { 0 };
    self.csr[MSTATUS as usize] = (status & !MRET_MASK) |
    // MIE           MPIE       MPRV         MPP(set to U which is 0)
      (mpie << 3) | (1 << 7) | (mprv << 17);
    (self.csr[MEPC as usize], mpp)
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

  pub(crate) fn read_mstatus_tsr(&self) -> bool {
    let status = self.csr[MSTATUS as usize];
    (status >> 22) & 0b1 == 1
  }

  pub(crate) fn read_mstatus_mprv_mpp_sum_mxr(&self) -> (bool, Mode, bool, bool) {
    let status = self.csr[MSTATUS as usize];
    ((status >> 17) & 0b1 == 1,
      Mode::from_u8(((status >> 11) & 0b11) as u8),
      (status >> 18) & 0b1 == 1,
      (status >> 19) & 0b1 == 1)
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

  pub(crate) fn read_medeleg(&self) -> u64 {
    self.csr[MEDELEG as usize]
  }

  pub(crate) fn read_mideleg(&self) -> u64 {
    self.csr[MIDELEG as usize]
  }

  pub(crate) fn write_mepc(&mut self, data: u64) {
    self.csr[MEPC as usize] = data;
  }

  pub(crate) fn write_mcause(&mut self, data: u64) {
    self.csr[MCAUSE as usize] = data;
  }

  pub(crate) fn write_mtval(&mut self, data: u64) {
    self.csr[MTVAL as usize] = data;
  }

  pub(crate) fn read_mtvec(&mut self) -> u64 {
    self.csr[MTVEC as usize]
  }

  pub(crate) fn write_sepc(&mut self, data: u64) {
    self.csr[SEPC as usize] = data;
  }

  pub(crate) fn write_scause(&mut self, data: u64) {
    self.csr[SCAUSE as usize] = data;
  }

  pub(crate) fn write_stval(&mut self, data: u64) {
    self.csr[STVAL as usize] = data;
  }

  pub(crate) fn read_stvec(&mut self) -> u64 {
    self.csr[STVEC as usize]
  }

  pub(crate) fn read_satp(&self) -> u64 {
    self.csr[SATP as usize]
  }
}
