use std::num::FpCategory;

use crate::instructions::{Instructor, types::{funct3, I, InstructionParser, S, funct_rfp_rs3, RFPRS3, funct_rfp, RFP, funct_rfp_rs2, funct_rfp_rm, funct_rfp_rs2_rm}};

pub(crate) fn f() -> Vec<Instructor> {
  Vec::from([
    Instructor {
      name: "FLW",
      opcode: 0b0000111,
      segments: funct3(0b010),
      run: |inst, cpu| {
        let I { imm, rs1, rd } = inst.i();
        let address = cpu.regs[rs1].wrapping_add(imm as u64);
        cpu.fregs.set(rd, f32::from_bits(cpu.mem.read32(address)) as f64);
      },
    },

    Instructor {
      name: "FSW",
      opcode: 0b0100111,
      segments: funct3(0b010),
      run: |inst, cpu| {
        let S { imm, rs2, rs1 } = inst.s();
        let address = cpu.regs[rs1].wrapping_add(imm as u64);
        cpu.mem.write32(address, (cpu.fregs[rs2] as f32).to_bits());
      },
    },

    Instructor {
      name: "FMADD.S",
      opcode: 0b1000011,
      segments: funct_rfp_rs3(0b00),
      run: |inst, cpu| {
        let RFPRS3 { rs3, rs2, rs1, rd } = inst.rfp_rs3();
        let a = cpu.fregs[rs1] as f32;
        let b = cpu.fregs[rs2] as f32;
        let c = cpu.fregs[rs3] as f32;
        cpu.fregs.set(rd, a.mul_add(b, c) as f64);
      },
    },

    Instructor {
      name: "FMSUB.S",
      opcode: 0b1000111,
      segments: funct_rfp_rs3(0b00),
      run: |inst, cpu| {
        let RFPRS3 { rs3, rs2, rs1, rd } = inst.rfp_rs3();
        let a = cpu.fregs[rs1] as f32;
        let b = cpu.fregs[rs2] as f32;
        let c = cpu.fregs[rs3] as f32;
        cpu.fregs.set(rd, a.mul_add(b, -c) as f64);
      },
    },

    Instructor {
      name: "FNMSUB.S",
      opcode: 0b1001011,
      segments: funct_rfp_rs3(0b00),
      run: |inst, cpu| {
        let RFPRS3 { rs3, rs2, rs1, rd } = inst.rfp_rs3();
        let a = cpu.fregs[rs1] as f32;
        let b = cpu.fregs[rs2] as f32;
        let c = cpu.fregs[rs3] as f32;
        cpu.fregs.set(rd, (-a).mul_add(b, -c) as f64);
      },
    },

    Instructor {
      name: "FNMADD.S",
      opcode: 0b1001111,
      segments: funct_rfp_rs3(0b00),
      run: |inst, cpu| {
        let RFPRS3 { rs3, rs2, rs1, rd } = inst.rfp_rs3();
        let a = cpu.fregs[rs1] as f32;
        let b = cpu.fregs[rs2] as f32;
        let c = cpu.fregs[rs3] as f32;
        cpu.fregs.set(rd, (-a).mul_add(b, c) as f64);
      },
    },

    Instructor {
      name: "FADD.S",
      opcode: 0b1010011,
      segments: funct_rfp(0b00, 0b00000),
      run: |inst, cpu| {
        let RFP { rs2, rs1, rd } = inst.rfp();
        let a = cpu.fregs[rs1] as f32;
        let b = cpu.fregs[rs2] as f32;
        cpu.fregs.set(rd, (a + b) as f64);
      },
    },

    Instructor {
      name: "FSUB.S",
      opcode: 0b1010011,
      segments: funct_rfp(0b00, 0b00001),
      run: |inst, cpu| {
        let RFP { rs2, rs1, rd } = inst.rfp();
        let a = cpu.fregs[rs1] as f32;
        let b = cpu.fregs[rs2] as f32;
        cpu.fregs.set(rd, (a - b) as f64);
      },
    },

    Instructor {
      name: "FMUL.S",
      opcode: 0b1010011,
      segments: funct_rfp(0b00, 0b00010),
      run: |inst, cpu| {
        let RFP { rs2, rs1, rd } = inst.rfp();
        let a = cpu.fregs[rs1] as f32;
        let b = cpu.fregs[rs2] as f32;
        cpu.fregs.set(rd, (a * b) as f64);
      },
    },

    // TODO: fcsr dz
    Instructor {
      name: "FDIV.S",
      opcode: 0b1010011,
      segments: funct_rfp(0b00, 0b00011),
      run: |inst, cpu| {
        let RFP { rs2, rs1, rd } = inst.rfp();
        let a = cpu.fregs[rs1] as f32;
        let b = cpu.fregs[rs2] as f32;
        cpu.fregs.set(rd, (a / b) as f64);
      },
    },

    Instructor {
      name: "FSQRT.S",
      opcode: 0b1010011,
      segments: funct_rfp_rs2(0b00000, 0b00, 0b01011),
      run: |inst, cpu| {
        let RFP { rs2: _, rs1, rd } = inst.rfp();
        cpu.fregs.set(rd, (cpu.fregs[rs1] as f32).sqrt() as f64);
      },
    },

    Instructor {
      name: "FSGNJ.S",
      opcode: 0b1010011,
      segments: funct_rfp_rm(0b000, 0b00, 0b00010),
      run: |inst, cpu| {
        let RFP { rs2, rs1, rd } = inst.rfp();
        let a = cpu.fregs[rs1] as f32;
        let b = cpu.fregs[rs2] as f32;
        cpu.fregs.set(rd, a.copysign(b) as f64);
      },
    },

    Instructor {
      name: "FSGNJN.S",
      opcode: 0b1010011,
      segments: funct_rfp_rm(0b001, 0b00, 0b00010),
      run: |inst, cpu| {
        let RFP { rs2, rs1, rd } = inst.rfp();
        let a = cpu.fregs[rs1] as f32;
        let b = cpu.fregs[rs2] as f32;
        cpu.fregs.set(rd, a.copysign(-b) as f64);
      },
    },

    Instructor {
      name: "FSGNJX.S",
      opcode: 0b1010011,
      segments: funct_rfp_rm(0b010, 0b00, 0b00010),
      run: |inst, cpu| {
        let RFP { rs2, rs1, rd } = inst.rfp();
        let a = (cpu.fregs[rs1] as f32).to_bits();
        let b = (cpu.fregs[rs2] as f32).to_bits();
        cpu.fregs.set(rd, f32::from_bits(((a & 0x80000000) ^ (b & 0x80000000)) | (a & 0x7fffffff)) as f64);
      },
    },

    Instructor {
      name: "FMIN.S",
      opcode: 0b1010011,
      segments: funct_rfp_rm(0b000, 0b00, 0b00101),
      run: |inst, cpu| {
        let RFP { rs2, rs1, rd } = inst.rfp();
        let a = cpu.fregs[rs1] as f32;
        let b = cpu.fregs[rs2] as f32;
        cpu.fregs.set(rd, a.min(b) as f64);
      },
    },

    Instructor {
      name: "FMAX.S",
      opcode: 0b1010011,
      segments: funct_rfp_rm(0b001, 0b00, 0b00101),
      run: |inst, cpu| {
        let RFP { rs2, rs1, rd } = inst.rfp();
        let a = cpu.fregs[rs1] as f32;
        let b = cpu.fregs[rs2] as f32;
        cpu.fregs.set(rd, a.max(b) as f64);
      },
    },

    Instructor {
      name: "FCVT.W.S",
      opcode: 0b1010011,
      segments: funct_rfp_rs2(0b00000, 0b00, 0b11000),
      run: |inst, cpu| {
        let RFP { rs2: _, rs1, rd } = inst.rfp();
        cpu.regs.set(rd, cpu.fregs[rs1] as f32 as i32 as u64);
      },
    },

    Instructor {
      name: "FCVT.WU.S",
      opcode: 0b1010011,
      segments: funct_rfp_rs2(0b00001, 0b00, 0b11000),
      run: |inst, cpu| {
        let RFP { rs2: _, rs1, rd } = inst.rfp();
        cpu.regs.set(rd, cpu.fregs[rs1] as f32 as u32 as u64);
      },
    },

    Instructor {
      name: "FMV.X.W",
      opcode: 0b1010011,
      segments: funct_rfp_rs2_rm(0b000, 0b00000, 0b00, 0b11100),
      run: |inst, cpu| {
        let RFP { rs2: _, rs1, rd } = inst.rfp();
        let num = cpu.fregs[rs1];
        let higher = (((num as u64) << 32) - 1) << 32;
        cpu.regs.set(rd, ((num.to_bits()) as u32 as u64) | higher);
      },
    },

    Instructor {
      name: "FEQ.S",
      opcode: 0b1010011,
      segments: funct_rfp_rm(0b010, 0b00, 0b10100),
      run: |inst, cpu| {
        let RFP { rs2, rs1, rd } = inst.rfp();
        let a = cpu.fregs[rs1] as f32;
        let b = cpu.fregs[rs2] as f32;
        cpu.regs.set(rd, if a == b { 1 } else { 0 });
      },
    },

    Instructor {
      name: "FLT.S",
      opcode: 0b1010011,
      segments: funct_rfp_rm(0b001, 0b00, 0b10100),
      run: |inst, cpu| {
        let RFP { rs2, rs1, rd } = inst.rfp();
        let a = cpu.fregs[rs1] as f32;
        let b = cpu.fregs[rs2] as f32;
        cpu.regs.set(rd, if a < b { 1 } else { 0 });
      },
    },

    Instructor {
      name: "FLE.S",
      opcode: 0b1010011,
      segments: funct_rfp_rm(0b000, 0b00, 0b10100),
      run: |inst, cpu| {
        let RFP { rs2, rs1, rd } = inst.rfp();
        let a = cpu.fregs[rs1] as f32;
        let b = cpu.fregs[rs2] as f32;
        cpu.regs.set(rd, if a <= b { 1 } else { 0 });
      },
    },

    Instructor {
      name: "FCLASS.S",
      opcode: 0b1010011,
      segments: funct_rfp_rs2_rm(0b001, 0b00000, 0b00, 0b11100),
      run: |inst, cpu| {
        let RFP { rs2: _, rs1, rd } = inst.rfp();
        let num = cpu.fregs[rs1] as f32;
        let res = match num.classify() {
          FpCategory::Infinite => if num.is_sign_negative() { 0 } else { 7 },
          FpCategory::Normal => if num.is_sign_negative() { 1 } else { 6 },
          FpCategory::Subnormal => if num.is_sign_negative() { 2 } else { 5 },
          FpCategory::Zero => if num.is_sign_negative() { 3 } else { 4 },
          FpCategory::Nan => if num.is_sign_negative() { 4 } else { 3 },
        };
        cpu.regs.set(rd, res);
      },
    },

    Instructor {
      name: "FCVT.S.W",
      opcode: 0b1010011,
      segments: funct_rfp_rs2(0b00000, 0b00, 0b11010),
      run: |inst, cpu| {
        let RFP { rs2: _, rs1, rd } = inst.rfp();
        cpu.fregs.set(rd, cpu.regs[rs1] as i32 as f32 as f64);
      },
    },

    Instructor {
      name: "FCVT.S.WU",
      opcode: 0b1010011,
      segments: funct_rfp_rs2(0b00001, 0b00, 0b11010),
      run: |inst, cpu| {
        let RFP { rs2: _, rs1, rd } = inst.rfp();
        cpu.fregs.set(rd, cpu.regs[rs1] as u32 as f32 as f64);
      },
    },

    Instructor {
      name: "FMV.W.X",
      opcode: 0b1010011,
      segments: funct_rfp_rs2_rm(0b000, 0b00000, 0b00, 0b11110),
      run: |inst, cpu| {
        let RFP { rs2: _, rs1, rd } = inst.rfp();
        cpu.fregs.set(rd, f32::from_bits(cpu.regs[rs1] as u32) as f64);
      },
    },

    Instructor {
      name: "FCVT.L.S",
      opcode: 0b1010011,
      segments: funct_rfp_rs2(0b00010, 0b00, 0b11000),
      run: |inst, cpu| {
        let RFP { rs2: _, rs1, rd } = inst.rfp();
        cpu.regs.set(rd, cpu.fregs[rs1] as f32 as u64);
      },
    },

    Instructor {
      name: "FCVT.LU.S",
      opcode: 0b1010011,
      segments: funct_rfp_rs2(0b00011, 0b00, 0b11000),
      run: |inst, cpu| {
        let RFP { rs2: _, rs1, rd } = inst.rfp();
        cpu.regs.set(rd, cpu.fregs[rs1] as f32 as u64);
      },
    },

    Instructor {
      name: "FCVT.S.L",
      opcode: 0b1010011,
      segments: funct_rfp_rs2(0b00010, 0b00, 0b11010),
      run: |inst, cpu| {
        let RFP { rs2: _, rs1, rd } = inst.rfp();
        cpu.fregs.set(rd, cpu.regs[rs1] as i32 as f32 as f64);
      },
    },

    Instructor {
      name: "FCVT.S.LU",
      opcode: 0b1010011,
      segments: funct_rfp_rs2(0b00011, 0b00, 0b11010),
      run: |inst, cpu| {
        let RFP { rs2: _, rs1, rd } = inst.rfp();
        cpu.fregs.set(rd, cpu.regs[rs1] as f32 as f64);
      },
    },
  ])
}