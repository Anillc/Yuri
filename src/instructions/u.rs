use crate::{instructions::{Funct, Instruction}, cpu::Cpu};

use super::Instructor;

pub(crate) fn u_instructions() -> Vec<Instructor> {
  Vec::from([
    Instructor{
      name: "LUI",
      opcode: 0b0110111,
      funct: Funct::U,
      run: |inst, cpu| match inst {
        Instruction::U { imm, rd, opcode: _ } => {
          cpu.regs.set(rd, (imm << 12) as u64);
        },
        _ => unreachable!(),
      },
    },

    Instructor{
      name: "AUIPC",
      opcode: 0b0010111,
      funct: Funct::U,
      run: |inst, cpu| match inst {
        Instruction::U { imm, rd, opcode: _ } => {
          cpu.regs.set(rd, cpu.pc.wrapping_add((imm << 12) as u64))
        },
        _ => unreachable!(),
      }
    },
  ])
}
