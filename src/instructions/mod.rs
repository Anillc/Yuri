use std::collections::HashMap;
use once_cell::sync::Lazy;

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

//                                  opcode+funct
static R_INSTRUCTIONS: Lazy<HashMap<u32, Instructor>> = Lazy::new(|| r_instructions());
static I_INSTRUCTIONS: Lazy<HashMap<u32, Instructor>> = Lazy::new(|| i_instructions());
static S_INSTRUCTIONS: Lazy<HashMap<u32, Instructor>> = Lazy::new(|| s_instructions());
static B_INSTRUCTIONS: Lazy<HashMap<u32, Instructor>> = Lazy::new(|| b_instructions());
static U_INSTRUCTIONS: Lazy<HashMap<u32, Instructor>> = Lazy::new(|| u_instructions());
static J_INSTRUCTIONS: Lazy<HashMap<u32, Instructor>> = Lazy::new(|| j_instructions());

//                                opcode
static INSTRUCTIONS: Lazy<HashMap<u8, InstructionType>> = Lazy::new(|| {
  let sets = [
    &R_INSTRUCTIONS,
    &I_INSTRUCTIONS,
    &S_INSTRUCTIONS,
    &B_INSTRUCTIONS,
    &U_INSTRUCTIONS,
    &J_INSTRUCTIONS,
  ];
  let mut map = HashMap::new();
  for set in sets {
    for instructor in set.values() {
      map.insert(instructor.opcode, instructor.inst_type);
    }
  }
  map
});

#[derive(Debug)]
pub(crate) struct Instructor {
  opcode: u8,
  funct: u32,
  inst_type: InstructionType,
  name: &'static str,
  run: fn(inst: Instruction),
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum InstructionType {
  R, I, S, B, U, J
}

impl InstructionType {
  fn query(&self, inst: u32) -> Option<&Instructor> {
    match self {
      InstructionType::R => {
        let funct = inst & 0b11111110000000000111000001111111;
        R_INSTRUCTIONS.get(&funct)
      },
      InstructionType::I => {
        let funct = inst & 0b00000000000000000111000001111111;
        I_INSTRUCTIONS.get(&funct)
      },
      InstructionType::S => {
        let funct = inst & 0b00000000000000000111000001111111;
        S_INSTRUCTIONS.get(&funct)
      },
      InstructionType::B => {
        let funct = inst & 0b00000000000000000111000001111111;
        B_INSTRUCTIONS.get(&funct)
      },
      InstructionType::U => {
        let funct = inst & 0b00000000000000000000000001111111;
        U_INSTRUCTIONS.get(&funct)
      },
      InstructionType::J => {
        let funct = inst & 0b00000000000000000000000001111111;
        J_INSTRUCTIONS.get(&funct)
      },
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Instruction {
  R{
    funct7: u8,
    rs2: u8,
    rs1: u8,
    funct3: u8,
    rd: u8,
    opcode: u8,
  },
  I{
    imm: u16,
    rs1: u8,
    funct3: u8,
    rd: u8,
    opcode: u8,
  },
  S{
    imm: u16,
    rs2: u8,
    rs1: u8,
    funct3: u8,
    opcode: u8,
  },
  B{
    imm: u16,
    rs2: u8,
    rs1: u8,
    funct3: u8,
    opcode: u8,
  },
  U{
    imm: u32,
    rd: u8,
    opcode: u8,
  },
  J{
    imm: u32,
    rd: u8,
    opcode: u8,
  },
}

pub(crate) fn parse(inst: u32) -> (&'static Instructor, Instruction) {
  // TODO: handle None
  let inst_type = INSTRUCTIONS.get(&((inst & 0b1111111) as u8)).unwrap();
  let instructor = inst_type.query(inst).unwrap();
  let instruction = match inst_type {
    InstructionType::R => {
      Instruction::R{
        funct7: ((inst >> 25) & 0b1111111) as u8,
        rs2: ((inst >> 20) & 0b11111) as u8,
        rs1: ((inst >> 15) & 0b11111) as u8,
        funct3: ((inst >> 12) & 0b111) as u8,
        rd: ((inst >> 7) & 0b11111) as u8,
        opcode: (inst & 0b1111111) as u8,
      }
    },
    InstructionType::I => {
      Instruction::I{
        imm: ((inst >> 20) & 0b111111111111) as u16,
        rs1: ((inst >> 15) & 0b11111) as u8,
        funct3: ((inst >> 12) & 0b111) as u8,
        rd: ((inst >> 7) & 0b11111) as u8,
        opcode: (inst & 0b1111111) as u8,
      }
    },
    InstructionType::S => {
      Instruction::S{
        imm: (((inst >> 25) & 0b11111) << 5) as u16 | ((inst >> 7) & 0b11111) as u16,
        rs2: ((inst >> 20) & 0b11111) as u8,
        rs1: ((inst >> 15) & 0b11111) as u8,
        funct3: ((inst >> 12) & 0b111) as u8,
        opcode: (inst & 0b1111111) as u8,
      }
    },
    InstructionType::B => {
      Instruction::B{
        imm: (((inst >> 31) & 0b1) << 12) as u16 | (((inst >> 7) & 0b1) << 11) as u16 | (((inst >> 25) & 0b111111) << 5) as u16 | (((inst >> 8) & 0b1111) << 1) as u16,
        rs2: ((inst >> 20) & 0b11111) as u8,
        rs1: ((inst >> 15) & 0b11111) as u8,
        funct3: ((inst >> 12) & 0b111) as u8,
        opcode: (inst & 0b1111111) as u8,
      }
    },
    InstructionType::U => {
      Instruction::U{
        imm: (inst >> 12) & 0b11111111111111111111,
        rd: ((inst >> 7) & 0b11111) as u8,
        opcode: (inst & 0b1111111) as u8,
      }
    },
    InstructionType::J => {
      Instruction::J{
        imm: (((inst >> 31) & 0b1) << 20) | (((inst >> 12) & 0b11111111) << 12) | (((inst >> 20) & 0b1) << 11) | (((inst >> 21) & 0b1111111111) << 1),
        rd: ((inst >> 7) & 0b11111) as u8,
        opcode: (inst & 0b1111111) as u8,
      }
    },
  };
  (instructor, instruction)
}
