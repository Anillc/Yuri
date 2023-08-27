use super::{Instructor, Funct, Instruction};

pub(crate) fn s_instructions() -> Vec<Instructor> {
  Vec::from([
    Instructor {
      name: "SB",
      opcode: 0b0100011,
      funct: Funct::S(0b000),
      run: |inst, cpu| match inst {
        Instruction::S { imm, rs2, rs1, funct3: _, opcode: _ } => {
          let address = cpu.regs[rs1].wrapping_add(imm as u64);
          cpu.mem.write8(address, cpu.regs[rs2] as u8);
        },
        _ => unreachable!(),
      },
    },

    Instructor {
      name: "SH",
      opcode: 0b0100011,
      funct: Funct::S(0b001),
      run: |inst, cpu| match inst {
        Instruction::S { imm, rs2, rs1, funct3: _, opcode: _ } => {
          let address = cpu.regs[rs1].wrapping_add(imm as u64);
          cpu.mem.write16(address, cpu.regs[rs2] as u16);
        },
        _ => unreachable!(),
      },
    },

    Instructor {
      name: "SW",
      opcode: 0b0100011,
      funct: Funct::S(0b010),
      run: |inst, cpu| match inst {
        Instruction::S { imm, rs2, rs1, funct3: _, opcode: _ } => {
          let address = cpu.regs[rs1].wrapping_add(imm as u64);
          cpu.mem.write32(address, cpu.regs[rs2] as u32);
        },
        _ => unreachable!(),
      },
    },

    Instructor {
      name: "SD",
      opcode: 0b0100011,
      funct: Funct::S(0b011),
      run: |inst, cpu| match inst {
        Instruction::S { imm, rs2, rs1, funct3: _, opcode: _ } => {
          let address = cpu.regs[rs1].wrapping_add(imm as u64);
          cpu.mem.write64(address, cpu.regs[rs2]);
        },
        _ => unreachable!(),
      },
    },
  ])
}
