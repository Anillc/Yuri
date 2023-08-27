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

    Instructor {
      name: "ADDI",
      opcode: 0b0010011,
      funct: Funct::I(0b000),
      run: |inst, cpu| match inst {
        Instruction::I { imm, rs1, funct3: _, rd, opcode: _ } => {
          cpu.regs.set(rd, cpu.regs[rs1].wrapping_add(imm as u64));
        },
        _ => unreachable!(),
      }
    },

    Instructor {
      name: "SLTI",
      opcode: 0b0010011,
      funct: Funct::I(0b010),
      run: |inst, cpu| match inst {
        Instruction::I { imm, rs1, funct3: _, rd, opcode: _ } => {
          cpu.regs.set(rd, if (cpu.regs[rs1] as i64) < imm { 1 } else { 0 });
        },
        _ => unreachable!(),
      }
    },

    Instructor {
      name: "SLTIU",
      opcode: 0b0010011,
      funct: Funct::I(0b011),
      run: |inst, cpu| match inst {
        Instruction::I { imm, rs1, funct3: _, rd, opcode: _ } => {
          cpu.regs.set(rd, if cpu.regs[rs1] < imm as u64 { 1 } else { 0 });
        },
        _ => unreachable!(),
      }
    },

    Instructor {
      name: "XORI",
      opcode: 0b0010011,
      funct: Funct::I(0b100),
      run: |inst, cpu| match inst {
        Instruction::I { imm, rs1, funct3: _, rd, opcode: _ } => {
          cpu.regs.set(rd, cpu.regs[rs1] ^ (imm as u64));
        },
        _ => unreachable!(),
      }
    },

    Instructor {
      name: "ORI",
      opcode: 0b0010011,
      funct: Funct::I(0b110),
      run: |inst, cpu| match inst {
        Instruction::I { imm, rs1, funct3: _, rd, opcode: _ } => {
          cpu.regs.set(rd, cpu.regs[rs1] | (imm as u64));
        },
        _ => unreachable!(),
      }
    },

    Instructor {
      name: "ANDI",
      opcode: 0b0010011,
      funct: Funct::I(0b111),
      run: |inst, cpu| match inst {
        Instruction::I { imm, rs1, funct3: _, rd, opcode: _ } => {
          cpu.regs.set(rd, cpu.regs[rs1] & (imm as u64));
        },
        _ => unreachable!(),
      }
    },

    Instructor {
      name: "SLLI",
      opcode: 0b0010011,
      funct: Funct::I(0b001),
      run: |inst, cpu| match inst {
        Instruction::I { imm, rs1, funct3: _, rd, opcode: _ } => {
          // TODO: support rv32i ?
          let shamt = imm & 0b111111;
          cpu.regs.set(rd, cpu.regs[rs1] << shamt);
        },
        _ => unreachable!(),
      }
    },

    Instructor {
      name: "SRLI/SRAI",
      opcode: 0b0010011,
      funct: Funct::I(0b101),
      run: |inst, cpu| match inst {
        Instruction::I { imm, rs1, funct3: _, rd, opcode: _ } => {
          let shamt = imm & 0b111111;
          match imm >> 6 {
            // SRLI
            0b000000 => cpu.regs.set(rd, cpu.regs[rs1] >> shamt),
            // SRAI
            0b010000 => cpu.regs.set(rd, (cpu.regs[rs1] as i64 >> shamt) as u64),
            // TODO: handle unknown instruction
            _ => panic!("unknown instruction"),
          }
        },
        _ => unreachable!(),
      }
    },

    Instructor {
      name: "FENCE",
      opcode: 0b0001111,
      funct: Funct::I(0b000),
      run: |inst, _cpu| match inst {
        Instruction::I { imm: _, rs1: _, funct3: _, rd: _, opcode: _ } => {
          // do nothing
        },
        _ => unreachable!(),
      }
    },

    Instructor {
      name: "ECALL/EBREAK",
      opcode: 0b1110011,
      funct: Funct::I(0b000),
      run: |inst, cpu| match inst {
        Instruction::I { imm, rs1, funct3: _, rd, opcode: _ } => {
          if rs1 != 0 || rd != 0 {
            // TODO
            panic!()
          }
          match imm {
            0b000000000000 => {
              // TODO: ECALL
            },
            0b000000000001 => {
              // TODO: EBREAK
            },
            // TODO
            _ => panic!(),
          }
        },
        _ => unreachable!(),
      }
    },
  ])
}
