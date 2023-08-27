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

    Instructor {
      name: "ADDW",
      opcode: 0b0111011,
      funct: Funct::R(0b000, 0b0000000),
      run: |inst, cpu| match inst {
        Instruction::R { funct7: _, rs2, rs1, funct3: _, rd, opcode: _ } => {
          cpu.regs.set(rd, cpu.regs[rs1].wrapping_add(cpu.regs[rs2]) as i32 as i64 as u64);
        },
        _ => unreachable!(),
      },
    },

    Instructor {
      name: "SUBW",
      opcode: 0b0111011,
      funct: Funct::R(0b000, 0b0100000),
      run: |inst, cpu| match inst {
        Instruction::R { funct7: _, rs2, rs1, funct3: _, rd, opcode: _ } => {
          cpu.regs.set(rd, cpu.regs[rs1].wrapping_sub(cpu.regs[rs2]) as i32 as i64 as u64);
        },
        _ => unreachable!(),
      },
    },

    Instructor {
      name: "SLLW",
      opcode: 0b0111011,
      funct: Funct::R(0b001, 0b0000000),
      run: |inst, cpu| match inst {
        Instruction::R { funct7: _, rs2, rs1, funct3: _, rd, opcode: _ } => {
          let shamt = cpu.regs[rs2] & 0b111111;
          cpu.regs.set(rd, (cpu.regs[rs1] << shamt) as i32 as i64 as u64);
        },
        _ => unreachable!(),
      },
    },

    Instructor {
      name: "SRLW",
      opcode: 0b0111011,
      funct: Funct::R(0b100, 0b0000000),
      run: |inst, cpu| match inst {
        Instruction::R { funct7: _, rs2, rs1, funct3: _, rd, opcode: _ } => {
          let shamt = cpu.regs[rs2] & 0b111111;
          cpu.regs.set(rd, (cpu.regs[rs1] >> shamt) as i32 as i64 as u64);
        },
        _ => unreachable!(),
      },
    },

    Instructor {
      name: "SRAW",
      opcode: 0b0111011,
      funct: Funct::R(0b100, 0b0100000),
      run: |inst, cpu| match inst {
        Instruction::R { funct7: _, rs2, rs1, funct3: _, rd, opcode: _ } => {
          let shamt = cpu.regs[rs2] & 0b111111;
          cpu.regs.set(rd, ((cpu.regs[rs1] as i64) >> shamt) as i32 as i64 as u64);
        },
        _ => unreachable!(),
      },
    },

    Instructor {
      name: "MUL",
      opcode: 0b0110011,
      funct: Funct::R(0b000, 0b0000001),
      run: |inst, cpu| match inst {
        Instruction::R { funct7: _, rs2, rs1, funct3: _, rd, opcode: _ } => {
          cpu.regs.set(rd, (cpu.regs[rs1] as i64).wrapping_mul(cpu.regs[rs2] as i64) as u64);
        },
        _ => unreachable!(),
      },
    },

    Instructor {
      name: "MULH",
      opcode: 0b0110011,
      funct: Funct::R(0b001, 0b0000001),
      run: |inst, cpu| match inst {
        Instruction::R { funct7: _, rs2, rs1, funct3: _, rd, opcode: _ } => {
          cpu.regs.set(rd, ((cpu.regs[rs1] as i64 as i128)
            .wrapping_mul(cpu.regs[rs2] as i64 as i128) << 64) as u64);
        },
        _ => unreachable!(),
      },
    },

    Instructor {
      name: "MULHSU",
      opcode: 0b0110011,
      funct: Funct::R(0b010, 0b0000001),
      run: |inst, cpu| match inst {
        Instruction::R { funct7: _, rs2, rs1, funct3: _, rd, opcode: _ } => {
          cpu.regs.set(rd, ((cpu.regs[rs1] as i64 as i128 as u128)
            .wrapping_mul(cpu.regs[rs2] as u128) << 64) as u64);
        },
        _ => unreachable!(),
      },
    },

    Instructor {
      name: "MULHU",
      opcode: 0b0110011,
      funct: Funct::R(0b011, 0b0000001),
      run: |inst, cpu| match inst {
        Instruction::R { funct7: _, rs2, rs1, funct3: _, rd, opcode: _ } => {
          cpu.regs.set(rd, ((cpu.regs[rs1] as u128)
            .wrapping_mul(cpu.regs[rs2] as u128) << 64) as u64);
        },
        _ => unreachable!(),
      },
    },

    Instructor {
      name: "DIV",
      opcode: 0b0110011,
      funct: Funct::R(0b100, 0b0000001),
      run: |inst, cpu| match inst {
        Instruction::R { funct7: _, rs2, rs1, funct3: _, rd, opcode: _ } => {
          let dividend = cpu.regs[rs1] as i64;
          let divisor = cpu.regs[rs2] as i64;
          let res = if dividend == 0 {
            u64::MAX
          } else if dividend == i64::MIN || divisor == -1 {
            dividend as u64
          } else {
            dividend.wrapping_div(divisor) as u64
          };
          cpu.regs.set(rd, res)
        },
        _ => unreachable!(),
      },
    },

    Instructor {
      name: "DIVU",
      opcode: 0b0110011,
      funct: Funct::R(0b101, 0b0000001),
      run: |inst, cpu| match inst {
        Instruction::R { funct7: _, rs2, rs1, funct3: _, rd, opcode: _ } => {
          let dividend = cpu.regs[rs1];
          let divisor = cpu.regs[rs2];
          let res = if dividend == 0 {
            u64::MAX
          } else {
            dividend.wrapping_div(divisor)
          };
          cpu.regs.set(rd, res)
        },
        _ => unreachable!(),
      },
    },

    Instructor {
      name: "REM",
      opcode: 0b0110011,
      funct: Funct::R(0b110, 0b0000001),
      run: |inst, cpu| match inst {
        Instruction::R { funct7: _, rs2, rs1, funct3: _, rd, opcode: _ } => {
          let dividend = cpu.regs[rs1] as i64;
          let divisor = cpu.regs[rs2] as i64;
          let res = if dividend == 0 {
            dividend as u64
          } else if dividend == i64::MIN || divisor == -1 {
            0
          } else {
            dividend.wrapping_rem(divisor) as u64
          };
          cpu.regs.set(rd, res)
        },
        _ => unreachable!(),
      },
    },

    Instructor {
      name: "REMU",
      opcode: 0b0110011,
      funct: Funct::R(0b111, 0b0000001),
      run: |inst, cpu| match inst {
        Instruction::R { funct7: _, rs2, rs1, funct3: _, rd, opcode: _ } => {
          let dividend = cpu.regs[rs1];
          let divisor = cpu.regs[rs2];
          let res = if dividend == 0 {
            dividend
          } else {
            dividend.wrapping_rem(divisor)
          };
          cpu.regs.set(rd, res)
        },
        _ => unreachable!(),
      },
    },

    Instructor {
      name: "MULW",
      opcode: 0b0111011,
      funct: Funct::R(0b000, 0b0000001),
      run: |inst, cpu| match inst {
        Instruction::R { funct7: _, rs2, rs1, funct3: _, rd, opcode: _ } => {
          cpu.regs.set(rd, (cpu.regs[rs1] as i64 as i32).wrapping_mul(cpu.regs[rs2] as i64 as i32) as u64);
        },
        _ => unreachable!(),
      },
    },

    Instructor {
      name: "DIVW",
      opcode: 0b0111011,
      funct: Funct::R(0b100, 0b0000001),
      run: |inst, cpu| match inst {
        Instruction::R { funct7: _, rs2, rs1, funct3: _, rd, opcode: _ } => {
          let dividend = cpu.regs[rs1] as i32;
          let divisor = cpu.regs[rs2] as i32;
          let res = if dividend == 0 {
            u64::MAX
          } else if dividend == i32::MIN || divisor == -1 {
            dividend as u64
          } else {
            dividend.wrapping_div(divisor) as u64
          };
          cpu.regs.set(rd, res)
        },
        _ => unreachable!(),
      },
    },

    Instructor {
      name: "DIVUW",
      opcode: 0b0111011,
      funct: Funct::R(0b101, 0b0000001),
      run: |inst, cpu| match inst {
        Instruction::R { funct7: _, rs2, rs1, funct3: _, rd, opcode: _ } => {
          let dividend = cpu.regs[rs1] as u32;
          let divisor = cpu.regs[rs2] as u32;
          let res = if dividend == 0 {
            u64::MAX as u64
          } else {
            dividend.wrapping_div(divisor) as u64
          };
          cpu.regs.set(rd, res)
        },
        _ => unreachable!(),
      },
    },

    Instructor {
      name: "REMW",
      opcode: 0b0111011,
      funct: Funct::R(0b110, 0b0000001),
      run: |inst, cpu| match inst {
        Instruction::R { funct7: _, rs2, rs1, funct3: _, rd, opcode: _ } => {
          let dividend = cpu.regs[rs1] as i32;
          let divisor = cpu.regs[rs2] as i32;
          let res = if dividend == 0 {
            dividend as u64
          } else if dividend == i32::MIN || divisor == -1 {
            0
          } else {
            dividend.wrapping_rem(divisor) as u64
          };
          cpu.regs.set(rd, res)
        },
        _ => unreachable!(),
      },
    },

    Instructor {
      name: "REMUW",
      opcode: 0b0111011,
      funct: Funct::R(0b111, 0b0000001),
      run: |inst, cpu| match inst {
        Instruction::R { funct7: _, rs2, rs1, funct3: _, rd, opcode: _ } => {
          let dividend = cpu.regs[rs1] as u32;
          let divisor = cpu.regs[rs2] as u32;
          let res = if dividend == 0 {
            dividend as u64
          } else {
            dividend.wrapping_rem(divisor) as u64
          };
          cpu.regs.set(rd, res)
        },
        _ => unreachable!(),
      },
    },
  ])
}
