use super::{Instructor, Funct, Instruction};

pub(crate) fn r_instructions() -> Vec<Instructor> {
  Vec::from([
    Instructor {
      name: "ADD",
      opcode: 0b0110011,
      funct: Funct::R(0b000, 0b0000000),
      run: |inst, cpu| match inst {
        Instruction::R { funct7: _, rs2, rs1, funct3: _, rd, opcode: _ } => {
          cpu.regs.set(rd, cpu.regs[rs1].wrapping_add(cpu.regs[rs2]));
        },
        _ => unreachable!(),
      },
    },

    Instructor {
      name: "SUB",
      opcode: 0b0110011,
      funct: Funct::R(0b000, 0b0100000),
      run: |inst, cpu| match inst {
        Instruction::R { funct7: _, rs2, rs1, funct3: _, rd, opcode: _ } => {
          cpu.regs.set(rd, cpu.regs[rs1].wrapping_sub(cpu.regs[rs2]));
        },
        _ => unreachable!(),
      },
    },

    Instructor {
      name: "SLL",
      opcode: 0b0110011,
      funct: Funct::R(0b001, 0b0000000),
      run: |inst, cpu| match inst {
        Instruction::R { funct7: _, rs2, rs1, funct3: _, rd, opcode: _ } => {
          let shamt = cpu.regs[rs2] & 0b111111;
          cpu.regs.set(rd, cpu.regs[rs1] << shamt);
        },
        _ => unreachable!(),
      },
    },

    Instructor {
      name: "SLT",
      opcode: 0b0110011,
      funct: Funct::R(0b010, 0b0000000),
      run: |inst, cpu| match inst {
        Instruction::R { funct7: _, rs2, rs1, funct3: _, rd, opcode: _ } => {
          cpu.regs.set(rd, if (cpu.regs[rs1] as i64) < (cpu.regs[rs2] as i64) { 1 } else { 0 });
        },
        _ => unreachable!(),
      },
    },

    Instructor {
      name: "SLTU",
      opcode: 0b0110011,
      funct: Funct::R(0b011, 0b0000000),
      run: |inst, cpu| match inst {
        Instruction::R { funct7: _, rs2, rs1, funct3: _, rd, opcode: _ } => {
          cpu.regs.set(rd, if cpu.regs[rs1] < cpu.regs[rs2] { 1 } else { 0 });
        },
        _ => unreachable!(),
      },
    },

    Instructor {
      name: "XOR",
      opcode: 0b0110011,
      funct: Funct::R(0b100, 0b0000000),
      run: |inst, cpu| match inst {
        Instruction::R { funct7: _, rs2, rs1, funct3: _, rd, opcode: _ } => {
          cpu.regs.set(rd, cpu.regs[rs1] ^ cpu.regs[rs2]);
        },
        _ => unreachable!(),
      },
    },

    Instructor {
      name: "SRL",
      opcode: 0b0110011,
      funct: Funct::R(0b100, 0b0000000),
      run: |inst, cpu| match inst {
        Instruction::R { funct7: _, rs2, rs1, funct3: _, rd, opcode: _ } => {
          let shamt = cpu.regs[rs2] & 0b111111;
          cpu.regs.set(rd, cpu.regs[rs1] >> shamt);
        },
        _ => unreachable!(),
      },
    },

    Instructor {
      name: "SRA",
      opcode: 0b0110011,
      funct: Funct::R(0b100, 0b0100000),
      run: |inst, cpu| match inst {
        Instruction::R { funct7: _, rs2, rs1, funct3: _, rd, opcode: _ } => {
          let shamt = cpu.regs[rs2] & 0b111111;
          cpu.regs.set(rd, ((cpu.regs[rs1] as i64) >> shamt) as u64);
        },
        _ => unreachable!(),
      },
    },

    Instructor {
      name: "OR",
      opcode: 0b0110011,
      funct: Funct::R(0b110, 0b0000000),
      run: |inst, cpu| match inst {
        Instruction::R { funct7: _, rs2, rs1, funct3: _, rd, opcode: _ } => {
          cpu.regs.set(rd, cpu.regs[rs1] | cpu.regs[rs2]);
        },
        _ => unreachable!(),
      },
    },

    Instructor {
      name: "AND",
      opcode: 0b0110011,
      funct: Funct::R(0b111, 0b0000000),
      run: |inst, cpu| match inst {
        Instruction::R { funct7: _, rs2, rs1, funct3: _, rd, opcode: _ } => {
          cpu.regs.set(rd, cpu.regs[rs1] & cpu.regs[rs2]);
        },
        _ => unreachable!(),
      },
    },
  ])
}
