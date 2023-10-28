use crate::{instructions::{Instructor, InstructionSegment}, hart::Mode, trap::Exception};

use super::{U, InstructionParser, funct3, funct37, J, I, B, R, S};

pub(crate) fn i() -> Vec<Instructor> {
  Vec::from([
    Instructor {
      name: "LUI",
      opcode: 0b0110111,
      segments: vec![],
      run: |inst, _len, hart| {
        let U { imm, rd } = inst.u();
        hart.regs.set(rd, imm as u64);
        Ok(())
      },
    },

    Instructor {
      name: "AUIPC",
      opcode: 0b0010111,
      segments: vec![],
      run: |inst, _len, hart| {
        let U { imm, rd } = inst.u();
        hart.regs.set(rd, hart.pc.wrapping_add(imm as u64));
        Ok(())
      }
    },

    Instructor {
      name: "JAL",
      opcode: 0b1101111,
      segments: vec![],
      run: |inst, len, hart| {
        let J { imm, rd } = inst.j();
        let res = hart.pc.wrapping_add(if len == 32 { 4 } else { 2 });
        hart.pc = hart.pc.wrapping_add(imm as u64)
          .wrapping_sub(if len == 32 { 4 } else { 2 });
        hart.regs.set(rd, res);
        Ok(())
      },
    },

    Instructor {
      name: "JALR",
      opcode: 0b1100111,
      segments: funct3(0b000),
      run: |inst, len, hart| {
        let I { imm, rs1, rd } = inst.i();
        let res = hart.pc.wrapping_add(if len == 32 { 4 } else { 2 });
        hart.pc = hart.regs[rs1].wrapping_add(imm as u64)
          .wrapping_sub(if len == 32 { 4 } else { 2 });
        hart.regs.set(rd, res);
        Ok(())
      },
    },

    Instructor {
      name: "BEQ",
      opcode: 0b1100011,
      segments: funct3(0b000),
      run: |inst, len, hart| {
        let B { imm, rs2, rs1 } = inst.b();
        if hart.regs[rs1] == hart.regs[rs2] {
          hart.pc = hart.pc.wrapping_add(imm as u64)
            .wrapping_sub(if len == 32 { 4 } else { 2 });
        }
        Ok(())
      },
    },

    Instructor {
      name: "BNE",
      opcode: 0b1100011,
      segments: funct3(0b001),
      run: |inst, len, hart| {
        let B { imm, rs2, rs1 } = inst.b();
        if hart.regs[rs1] != hart.regs[rs2] {
          hart.pc = hart.pc.wrapping_add(imm as u64)
            .wrapping_sub(if len == 32 { 4 } else { 2 });
        }
        Ok(())
      },
    },

    Instructor {
      name: "BLT",
      opcode: 0b1100011,
      segments: funct3(0b100),
      run: |inst, len, hart| {
        let B { imm, rs2, rs1 } = inst.b();
        if (hart.regs[rs1] as i64) < (hart.regs[rs2] as i64) {
          hart.pc = hart.pc.wrapping_add(imm as u64)
            .wrapping_sub(if len == 32 { 4 } else { 2 });
        }
        Ok(())
      },
    },

    Instructor {
      name: "BGE",
      opcode: 0b1100011,
      segments: funct3(0b101),
      run: |inst, len, hart| {
        let B { imm, rs2, rs1 } = inst.b();
        if (hart.regs[rs1] as i64) >= (hart.regs[rs2] as i64) {
          hart.pc = hart.pc.wrapping_add(imm as u64)
            .wrapping_sub(if len == 32 { 4 } else { 2 });
        }
        Ok(())
      },
    },

    Instructor {
      name: "BLTU",
      opcode: 0b1100011,
      segments: funct3(0b110),
      run: |inst, len, hart| {
        let B { imm, rs2, rs1 } = inst.b();
        if hart.regs[rs1] < hart.regs[rs2] {
          hart.pc = hart.pc.wrapping_add(imm as u64)
            .wrapping_sub(if len == 32 { 4 } else { 2 });
        }
        Ok(())
      },
    },

    Instructor {
      name: "BGEU",
      opcode: 0b1100011,
      segments: funct3(0b111),
      run: |inst, len, hart| {
        let B { imm, rs2, rs1 } = inst.b();
        if hart.regs[rs1] >= hart.regs[rs2] {
          hart.pc = hart.pc.wrapping_add(imm as u64)
            .wrapping_sub(if len == 32 { 4 } else { 2 });
        }
        Ok(())
      },
    },

    Instructor {
      name: "LB",
      opcode: 0b0000011,
      segments: funct3(0b000),
      run: |inst, _len, hart| {
        let I { imm, rs1, rd } = inst.i();
        let address = hart.regs[rs1].wrapping_add(imm as u64);
        let data = hart.mem.read8(address) as i8 as i64 as u64;
        hart.regs.set(rd, data);
        Ok(())
      },
    },

    Instructor {
      name: "LH",
      opcode: 0b0000011,
      segments: funct3(0b001),
      run: |inst, _len, hart| {
        let I { imm, rs1, rd } = inst.i();
        let address = hart.regs[rs1].wrapping_add(imm as u64);
        let data = hart.mem.read16(address) as i16 as i64 as u64;
        hart.regs.set(rd, data);
        Ok(())
      },
    },

    Instructor {
      name: "LW",
      opcode: 0b0000011,
      segments: funct3(0b010),
      run: |inst, _len, hart| {
        let I { imm, rs1, rd } = inst.i();
        let address = hart.regs[rs1].wrapping_add(imm as u64);
        let data = hart.mem.read32(address) as i32 as i64 as u64;
        hart.regs.set(rd, data);
        Ok(())
      },
    },

    Instructor {
      name: "LBU",
      opcode: 0b0000011,
      segments: funct3(0b100),
      run: |inst, _len, hart| {
        let I { imm, rs1, rd } = inst.i();
        let address = hart.regs[rs1].wrapping_add(imm as u64);
        let data = hart.mem.read8(address) as u64;
        hart.regs.set(rd, data);
        Ok(())
      },
    },

    Instructor {
      name: "LHU",
      opcode: 0b0000011,
      segments: funct3(0b101),
      run: |inst, _len, hart| {
        let I { imm, rs1, rd } = inst.i();
        let address = hart.regs[rs1].wrapping_add(imm as u64);
        let data = hart.mem.read16(address) as u64;
        hart.regs.set(rd, data);
        Ok(())
      },
    },

    Instructor {
      name: "SB",
      opcode: 0b0100011,
      segments: funct3(0b000),
      run: |inst, _len, hart| {
        let S { imm, rs2, rs1 } = inst.s();
        let address = hart.regs[rs1].wrapping_add(imm as u64);
        hart.mem.write8(address, hart.regs[rs2] as u8);
        Ok(())
      },
    },

    Instructor {
      name: "SH",
      opcode: 0b0100011,
      segments: funct3(0b001),
      run: |inst, _len, hart| {
        let S { imm, rs2, rs1 } = inst.s();
        let address = hart.regs[rs1].wrapping_add(imm as u64);
        hart.mem.write16(address, hart.regs[rs2] as u16);
        Ok(())
      },
    },

    Instructor {
      name: "SW",
      opcode: 0b0100011,
      segments: funct3(0b010),
      run: |inst, _len, hart| {
        let S { imm, rs2, rs1 } = inst.s();
        let address = hart.regs[rs1].wrapping_add(imm as u64);
        hart.mem.write32(address, hart.regs[rs2] as u32);
        Ok(())
      },
    },

    Instructor {
      name: "ADDI",
      opcode: 0b0010011,
      segments: funct3(0b000),
      run: |inst, _len, hart| {
        let I { imm, rs1, rd } = inst.i();
        hart.regs.set(rd, hart.regs[rs1].wrapping_add(imm as u64));
        Ok(())
      },
    },

    Instructor {
      name: "SLTI",
      opcode: 0b0010011,
      segments: funct3(0b010),
      run: |inst, _len, hart| {
        let I { imm, rs1, rd } = inst.i();
        hart.regs.set(rd, if (hart.regs[rs1] as i64) < imm { 1 } else { 0 });
        Ok(())
      },
    },

    Instructor {
      name: "SLTIU",
      opcode: 0b0010011,
      segments: funct3(0b011),
      run: |inst, _len, hart| {
        let I { imm, rs1, rd } = inst.i();
        hart.regs.set(rd, if hart.regs[rs1] < imm as u64 { 1 } else { 0 });
        Ok(())
      },
    },

    Instructor {
      name: "XORI",
      opcode: 0b0010011,
      segments: funct3(0b100),
      run: |inst, _len, hart| {
        let I { imm, rs1, rd } = inst.i();
        hart.regs.set(rd, hart.regs[rs1] ^ (imm as u64));
        Ok(())
      },
    },

    Instructor {
      name: "ORI",
      opcode: 0b0010011,
      segments: funct3(0b110),
      run: |inst, _len, hart| {
        let I { imm, rs1, rd } = inst.i();
        hart.regs.set(rd, hart.regs[rs1] | (imm as u64));
        Ok(())
      },
    },

    Instructor {
      name: "ANDI",
      opcode: 0b0010011,
      segments: funct3(0b111),
      run: |inst, _len, hart| {
        let I { imm, rs1, rd } = inst.i();
        hart.regs.set(rd, hart.regs[rs1] & (imm as u64));
        Ok(())
      },
    },

    Instructor {
      name: "ADD",
      opcode: 0b0110011,
      segments: funct37(0b000, 0b0000000),
      run: |inst, _len, hart| {
        let R { rs2, rs1, rd } = inst.r();
        hart.regs.set(rd, hart.regs[rs1].wrapping_add(hart.regs[rs2]));
        Ok(())
      },
    },

    Instructor {
      name: "SUB",
      opcode: 0b0110011,
      segments: funct37(0b000, 0b0100000),
      run: |inst, _len, hart| {
        let R { rs2, rs1, rd } = inst.r();
        hart.regs.set(rd, hart.regs[rs1].wrapping_sub(hart.regs[rs2]));
        Ok(())
      },
    },

    Instructor {
      name: "SLL",
      opcode: 0b0110011,
      segments: funct37(0b001, 0b0000000),
      run: |inst, _len, hart| {
        let R { rs2, rs1, rd } = inst.r();
        let shamt = hart.regs[rs2] & 0b111111;
        hart.regs.set(rd, hart.regs[rs1] << shamt);
        Ok(())
      },
    },

    Instructor {
      name: "SLT",
      opcode: 0b0110011,
      segments: funct37(0b010, 0b0000000),
      run: |inst, _len, hart| {
        let R { rs2, rs1, rd } = inst.r();
        hart.regs.set(rd, if (hart.regs[rs1] as i64) < (hart.regs[rs2] as i64) { 1 } else { 0 });
        Ok(())
      },
    },

    Instructor {
      name: "SLTU",
      opcode: 0b0110011,
      segments: funct37(0b011, 0b0000000),
      run: |inst, _len, hart| {
        let R { rs2, rs1, rd } = inst.r();
        hart.regs.set(rd, if hart.regs[rs1] < hart.regs[rs2] { 1 } else { 0 });
        Ok(())
      },
    },

    Instructor {
      name: "XOR",
      opcode: 0b0110011,
      segments: funct37(0b100, 0b0000000),
      run: |inst, _len, hart| {
        let R { rs2, rs1, rd } = inst.r();
        hart.regs.set(rd, hart.regs[rs1] ^ hart.regs[rs2]);
        Ok(())
      },
    },

    Instructor {
      name: "SRL",
      opcode: 0b0110011,
      segments: funct37(0b101, 0b0000000),
      run: |inst, _len, hart| {
        let R { rs2, rs1, rd } = inst.r();
        let shamt = hart.regs[rs2] & 0b111111;
        hart.regs.set(rd, hart.regs[rs1] >> shamt);
        Ok(())
      },
    },

    Instructor {
      name: "SRA",
      opcode: 0b0110011,
      segments: funct37(0b101, 0b0100000),
      run: |inst, _len, hart| {
        let R { rs2, rs1, rd } = inst.r();
        let shamt = hart.regs[rs2] & 0b111111;
        hart.regs.set(rd, ((hart.regs[rs1] as i64) >> shamt) as u64);
        Ok(())
      },
    },

    Instructor {
      name: "OR",
      opcode: 0b0110011,
      segments: funct37(0b110, 0b0000000),
      run: |inst, _len, hart| {
        let R { rs2, rs1, rd } = inst.r();
        hart.regs.set(rd, hart.regs[rs1] | hart.regs[rs2]);
        Ok(())
      },
    },

    Instructor {
      name: "AND",
      opcode: 0b0110011,
      segments: funct37(0b111, 0b0000000),
      run: |inst, _len, hart| {
        let R { rs2, rs1, rd } = inst.r();
        hart.regs.set(rd, hart.regs[rs1] & hart.regs[rs2]);
        Ok(())
      },
    },

    Instructor {
      name: "FENCE",
      opcode: 0b0001111,
      segments: funct3(0b000),
      run: |_inst, _len, _hart| {
        // do nothing
        Ok(())
      },
    },

    Instructor {
      name: "ECALL",
      opcode: 0b1110011,
      segments: vec![
        InstructionSegment { start: 7, end: 31, comp: 0b0000000000000000000000000 }
      ],
      run: |_inst, _len, hart| {
        Err(match hart.mode {
          Mode::User => Exception::EnvironmentCallFromUMode,
          Mode::Supervisor => Exception::EnvironmentCallFromSMode,
          Mode::Machine => Exception::EnvironmentCallFromMMode,
        })
      },
    },

    Instructor {
      name: "EBREAK",
      opcode: 0b1110011,
      segments: vec![
        InstructionSegment { start: 7, end: 31, comp: 0b0000000000010000000000000 }
      ],
      run: |_inst, _len, hart| {
        Err(Exception::Breakpoint(hart.pc))
      },
    },

    Instructor {
      name: "LWU",
      opcode: 0b0000011,
      segments: funct3(0b110),
      run: |inst, _len, hart| {
        let I { imm, rs1, rd } = inst.i();
        let address = hart.regs[rs1].wrapping_add(imm as u64);
        let data = hart.mem.read32(address) as u64;
        hart.regs.set(rd, data);
        Ok(())
      },
    },

    Instructor {
      name: "LD",
      opcode: 0b0000011,
      segments: funct3(0b011),
      run: |inst, _len, hart| {
        let I { imm, rs1, rd } = inst.i();
        let address = hart.regs[rs1].wrapping_add(imm as u64);
        let data = hart.mem.read64(address);
        hart.regs.set(rd, data);
        Ok(())
      },
    },

    Instructor {
      name: "SD",
      opcode: 0b0100011,
      segments: funct3(0b011),
      run: |inst, _len, hart| {
        let S { imm, rs2, rs1 } = inst.s();
        let address = hart.regs[rs1].wrapping_add(imm as u64);
        hart.mem.write64(address, hart.regs[rs2]);
        Ok(())
      },
    },

    Instructor {
      name: "SLLI",
      opcode: 0b0010011,
      segments: vec![
        InstructionSegment { start: 12, end: 14, comp: 0b001 },
        InstructionSegment { start: 26, end: 31, comp: 0b000000 },
      ],
      run: |inst, _len, hart| {
        let shamt = inst >> 20 & 0b111111;
        let rs1 = (inst >> 15 & 0b11111) as usize;
        let rd = (inst >> 7 & 0b11111) as usize;
        hart.regs.set(rd, hart.regs[rs1] << shamt);
        Ok(())
      },
    },

    Instructor {
      name: "SRLI",
      opcode: 0b0010011,
      segments: vec![
        InstructionSegment { start: 12, end: 14, comp: 0b101 },
        InstructionSegment { start: 26, end: 31, comp: 0b000000 },
      ],
      run: |inst, _len, hart| {
        let shamt = inst >> 20 & 0b111111;
        let rs1 = (inst >> 15 & 0b11111) as usize;
        let rd = (inst >> 7 & 0b11111) as usize;
        hart.regs.set(rd, hart.regs[rs1] >> shamt);
        Ok(())
      },
    },

    Instructor {
      name: "SRAI",
      opcode: 0b0010011,
      segments: vec![
        InstructionSegment { start: 12, end: 14, comp: 0b101 },
        InstructionSegment { start: 26, end: 31, comp: 0b010000 },
      ],
      run: |inst, _len, hart| {
        let shamt = inst >> 20 & 0b111111;
        let rs1 = (inst >> 15 & 0b11111) as usize;
        let rd = (inst >> 7 & 0b11111) as usize;
        hart.regs.set(rd, (hart.regs[rs1] as i64 >> shamt) as u64);
        Ok(())
      },
    },

    Instructor {
      name: "ADDIW",
      opcode: 0b0011011,
      segments: funct3(0b000),
      run: |inst, _len, hart| {
        let I { imm, rs1, rd } = inst.i();
        hart.regs.set(rd, (hart.regs[rs1] as u32 as i32).wrapping_add(imm as i32) as i64 as u64);
        Ok(())
      },
    },

    Instructor {
      name: "SLLIW",
      opcode: 0b0011011,
      segments: vec![
        InstructionSegment { start: 12, end: 14, comp: 0b001 },
        InstructionSegment { start: 26, end: 31, comp: 0b000000 },
      ],
      run: |inst, _len, hart| {
        let shamt = inst >> 20 & 0b11111;
        let rs1 = (inst >> 15 & 0b11111) as usize;
        let rd = (inst >> 7 & 0b11111) as usize;
        hart.regs.set(rd, ((hart.regs[rs1] as u32 as i32) << shamt) as i64 as u64);
        Ok(())
      },
    },

    Instructor {
      name: "SRLIW",
      opcode: 0b0011011,
      segments: vec![
        InstructionSegment { start: 12, end: 14, comp: 0b101 },
        InstructionSegment { start: 26, end: 31, comp: 0b000000 },
      ],
      run: |inst, _len, hart| {
        let shamt = inst >> 20 & 0b11111;
        let rs1 = (inst >> 15 & 0b11111) as usize;
        let rd = (inst >> 7 & 0b11111) as usize;
        hart.regs.set(rd, (hart.regs[rs1] as u32 >> shamt) as i32 as i64 as u64);
        Ok(())
      },
    },

    Instructor {
      name: "SRAIW",
      opcode: 0b0011011,
      segments: vec![
        InstructionSegment { start: 12, end: 14, comp: 0b101 },
        InstructionSegment { start: 26, end: 31, comp: 0b010000 },
      ],
      run: |inst, _len, hart| {
        let shamt = inst >> 20 & 0b11111;
        let rs1 = (inst >> 15 & 0b11111) as usize;
        let rd = (inst >> 7 & 0b11111) as usize;
        hart.regs.set(rd, (hart.regs[rs1] as u32 as i32 >> shamt) as i64 as u64);
        Ok(())
      },
    },

    Instructor {
      name: "ADDW",
      opcode: 0b0111011,
      segments: funct37(0b000, 0b0000000),
      run: |inst, _len, hart| {
        let R { rs2, rs1, rd } = inst.r();
        hart.regs.set(rd, (hart.regs[rs1] as u32 as i32).wrapping_add(hart.regs[rs2] as u32 as i32) as i64 as u64);
        Ok(())
      },
    },

    Instructor {
      name: "SUBW",
      opcode: 0b0111011,
      segments: funct37(0b000, 0b0100000),
      run: |inst, _len, hart| {
        let R { rs2, rs1, rd } = inst.r();
        hart.regs.set(rd, (hart.regs[rs1] as u32 as i32).wrapping_sub(hart.regs[rs2] as u32 as i32) as i64 as u64);
        Ok(())
      },
    },

    Instructor {
      name: "SLLW",
      opcode: 0b0111011,
      segments: funct37(0b001, 0b0000000),
      run: |inst, _len, hart| {
        let R { rs2, rs1, rd } = inst.r();
        let shamt = hart.regs[rs2] & 0b11111;
        hart.regs.set(rd, ((hart.regs[rs1] as u32 as i32) << shamt) as i64 as u64);
        Ok(())
      },
    },

    Instructor {
      name: "SRLW",
      opcode: 0b0111011,
      segments: funct37(0b101, 0b0000000),
      run: |inst, _len, hart| {
        let R { rs2, rs1, rd } = inst.r();
        let shamt = hart.regs[rs2] & 0b11111;
        hart.regs.set(rd, (hart.regs[rs1] as u32 >> shamt) as i32 as i64 as u64);
        Ok(())
      },
    },

    Instructor {
      name: "SRAW",
      opcode: 0b0111011,
      segments: funct37(0b101, 0b0100000),
      run: |inst, _len, hart| {
        let R { rs2, rs1, rd } = inst.r();
        let shamt = hart.regs[rs2] & 0b11111;
        hart.regs.set(rd, (hart.regs[rs1] as u32 as i32 >> shamt) as i64 as u64);
        Ok(())
      },
    },
  ])
}