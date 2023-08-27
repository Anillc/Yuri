use std::collections::HashMap;
use once_cell::sync::Lazy;

use crate::cpu::Cpu;
use crate::utils::extend_sign;

use self::r::r_instructions;
use self::i::i_instructions;
use self::s::s_instructions;
use self::b::b_instructions;
use self::u::u_instructions;
use self::j::j_instructions;

mod r;
mod i;
mod s;
mod b;
mod u;
mod j;

//                                opcode, (mask, (key, instructor))
static INSTRUCTIONS: Lazy<HashMap<u8, (u32, HashMap<u32, Instructor>)>> = Lazy::new(|| {
  let sets = [
    (r_instructions(), 0b11111110000000000111000001111111),
    (i_instructions(), 0b00000000000000000111000001111111),
    (s_instructions(), 0b00000000000000000111000001111111),
    (b_instructions(), 0b00000000000000000111000001111111),
    (u_instructions(), 0b00000000000000000000000001111111),
    (j_instructions(), 0b00000000000000000000000001111111),
  ];
  let mut map = HashMap::new();
  for (set, mask) in sets {
    let set: HashMap<u8, Vec<Instructor>> = set.into_iter().fold(HashMap::new(), |mut acc, x| {
      acc.entry(x.opcode).or_default().push(x);
      acc
    });
    for (opcode, instructors) in set {
      let mut key_map = HashMap::new();
      for instructor in instructors {
        let mut key = instructor.opcode as u32;
        match instructor.funct {
          Funct::R(funct3, funct7) => key |= ((funct3 as u32) << 25) | ((funct7 as u32) << 12),
          Funct::I(funct3) => key |= (funct3 as u32) << 12,
          Funct::S(funct3) => key |= (funct3 as u32) << 12,
          Funct::B(funct3) => key |= (funct3 as u32) << 12,
          // U & J
          _ => {},
        };
        key_map.insert(key, instructor);
      }
      map.insert(opcode, (mask, key_map));
    }
  }
  map
});

#[derive(Debug, Clone, Copy)]
pub(crate) enum Funct {
  R(u8, u8), I(u8), S(u8), B(u8), U, J,
}

#[derive(Debug)]
pub(crate) struct Instructor {
  #[allow(dead_code)]
  pub(crate) name: &'static str,
  pub(crate) opcode: u8,
  pub(crate) funct: Funct,
  pub(crate) run: fn(inst: Instruction, cpu: &mut Cpu),
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Instruction {
  R {
    funct7: u8,
    rs2: usize,
    rs1: usize,
    funct3: u8,
    rd: usize,
    opcode: u8,
  },
  I {
    imm: i64,
    rs1: usize,
    funct3: u8,
    rd: usize,
    opcode: u8,
  },
  S {
    imm: i64,
    rs2: usize,
    rs1: usize,
    funct3: u8,
    opcode: u8,
  },
  B {
    imm: i64,
    rs2: usize,
    rs1: usize,
    funct3: u8,
    opcode: u8,
  },
  U {
    imm: i64,
    rd: usize,
    opcode: u8,
  },
  J {
    imm: i64,
    rd: usize,
    opcode: u8,
  },
}

pub(crate) fn parse(inst: u32) -> (&'static Instructor, Instruction) {
  // TODO: handle None
  let (mask, instructors) = INSTRUCTIONS.get(&((inst & 0b1111111) as u8)).unwrap();
  let instructor = instructors.get(&(inst & mask)).unwrap();
  let instruction = match instructor.funct {
    Funct::R(funct3, funct7) => {
      Instruction::R {
        funct7,
        rs2: ((inst >> 20) & 0b11111) as usize,
        rs1: ((inst >> 15) & 0b11111) as usize,
        funct3,
        rd: ((inst >> 7) & 0b11111) as usize,
        opcode: (inst & 0b1111111) as u8,
      }
    },
    Funct::I(funct3) => {
      Instruction::I {
        imm: extend_sign(((inst >> 20) & 0b111111111111) as u64, 12),
        rs1: ((inst >> 15) & 0b11111) as usize,
        funct3,
        rd: ((inst >> 7) & 0b11111) as usize,
        opcode: (inst & 0b1111111) as u8,
      }
    },
    Funct::S(funct3) => {
      Instruction::S {
        imm: extend_sign((((inst >> 25) & 0b11111) << 5) as u64 | ((inst >> 7) & 0b11111) as u64, 12),
        rs2: ((inst >> 20) & 0b11111) as usize,
        rs1: ((inst >> 15) & 0b11111) as usize,
        funct3,
        opcode: (inst & 0b1111111) as u8,
      }
    },
    Funct::B(funct3) => {
      Instruction::B {
        imm: extend_sign((((inst >> 31) & 0b1) << 12) as u64 | (((inst >> 7) & 0b1) << 11) as u64 | (((inst >> 25) & 0b111111) << 5) as u64 | (((inst >> 8) & 0b1111) << 1) as u64, 13),
        rs2: ((inst >> 20) & 0b11111) as usize,
        rs1: ((inst >> 15) & 0b11111) as usize,
        funct3,
        opcode: (inst & 0b1111111) as u8,
      }
    },
    Funct::U => {
      Instruction::U {
        imm: extend_sign(((inst >> 12) & 0b11111111111111111111) as u64, 20),
        rd: ((inst >> 7) & 0b11111) as usize,
        opcode: (inst & 0b1111111) as u8,
      }
    },
    Funct::J => {
      Instruction::J {
        imm: extend_sign(((((inst >> 31) & 0b1) << 20) | (((inst >> 12) & 0b11111111) << 12) | (((inst >> 20) & 0b1) << 11) | (((inst >> 21) & 0b1111111111) << 1)) as u64, 21),
        rd: ((inst >> 7) & 0b11111) as usize,
        opcode: (inst & 0b1111111) as u8,
      }
    },
  };
  (instructor, instruction)
}
