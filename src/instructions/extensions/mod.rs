use crate::utils::extend_sign;
use super::InstructionSegment;

pub(crate) mod i;
pub(crate) mod zifenci;
pub(crate) mod zicsr;
pub(crate) mod m;
pub(crate) mod a;
pub(crate) mod f;
pub(crate) mod d;
pub(crate) mod c;

pub(crate) struct R {
  pub(crate) rs2: usize,
  pub(crate) rs1: usize,
  pub(crate) rd: usize,
}

pub(crate) struct I {
  pub(crate) imm: i64,
  pub(crate) rs1: usize,
  pub(crate) rd: usize,
}

pub(crate) struct S {
  pub(crate) imm: i64,
  pub(crate) rs2: usize,
  pub(crate) rs1: usize,
}

pub(crate) struct B {
  pub(crate) imm: i64,
  pub(crate) rs2: usize,
  pub(crate) rs1: usize,
}

pub(crate) struct U {
  pub(crate) imm: i64,
  pub(crate) rd: usize,
}

pub(crate) struct J {
  pub(crate) imm: i64,
  pub(crate) rd: usize,
}

// R-type for A extension
pub(crate) struct RA {
  pub(crate) aq: bool,
  pub(crate) rl: bool,
  pub(crate) rs2: usize,
  pub(crate) rs1: usize,
  pub(crate) rd: usize,
}

// Float Point
pub(crate) struct RFP {
  pub(crate) rs2: usize,
  pub(crate) rs1: usize,
  pub(crate) rm: u8,
  pub(crate) rd: usize,
}

pub(crate) struct RFPRS3 {
  pub(crate) rs3: usize,
  pub(crate) rs2: usize,
  pub(crate) rs1: usize,
  pub(crate) rm: u8,
  pub(crate) rd: usize,
}

pub(crate) trait InstructionParser {
  fn r(&self) -> R;
  fn i(&self) -> I;
  fn s(&self) -> S;
  fn b(&self) -> B;
  fn u(&self) -> U;
  fn j(&self) -> J;
  fn ra(&self) -> RA;
  fn rfp(&self) -> RFP;
  fn rfp_rs3(&self) -> RFPRS3;
}

impl InstructionParser for u32 {
  fn r(&self) -> R {
    R {
      rs2: ((self >> 20) & 0b11111) as usize,
      rs1: ((self >> 15) & 0b11111) as usize,
      rd: ((self >> 7) & 0b11111) as usize,
    }
  }

  fn i(&self) -> I {
    I {
      imm: extend_sign((self >> 20) as u64, 12),
      rs1: ((self >> 15) & 0b11111) as usize,
      rd: ((self >> 7) & 0b11111) as usize,
    }
  }

  fn s(&self) -> S {
    S {
      imm: extend_sign(((self >> 25) << 5) as u64 | ((self >> 7) & 0b11111) as u64, 12),
      rs2: ((self >> 20) & 0b11111) as usize,
      rs1: ((self >> 15) & 0b11111) as usize,
    }
  }

  fn b(&self) -> B {
    B {
      imm: extend_sign(((self >> 31) << 12) as u64 | (((self >> 7) & 0b1) << 11) as u64 | (((self >> 25) & 0b111111) << 5) as u64 | (((self >> 8) & 0b1111) << 1) as u64, 13),
      rs2: ((self >> 20) & 0b11111) as usize,
      rs1: ((self >> 15) & 0b11111) as usize,
    }
  }

  fn u(&self) -> U {
    U {
      imm: extend_sign((self & 0b11111111111111111111000000000000) as u64, 32),
      rd: ((self >> 7) & 0b11111) as usize,
    }
  }

  fn j(&self) -> J {
    J {
      imm: extend_sign((((self >> 31) << 20) | (((self >> 12) & 0b11111111) << 12) | (((self >> 20) & 0b1) << 11) | (((self >> 21) & 0b1111111111) << 1)) as u64, 21),
      rd: ((self >> 7) & 0b11111) as usize,
    }
  }

