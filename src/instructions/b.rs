use super::{Instructor, Funct, Instruction};

pub(crate) fn b_instructions() -> Vec<Instructor> {
  Vec::from([
    Instructor {
      name: "BEQ",
      opcode: 0b1100011,
      funct: Funct::B(0b000),
      run: |inst, cpu| match inst {
        Instruction::B { imm, rs2, rs1, funct3: _, opcode: _ } => {
          if cpu.regs[rs1] == cpu.regs[rs2] {
            cpu.pc = cpu.pc.wrapping_add(imm as u64);
          }
        },
        _ => unreachable!(),
      },
    },

    Instructor {
      name: "BNE",
      opcode: 0b1100011,
      funct: Funct::B(0b001),
      run: |inst, cpu| match inst {
        Instruction::B { imm, rs2, rs1, funct3: _, opcode: _ } => {
          if cpu.regs[rs1] != cpu.regs[rs2] {
            cpu.pc = cpu.pc.wrapping_add(imm as u64);
          }
        },
        _ => unreachable!(),
      },
    },

    Instructor {
      name: "BLT",
      opcode: 0b1100011,
      funct: Funct::B(0b100),
      run: |inst, cpu| match inst {
        Instruction::B { imm, rs2, rs1, funct3: _, opcode: _ } => {
          if (cpu.regs[rs1] as i64) < (cpu.regs[rs2] as i64) {
            cpu.pc = cpu.pc.wrapping_add(imm as u64);
          }
        },
        _ => unreachable!(),
      },
    },

    Instructor {
      name: "BGE",
      opcode: 0b1100011,
      funct: Funct::B(0b101),
      run: |inst, cpu| match inst {
        Instruction::B { imm, rs2, rs1, funct3: _, opcode: _ } => {
          if (cpu.regs[rs1] as i64) > (cpu.regs[rs2] as i64) {
            cpu.pc = cpu.pc.wrapping_add(imm as u64);
          }
        },
        _ => unreachable!(),
      },
    },

    Instructor {
      name: "BLTU",
      opcode: 0b1100011,
      funct: Funct::B(0b110),
      run: |inst, cpu| match inst {
        Instruction::B { imm, rs2, rs1, funct3: _, opcode: _ } => {
          if cpu.regs[rs1] < cpu.regs[rs2] {
            cpu.pc = cpu.pc.wrapping_add(imm as u64);
          }
        },
        _ => unreachable!(),
      },
    },

    Instructor {
      name: "BGEU",
      opcode: 0b1100011,
      funct: Funct::B(0b111),
      run: |inst, cpu| match inst {
        Instruction::B { imm, rs2, rs1, funct3: _, opcode: _ } => {
          if cpu.regs[rs1] > cpu.regs[rs2] {
            cpu.pc = cpu.pc.wrapping_add(imm as u64);
          }
        },
        _ => unreachable!(),
      },
    },
  ])
}
