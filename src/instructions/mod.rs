use std::collections::HashMap;

use once_cell::sync::Lazy;

static INSTRUCTIONS: Lazy<HashMap<u32, Instructor>> = Lazy::new(|| {
  HashMap::new()
});

pub(crate) struct Instructor {
  id: u32,
  name: &'static str,
  instruction_type: InstructionType,
  run: fn(inst: Instruction),
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum InstructionType {
  R, I, S, B, U, J
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

pub(crate) fn parse(inst: u32, inst_type: InstructionType) -> Instruction {
  match inst_type {
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
  }
}
