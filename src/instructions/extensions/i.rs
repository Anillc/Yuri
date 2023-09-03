use crate::{instructions::{Instructor, InstructionSegment, InstructorResult}, syscall::SYSCALL_MAP};

use super::{U, InstructionParser, funct3, funct37, J, I, B, R, S};

pub(crate) fn i() -> Vec<Instructor> {
  Vec::from([
    Instructor {
      name: "LUI",
      opcode: 0b0110111,
      segments: vec![],
      run: |inst, cpu| {
        let U { imm, rd } = inst.u();
        cpu.regs.set(rd, imm as u64);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "AUIPC",
      opcode: 0b0010111,
      segments: vec![],
      run: |inst, cpu| {
        let U { imm, rd } = inst.u();
        cpu.regs.set(rd, cpu.pc.wrapping_add(imm as u64));
        InstructorResult::Success
      }
    },

    Instructor {
      name: "JAL",
      opcode: 0b1101111,
      segments: vec![],
      run: |inst, cpu| {
        let J { imm, rd } = inst.j();
        let res = cpu.pc.wrapping_add(4);
        cpu.pc = cpu.pc.wrapping_add(imm as u64);
        cpu.regs.set(rd, res);
        InstructorResult::Jump
      },
    },

    Instructor {
      name: "JALR",
      opcode: 0b1100111,
      segments: funct3(0b000),
      run: |inst, cpu| {
        let I { imm, rs1, rd } = inst.i();
        let res = cpu.pc.wrapping_add(4);
        cpu.pc = cpu.regs[rs1].wrapping_add(imm as u64);
        cpu.regs.set(rd, res);
        InstructorResult::Jump
      },
    },

    Instructor {
      name: "BEQ",
      opcode: 0b1100011,
      segments: funct3(0b000),
      run: |inst, cpu| {
        let B { imm, rs2, rs1 } = inst.b();
        if cpu.regs[rs1] == cpu.regs[rs2] {
          cpu.pc = cpu.pc.wrapping_add(imm as u64);
          InstructorResult::Jump
        } else {
          InstructorResult::Success
        }
      },
    },

    Instructor {
      name: "BNE",
      opcode: 0b1100011,
      segments: funct3(0b001),
      run: |inst, cpu| {
        let B { imm, rs2, rs1 } = inst.b();
        if cpu.regs[rs1] != cpu.regs[rs2] {
          cpu.pc = cpu.pc.wrapping_add(imm as u64);
          InstructorResult::Jump
        } else {
          InstructorResult::Success
        }
      },
    },

    Instructor {
      name: "BLT",
      opcode: 0b1100011,
      segments: funct3(0b100),
      run: |inst, cpu| {
        let B { imm, rs2, rs1 } = inst.b();
        if (cpu.regs[rs1] as i64) < (cpu.regs[rs2] as i64) {
          cpu.pc = cpu.pc.wrapping_add(imm as u64);
          InstructorResult::Jump
        } else {
          InstructorResult::Success
        }
      },
    },

    Instructor {
      name: "BGE",
      opcode: 0b1100011,
      segments: funct3(0b101),
      run: |inst, cpu| {
        let B { imm, rs2, rs1 } = inst.b();
        if (cpu.regs[rs1] as i64) >= (cpu.regs[rs2] as i64) {
          cpu.pc = cpu.pc.wrapping_add(imm as u64);
          InstructorResult::Jump
        } else {
          InstructorResult::Success
        }
      },
    },

    Instructor {
      name: "BLTU",
      opcode: 0b1100011,
      segments: funct3(0b110),
      run: |inst, cpu| {
        let B { imm, rs2, rs1 } = inst.b();
        if cpu.regs[rs1] < cpu.regs[rs2] {
          cpu.pc = cpu.pc.wrapping_add(imm as u64);
          InstructorResult::Jump
        } else {
          InstructorResult::Success
        }
      },
    },

    Instructor {
      name: "BGEU",
      opcode: 0b1100011,
      segments: funct3(0b111),
      run: |inst, cpu| {
        let B { imm, rs2, rs1 } = inst.b();
        if cpu.regs[rs1] >= cpu.regs[rs2] {
          cpu.pc = cpu.pc.wrapping_add(imm as u64);
          InstructorResult::Jump
        } else {
          InstructorResult::Success
        }
      },
    },

    Instructor {
      name: "LB",
      opcode: 0b0000011,
      segments: funct3(0b000),
      run: |inst, cpu| {
        let I { imm, rs1, rd } = inst.i();
        let address = cpu.regs[rs1].wrapping_add(imm as u64);
        let data = cpu.mem.read8(address) as i8 as i64 as u64;
        cpu.regs.set(rd, data);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "LH",
      opcode: 0b0000011,
      segments: funct3(0b001),
      run: |inst, cpu| {
        let I { imm, rs1, rd } = inst.i();
        let address = cpu.regs[rs1].wrapping_add(imm as u64);
        let data = cpu.mem.read16(address) as i16 as i64 as u64;
        cpu.regs.set(rd, data);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "LW",
      opcode: 0b0000011,
      segments: funct3(0b010),
      run: |inst, cpu| {
        let I { imm, rs1, rd } = inst.i();
        let address = cpu.regs[rs1].wrapping_add(imm as u64);
        let data = cpu.mem.read32(address) as i32 as i64 as u64;
        cpu.regs.set(rd, data);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "LBU",
      opcode: 0b0000011,
      segments: funct3(0b100),
      run: |inst, cpu| {
        let I { imm, rs1, rd } = inst.i();
        let address = cpu.regs[rs1].wrapping_add(imm as u64);
        let data = cpu.mem.read8(address) as u64;
        cpu.regs.set(rd, data);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "LHU",
      opcode: 0b0000011,
      segments: funct3(0b101),
      run: |inst, cpu| {
        let I { imm, rs1, rd } = inst.i();
        let address = cpu.regs[rs1].wrapping_add(imm as u64);
        let data = cpu.mem.read16(address) as u64;
        cpu.regs.set(rd, data);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "SB",
      opcode: 0b0100011,
      segments: funct3(0b000),
      run: |inst, cpu| {
        let S { imm, rs2, rs1 } = inst.s();
        let address = cpu.regs[rs1].wrapping_add(imm as u64);
        cpu.mem.write8(address, cpu.regs[rs2] as u8);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "SH",
      opcode: 0b0100011,
      segments: funct3(0b001),
      run: |inst, cpu| {
        let S { imm, rs2, rs1 } = inst.s();
        let address = cpu.regs[rs1].wrapping_add(imm as u64);
        cpu.mem.write16(address, cpu.regs[rs2] as u16);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "SW",
      opcode: 0b0100011,
      segments: funct3(0b010),
      run: |inst, cpu| {
        let S { imm, rs2, rs1 } = inst.s();
        let address = cpu.regs[rs1].wrapping_add(imm as u64);
        cpu.mem.write32(address, cpu.regs[rs2] as u32);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "ADDI",
      opcode: 0b0010011,
      segments: funct3(0b000),
      run: |inst, cpu| {
        let I { imm, rs1, rd } = inst.i();
        cpu.regs.set(rd, cpu.regs[rs1].wrapping_add(imm as u64));
        InstructorResult::Success
      },
    },

    Instructor {
      name: "SLTI",
      opcode: 0b0010011,
      segments: funct3(0b010),
      run: |inst, cpu| {
        let I { imm, rs1, rd } = inst.i();
        cpu.regs.set(rd, if (cpu.regs[rs1] as i64) < imm { 1 } else { 0 });
        InstructorResult::Success
      },
    },

    Instructor {
      name: "SLTIU",
      opcode: 0b0010011,
      segments: funct3(0b011),
      run: |inst, cpu| {
        let I { imm, rs1, rd } = inst.i();
        cpu.regs.set(rd, if cpu.regs[rs1] < imm as u64 { 1 } else { 0 });
        InstructorResult::Success
      },
    },

    Instructor {
      name: "XORI",
      opcode: 0b0010011,
      segments: funct3(0b100),
      run: |inst, cpu| {
        let I { imm, rs1, rd } = inst.i();
        cpu.regs.set(rd, cpu.regs[rs1] ^ (imm as u64));
        InstructorResult::Success
      },
    },

    Instructor {
      name: "ORI",
      opcode: 0b0010011,
      segments: funct3(0b110),
      run: |inst, cpu| {
        let I { imm, rs1, rd } = inst.i();
        cpu.regs.set(rd, cpu.regs[rs1] | (imm as u64));
        InstructorResult::Success
      },
    },

    Instructor {
      name: "ANDI",
      opcode: 0b0010011,
      segments: funct3(0b111),
      run: |inst, cpu| {
        let I { imm, rs1, rd } = inst.i();
        cpu.regs.set(rd, cpu.regs[rs1] & (imm as u64));
        InstructorResult::Success
      },
    },

    Instructor {
      name: "ADD",
      opcode: 0b0110011,
      segments: funct37(0b000, 0b0000000),
      run: |inst, cpu| {
        let R { rs2, rs1, rd } = inst.r();
        cpu.regs.set(rd, cpu.regs[rs1].wrapping_add(cpu.regs[rs2]));
        InstructorResult::Success
      },
    },

    Instructor {
      name: "SUB",
      opcode: 0b0110011,
      segments: funct37(0b000, 0b0100000),
      run: |inst, cpu| {
        let R { rs2, rs1, rd } = inst.r();
        cpu.regs.set(rd, cpu.regs[rs1].wrapping_sub(cpu.regs[rs2]));
        InstructorResult::Success
      },
    },

    Instructor {
      name: "SLL",
      opcode: 0b0110011,
      segments: funct37(0b001, 0b0000000),
      run: |inst, cpu| {
        let R { rs2, rs1, rd } = inst.r();
        let shamt = cpu.regs[rs2] & 0b111111;
        cpu.regs.set(rd, cpu.regs[rs1] << shamt);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "SLT",
      opcode: 0b0110011,
      segments: funct37(0b010, 0b0000000),
      run: |inst, cpu| {
        let R { rs2, rs1, rd } = inst.r();
        cpu.regs.set(rd, if (cpu.regs[rs1] as i64) < (cpu.regs[rs2] as i64) { 1 } else { 0 });
        InstructorResult::Success
      },
    },

    Instructor {
      name: "SLTU",
      opcode: 0b0110011,
      segments: funct37(0b011, 0b0000000),
      run: |inst, cpu| {
        let R { rs2, rs1, rd } = inst.r();
        cpu.regs.set(rd, if cpu.regs[rs1] < cpu.regs[rs2] { 1 } else { 0 });
        InstructorResult::Success
      },
    },

    Instructor {
      name: "XOR",
      opcode: 0b0110011,
      segments: funct37(0b100, 0b0000000),
      run: |inst, cpu| {
        let R { rs2, rs1, rd } = inst.r();
        cpu.regs.set(rd, cpu.regs[rs1] ^ cpu.regs[rs2]);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "SRL",
      opcode: 0b0110011,
      segments: funct37(0b100, 0b0000000),
      run: |inst, cpu| {
        let R { rs2, rs1, rd } = inst.r();
        let shamt = cpu.regs[rs2] & 0b111111;
        cpu.regs.set(rd, cpu.regs[rs1] >> shamt);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "SRA",
      opcode: 0b0110011,
      segments: funct37(0b100, 0b0100000),
      run: |inst, cpu| {
        let R { rs2, rs1, rd } = inst.r();
        let shamt = cpu.regs[rs2] & 0b111111;
        cpu.regs.set(rd, ((cpu.regs[rs1] as i64) >> shamt) as u64);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "OR",
      opcode: 0b0110011,
      segments: funct37(0b110, 0b0000000),
      run: |inst, cpu| {
        let R { rs2, rs1, rd } = inst.r();
        cpu.regs.set(rd, cpu.regs[rs1] | cpu.regs[rs2]);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "AND",
      opcode: 0b0110011,
      segments: funct37(0b111, 0b0000000),
      run: |inst, cpu| {
        let R { rs2, rs1, rd } = inst.r();
        cpu.regs.set(rd, cpu.regs[rs1] & cpu.regs[rs2]);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "FENCE",
      opcode: 0b0001111,
      segments: funct3(0b000),
      run: |_inst, _cpu| {
        // do nothing
        InstructorResult::Success
      },
    },

    Instructor {
      name: "ECALL",
      opcode: 0b1110011,
      segments: vec![
        InstructionSegment { start: 7, end: 31, comp: 0b0000000000000000000000000 }
      ],
      run: |_inst, cpu| {
        // TODO
        let num = cpu.regs[17];
        let num = SYSCALL_MAP.get(&num).unwrap();
        let res = unsafe {
          libc::syscall(*num,
            cpu.regs[10],
            cpu.regs[11],
            cpu.regs[12],
            cpu.regs[13],
            cpu.regs[14],
            cpu.regs[15],
            cpu.regs[16])
        };
        cpu.regs.set(10, res as u64);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "EBREAK",
      opcode: 0b1110011,
      segments: vec![
        InstructionSegment { start: 7, end: 31, comp: 0b0000000000010000000000000 }
      ],
      run: |_inst, _cpu| {
        // TODO
        InstructorResult::Success
      },
    },

    Instructor {
      name: "LWU",
      opcode: 0b0000011,
      segments: funct3(0b110),
      run: |inst, cpu| {
        let I { imm, rs1, rd } = inst.i();
        let address = cpu.regs[rs1].wrapping_add(imm as u64);
        let data = cpu.mem.read32(address) as u64;
        cpu.regs.set(rd, data);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "LD",
      opcode: 0b0000011,
      segments: funct3(0b011),
      run: |inst, cpu| {
        let I { imm, rs1, rd } = inst.i();
        let address = cpu.regs[rs1].wrapping_add(imm as u64);
        let data = cpu.mem.read64(address);
        cpu.regs.set(rd, data);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "SD",
      opcode: 0b0100011,
      segments: funct3(0b011),
      run: |inst, cpu| {
        let S { imm, rs2, rs1 } = inst.s();
        let address = cpu.regs[rs1].wrapping_add(imm as u64);
        cpu.mem.write64(address, cpu.regs[rs2]);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "SLLI",
      opcode: 0b0010011,
      segments: vec![
        InstructionSegment { start: 12, end: 14, comp: 0b001 },
        InstructionSegment { start: 26, end: 31, comp: 0b000000 },
      ],
      run: |inst, cpu| {
        let shamt = inst >> 20 & 0b111111;
        let rs1 = (inst >> 15 & 0b11111) as usize;
        let rd = (inst >> 7 & 0b11111) as usize;
        cpu.regs.set(rd, cpu.regs[rs1] << shamt);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "SRLI",
      opcode: 0b0010011,
      segments: vec![
        InstructionSegment { start: 12, end: 14, comp: 0b101 },
        InstructionSegment { start: 26, end: 31, comp: 0b000000 },
      ],
      run: |inst, cpu| {
        let shamt = inst >> 20 & 0b111111;
        let rs1 = (inst >> 15 & 0b11111) as usize;
        let rd = (inst >> 7 & 0b11111) as usize;
        cpu.regs.set(rd, cpu.regs[rs1] >> shamt);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "SRAI",
      opcode: 0b0010011,
      segments: vec![
        InstructionSegment { start: 12, end: 14, comp: 0b101 },
        InstructionSegment { start: 26, end: 31, comp: 0b010000 },
      ],
      run: |inst, cpu| {
        let shamt = inst >> 20 & 0b111111;
        let rs1 = (inst >> 15 & 0b11111) as usize;
        let rd = (inst >> 7 & 0b11111) as usize;
        cpu.regs.set(rd, (cpu.regs[rs1] as i64 >> shamt) as u64);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "ADDIW",
      opcode: 0b0011011,
      segments: funct3(0b000),
      run: |inst, cpu| {
        let I { imm, rs1, rd } = inst.i();
        cpu.regs.set(rd, cpu.regs[rs1].wrapping_add(imm as u64) as i32 as i64 as u64);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "SLLIW",
      opcode: 0b0011011,
      segments: vec![
        InstructionSegment { start: 12, end: 14, comp: 0b001 },
        InstructionSegment { start: 26, end: 31, comp: 0b000000 },
      ],
      run: |inst, cpu| {
        let shamt = inst >> 20 & 0b111111;
        let rs1 = (inst >> 15 & 0b11111) as usize;
        let rd = (inst >> 7 & 0b11111) as usize;
        cpu.regs.set(rd, (cpu.regs[rs1] << shamt) as i32 as i64 as u64);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "SRLIW",
      opcode: 0b0011011,
      segments: vec![
        InstructionSegment { start: 12, end: 14, comp: 0b101 },
        InstructionSegment { start: 26, end: 31, comp: 0b000000 },
      ],
      run: |inst, cpu| {
        let shamt = inst >> 20 & 0b111111;
        let rs1 = (inst >> 15 & 0b11111) as usize;
        let rd = (inst >> 7 & 0b11111) as usize;
        cpu.regs.set(rd, (cpu.regs[rs1] >> shamt) as i32 as i64 as u64);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "SRAIW",
      opcode: 0b0011011,
      segments: vec![
        InstructionSegment { start: 12, end: 14, comp: 0b101 },
        InstructionSegment { start: 26, end: 31, comp: 0b010000 },
      ],
      run: |inst, cpu| {
        let shamt = inst >> 20 & 0b111111;
        let rs1 = (inst >> 15 & 0b11111) as usize;
        let rd = (inst >> 7 & 0b11111) as usize;
        cpu.regs.set(rd, (cpu.regs[rs1] as i64 >> shamt) as i32 as i64 as u64);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "ADDW",
      opcode: 0b0111011,
      segments: funct37(0b000, 0b0000000),
      run: |inst, cpu| {
        let R { rs2, rs1, rd } = inst.r();
        cpu.regs.set(rd, cpu.regs[rs1].wrapping_add(cpu.regs[rs2]) as i32 as i64 as u64);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "SUBW",
      opcode: 0b0111011,
      segments: funct37(0b000, 0b0100000),
      run: |inst, cpu| {
        let R { rs2, rs1, rd } = inst.r();
        cpu.regs.set(rd, cpu.regs[rs1].wrapping_sub(cpu.regs[rs2]) as i32 as i64 as u64);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "SLLW",
      opcode: 0b0111011,
      segments: funct37(0b001, 0b0000000),
      run: |inst, cpu| {
        let R { rs2, rs1, rd } = inst.r();
        let shamt = cpu.regs[rs2] & 0b111111;
        cpu.regs.set(rd, (cpu.regs[rs1] << shamt) as i32 as i64 as u64);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "SRLW",
      opcode: 0b0111011,
      segments: funct37(0b100, 0b0000000),
      run: |inst, cpu| {
        let R { rs2, rs1, rd } = inst.r();
        let shamt = cpu.regs[rs2] & 0b111111;
        cpu.regs.set(rd, (cpu.regs[rs1] >> shamt) as i32 as i64 as u64);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "SRAW",
      opcode: 0b0111011,
      segments: funct37(0b100, 0b0100000),
      run: |inst, cpu| {
        let R { rs2, rs1, rd } = inst.r();
        let shamt = cpu.regs[rs2] & 0b111111;
        cpu.regs.set(rd, ((cpu.regs[rs1] as i64) >> shamt) as i32 as i64 as u64);
        InstructorResult::Success
      },
    },
  ])
}