  fn ra(&self) -> RA {
    RA {
      aq: (self >> 26) & 0b1 == 1,
      rl: (self >> 25) & 0b1 == 1,
      rs2: ((self >> 20) & 0b11111) as usize,
      rs1: ((self >> 15) & 0b11111) as usize,
      rd: ((self >> 7) & 0b11111) as usize,
    }
  }

  fn rfp(&self) -> RFP {
    RFP {
      rs2: ((self >> 20) & 0b11111) as usize,
      rs1: ((self >> 15) & 0b11111) as usize,
      rm: ((self >> 12) & 0b111) as u8,
      rd: ((self >> 7) & 0b11111) as usize,
    }
  }

  fn rfp_rs3(&self) -> RFPRS3 {
    RFPRS3 {
      rs3: ((self >> 27) & 0b11111) as usize,
      rs2: ((self >> 20) & 0b11111) as usize,
      rs1: ((self >> 15) & 0b11111) as usize,
      rm: ((self >> 12) & 0b111) as u8,
      rd: ((self >> 7) & 0b11111) as usize,
    }
  }
}

pub(crate) fn funct3(funct3: u8) -> Vec<InstructionSegment> {
  vec![
    InstructionSegment { start: 12, end: 14, comp: funct3 as u32 }
  ]
}

pub(crate) fn funct37(funct3: u8, funct7: u8) -> Vec<InstructionSegment> {
  vec![
    InstructionSegment { start: 12, end: 14, comp: funct3 as u32 },
    InstructionSegment { start: 25, end: 31, comp: funct7 as u32 },
  ]
}

pub(crate) fn funct_ra(funct3: u8, funct5: u8) -> Vec<InstructionSegment> {
  vec![
    InstructionSegment { start: 12, end: 14, comp: funct3 as u32 },
    InstructionSegment { start: 27, end: 31, comp: funct5 as u32 },
  ]
}

pub(crate) fn funct_ra_rs2(funct3: u8, funct5: u8) -> Vec<InstructionSegment> {
  vec![
    InstructionSegment { start: 12, end: 14, comp: funct3 as u32 },
    InstructionSegment { start: 20, end: 24, comp: 0b000 },
    InstructionSegment { start: 27, end: 31, comp: funct5 as u32 },
  ]
}

pub(crate) fn funct_rfp(fmt: u8, funct5: u8) -> Vec<InstructionSegment> {
  vec![
    InstructionSegment { start: 25, end: 26, comp: fmt as u32 },
    InstructionSegment { start: 27, end: 31, comp: funct5 as u32},
  ]
}

pub(crate) fn funct_rfp_rs2(rs2: u8, fmt: u8, funct5: u8) -> Vec<InstructionSegment> {
  vec![
    InstructionSegment { start: 20, end: 24, comp: rs2 as u32 },
    InstructionSegment { start: 25, end: 26, comp: fmt as u32 },
    InstructionSegment { start: 27, end: 31, comp: funct5 as u32},
  ]
}

pub(crate) fn funct_rfp_rs3(fmt: u8) -> Vec<InstructionSegment> {
  vec![
    InstructionSegment { start: 25, end: 26, comp: fmt as u32 },
  ]
}

pub(crate) fn funct_rfp_rm(rm: u8, fmt: u8, funct5: u8) -> Vec<InstructionSegment> {
  vec![
    InstructionSegment { start: 12, end: 14, comp: rm as u32 },
    InstructionSegment { start: 25, end: 26, comp: fmt as u32 },
    InstructionSegment { start: 27, end: 31, comp: funct5 as u32},
  ]
}

pub(crate) fn funct_rfp_rs2_rm(rm: u8, rs2: u8, fmt: u8, funct5: u8) -> Vec<InstructionSegment> {
  vec![
    InstructionSegment { start: 12, end: 14, comp: rm as u32 },
    InstructionSegment { start: 20, end: 24, comp: rs2 as u32 },
    InstructionSegment { start: 25, end: 26, comp: fmt as u32 },
    InstructionSegment { start: 27, end: 31, comp: funct5 as u32},
  ]
}
