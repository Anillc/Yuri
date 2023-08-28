use crate::utils::extend_sign;

use super::InstructionSegment;

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

pub(crate) trait InstructionParser {
  fn r(&self) -> R;
  fn i(&self) -> I;
  fn s(&self) -> S;
  fn b(&self) -> B;
  fn u(&self) -> U;
  fn j(&self) -> J;
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
      imm: extend_sign(((self >> 20) & 0b111111111111) as u64, 12),
      rs1: ((self >> 15) & 0b11111) as usize,
      rd: ((self >> 7) & 0b11111) as usize,
    }
  }

  fn s(&self) -> S {
    S {
      imm: extend_sign((((self >> 25) & 0b11111) << 5) as u64 | ((self >> 7) & 0b11111) as u64, 12),
      rs2: ((self >> 20) & 0b11111) as usize,
      rs1: ((self >> 15) & 0b11111) as usize,
    }
  }

  fn b(&self) -> B {
    B {
      imm: extend_sign((((self >> 31) & 0b1) << 12) as u64 | (((self >> 7) & 0b1) << 11) as u64 | (((self >> 25) & 0b111111) << 5) as u64 | (((self >> 8) & 0b1111) << 1) as u64, 13),
      rs2: ((self >> 20) & 0b11111) as usize,
      rs1: ((self >> 15) & 0b11111) as usize,
    }
  }

  fn u(&self) -> U {
    U {
      imm: extend_sign(((self >> 12) & 0b11111111111111111111) as u64, 20),
      rd: ((self >> 7) & 0b11111) as usize,
    }
  }

  fn j(&self) -> J {
    J {
      imm: extend_sign(((((self >> 31) & 0b1) << 20) | (((self >> 12) & 0b11111111) << 12) | (((self >> 20) & 0b1) << 11) | (((self >> 21) & 0b1111111111) << 1)) as u64, 21),
      rd: ((self >> 7) & 0b11111) as usize,
    }
  }
}

pub(crate) fn funct3(funct3: u8) -> Vec<InstructionSegment> {
  vec![InstructionSegment { start: 12, end: 14, comp: funct3 as u32 }]
}

pub(crate) fn funct37(funct3: u8, funct7: u8) -> Vec<InstructionSegment> {
  vec![
    InstructionSegment { start: 12, end: 14, comp: funct3 as u32 },
    InstructionSegment { start: 25, end: 31, comp: funct7 as u32 },
  ]
}
