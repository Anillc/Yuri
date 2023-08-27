use super::{Instructor, Funct, Instruction};

pub(crate) fn i_instructions() -> Vec<Instructor> {
  Vec::from([
    Instructor {
      name: "JALR",
      opcode: 0b1100111,
      funct: Funct::I(0b000),
      run: |inst, cpu| match inst {
        Instruction::I { imm, rs1, funct3: _, rd, opcode: _ } => {
          let res = cpu.pc.wrapping_add(4);
          cpu.pc = cpu.regs[rs1].wrapping_add(imm as u64);
          cpu.regs.set(rd, res);
        },
        _ => unreachable!(),
      },
    },

    Instructor {
      name: "LB",
      opcode: 0b0000011,
      funct: Funct::I(0b000),
      run: |inst, cpu| match inst {
        Instruction::I { imm, rs1, funct3: _, rd, opcode: _ } => {
          let address = cpu.regs[rs1].wrapping_add(imm as u64);
          let data = cpu.mem.read8(address) as i8 as i64 as u64;
          cpu.regs.set(rd, data);
        },
        _ => unreachable!(),
      },
    },

    Instructor {
      name: "LH",
      opcode: 0b0000011,
      funct: Funct::I(0b001),
      run: |inst, cpu| match inst {
        Instruction::I { imm, rs1, funct3: _, rd, opcode: _ } => {
          let address = cpu.regs[rs1].wrapping_add(imm as u64);
          let data = cpu.mem.read16(address) as i16 as i64 as u64;
          cpu.regs.set(rd, data);
        },
        _ => unreachable!(),
      },
    },

    Instructor {
      name: "LW",
      opcode: 0b0000011,
      funct: Funct::I(0b100),
      run: |inst, cpu| match inst {
        Instruction::I { imm, rs1, funct3: _, rd, opcode: _ } => {
          let address = cpu.regs[rs1].wrapping_add(imm as u64);
          let data = cpu.mem.read32(address) as i32 as i64 as u64;
          cpu.regs.set(rd, data);
        },
        _ => unreachable!(),
      },
    },

    Instructor {
      name: "LBU",
      opcode: 0b0000011,
      funct: Funct::I(0b000),
      run: |inst, cpu| match inst {
        Instruction::I { imm, rs1, funct3: _, rd, opcode: _ } => {
          let address = cpu.regs[rs1].wrapping_add(imm as u64);
          let data = cpu.mem.read8(address) as u64;
          cpu.regs.set(rd, data);
        },
        _ => unreachable!(),
      },
    },

    Instructor {
      name: "LHU",
      opcode: 0b0000011,
      funct: Funct::I(0b101),
      run: |inst, cpu| match inst {
        Instruction::I { imm, rs1, funct3: _, rd, opcode: _ } => {
          let address = cpu.regs[rs1].wrapping_add(imm as u64);
          let data = cpu.mem.read16(address) as u64;
          cpu.regs.set(rd, data);
        },
        _ => unreachable!(),
      },
    },
  ])
}
