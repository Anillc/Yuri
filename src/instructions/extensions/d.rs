use std::num::FpCategory;

use crate::instructions::{Instructor, InstructorResult};

use super::{funct3, I, InstructionParser, S, funct_rfp_rs3, RFPRS3, funct_rfp, RFP, funct_rfp_rs2, funct_rfp_rm, funct_rfp_rs2_rm};

pub(crate) fn d() -> Vec<Instructor> {
  Vec::from([
    Instructor {
      name: "FLD",
      opcode: 0b0000111,
      segments: funct3(0b011),
      run: |inst, cpu| {
        let I { imm, rs1, rd } = inst.i();
        let address = cpu.regs[rs1].wrapping_add(imm as u64);
        cpu.fregs.set(rd, f64::from_bits(cpu.mem.read64(address)));
        InstructorResult::Success
      },
    },

    Instructor {
      name: "FSD",
      opcode: 0b0100111,
      segments: funct3(0b011),
      run: |inst, cpu| {
        let S { imm, rs2, rs1 } = inst.s();
        let address = cpu.regs[rs1].wrapping_add(imm as u64);
        cpu.mem.write64(address, cpu.fregs[rs2].to_bits());
        InstructorResult::Success
      },
    },

    Instructor {
      name: "FMADD.D",
      opcode: 0b1000011,
      segments: funct_rfp_rs3(0b01),
      run: |inst, cpu| {
        let RFPRS3 { rs3, rs2, rs1, rd } = inst.rfp_rs3();
        let a = cpu.fregs[rs1];
        let b = cpu.fregs[rs2];
        let c = cpu.fregs[rs3];
        cpu.fregs.set(rd, a.mul_add(b, c));
        InstructorResult::Success
      },
    },

    Instructor {
      name: "FMSUB.D",
      opcode: 0b1000111,
      segments: funct_rfp_rs3(0b01),
      run: |inst, cpu| {
        let RFPRS3 { rs3, rs2, rs1, rd } = inst.rfp_rs3();
        let a = cpu.fregs[rs1];
        let b = cpu.fregs[rs2];
        let c = cpu.fregs[rs3];
        cpu.fregs.set(rd, a.mul_add(b, -c));
        InstructorResult::Success
      },
    },

    Instructor {
      name: "FNMSUB.D",
      opcode: 0b1001011,
      segments: funct_rfp_rs3(0b01),
      run: |inst, cpu| {
        let RFPRS3 { rs3, rs2, rs1, rd } = inst.rfp_rs3();
        let a = cpu.fregs[rs1];
        let b = cpu.fregs[rs2];
        let c = cpu.fregs[rs3];
        cpu.fregs.set(rd, (-a).mul_add(b, -c));
        InstructorResult::Success
      },
    },

    Instructor {
      name: "FNMADD.D",
      opcode: 0b1001111,
      segments: funct_rfp_rs3(0b01),
      run: |inst, cpu| {
        let RFPRS3 { rs3, rs2, rs1, rd } = inst.rfp_rs3();
        let a = cpu.fregs[rs1];
        let b = cpu.fregs[rs2];
        let c = cpu.fregs[rs3];
        cpu.fregs.set(rd, (-a).mul_add(b, c));
        InstructorResult::Success
      },
    },

    Instructor {
      name: "FADD.D",
      opcode: 0b1010011,
      segments: funct_rfp(0b01, 0b00000),
      run: |inst, cpu| {
        let RFP { rs2, rs1, rd } = inst.rfp();
        let a = cpu.fregs[rs1];
        let b = cpu.fregs[rs2];
        cpu.fregs.set(rd, a + b);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "FSUB.D",
      opcode: 0b1010011,
      segments: funct_rfp(0b01, 0b00001),
      run: |inst, cpu| {
        let RFP { rs2, rs1, rd } = inst.rfp();
        let a = cpu.fregs[rs1];
        let b = cpu.fregs[rs2];
        cpu.fregs.set(rd, a - b);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "FMUL.D",
      opcode: 0b1010011,
      segments: funct_rfp(0b01, 0b00010),
      run: |inst, cpu| {
        let RFP { rs2, rs1, rd } = inst.rfp();
        let a = cpu.fregs[rs1];
        let b = cpu.fregs[rs2];
        cpu.fregs.set(rd, a * b);
        InstructorResult::Success
      },
    },

    // TODO: fcsr dz
    Instructor {
      name: "FDIV.D",
      opcode: 0b1010011,
      segments: funct_rfp(0b01, 0b00011),
      run: |inst, cpu| {
        let RFP { rs2, rs1, rd } = inst.rfp();
        let a = cpu.fregs[rs1];
        let b = cpu.fregs[rs2];
        cpu.fregs.set(rd, a / b);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "FSQRT.D",
      opcode: 0b1010011,
      segments: funct_rfp_rs2(0b00000, 0b01, 0b01011),
      run: |inst, cpu| {
        let RFP { rs2: _, rs1, rd } = inst.rfp();
        cpu.fregs.set(rd, cpu.fregs[rs1].sqrt());
        InstructorResult::Success
      },
    },

    Instructor {
      name: "FSGNJ.D",
      opcode: 0b1010011,
      segments: funct_rfp_rm(0b000, 0b01, 0b00010),
      run: |inst, cpu| {
        let RFP { rs2, rs1, rd } = inst.rfp();
        let a = cpu.fregs[rs1];
        let b = cpu.fregs[rs2];
        cpu.fregs.set(rd, a.copysign(b));
        InstructorResult::Success
      },
    },

    Instructor {
      name: "FSGNJN.D",
      opcode: 0b1010011,
      segments: funct_rfp_rm(0b001, 0b01, 0b00010),
      run: |inst, cpu| {
        let RFP { rs2, rs1, rd } = inst.rfp();
        let a = cpu.fregs[rs1];
        let b = cpu.fregs[rs2];
        cpu.fregs.set(rd, a.copysign(-b));
        InstructorResult::Success
      },
    },

    Instructor {
      name: "FSGNJX.D",
      opcode: 0b1010011,
      segments: funct_rfp_rm(0b010, 0b01, 0b00010),
      run: |inst, cpu| {
        let RFP { rs2, rs1, rd } = inst.rfp();
        let a = cpu.fregs[rs1].to_bits();
        let b = cpu.fregs[rs2].to_bits();
        cpu.fregs.set(rd, f64::from_bits(((a & 0x80000000) ^ (b & 0x80000000)) | (a & 0x7fffffff)));
        InstructorResult::Success
      },
    },

    Instructor {
      name: "FMIN.D",
      opcode: 0b1010011,
      segments: funct_rfp_rm(0b000, 0b01, 0b00101),
      run: |inst, cpu| {
        let RFP { rs2, rs1, rd } = inst.rfp();
        let a = cpu.fregs[rs1];
        let b = cpu.fregs[rs2];
        cpu.fregs.set(rd, a.min(b));
        InstructorResult::Success
      },
    },

    Instructor {
      name: "FMAX.D",
      opcode: 0b1010011,
      segments: funct_rfp_rm(0b001, 0b01, 0b00101),
      run: |inst, cpu| {
        let RFP { rs2, rs1, rd } = inst.rfp();
        let a = cpu.fregs[rs1];
        let b = cpu.fregs[rs2];
        cpu.fregs.set(rd, a.max(b));
        InstructorResult::Success
      },
    },

    Instructor {
      name: "FCVT.S.D",
      opcode: 0b1010011,
      segments: funct_rfp_rs2(0b00001, 0b00, 0b01000),
      run: |inst, cpu| {
        let RFP { rs2: _, rs1, rd } = inst.rfp();
        cpu.fregs.set(rd, cpu.fregs[rs1]);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "FCVT.D.S",
      opcode: 0b1010011,
      segments: funct_rfp_rs2(0b00000, 0b01, 0b01000),
      run: |inst, cpu| {
        let RFP { rs2: _, rs1, rd } = inst.rfp();
        cpu.fregs.set(rd, cpu.fregs[rs1] as f32 as f64);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "FEQ.D",
      opcode: 0b1010011,
      segments: funct_rfp_rm(0b010, 0b01, 0b10100),
      run: |inst, cpu| {
        let RFP { rs2, rs1, rd } = inst.rfp();
        let a = cpu.fregs[rs1];
        let b = cpu.fregs[rs2];
        cpu.regs.set(rd, if a == b { 1 } else { 0 });
        InstructorResult::Success
      },
    },

    Instructor {
      name: "FLT.D",
      opcode: 0b1010011,
      segments: funct_rfp_rm(0b001, 0b01, 0b10100),
      run: |inst, cpu| {
        let RFP { rs2, rs1, rd } = inst.rfp();
        let a = cpu.fregs[rs1];
        let b = cpu.fregs[rs2];
        cpu.regs.set(rd, if a < b { 1 } else { 0 });
        InstructorResult::Success
      },
    },

    Instructor {
      name: "FLE.D",
      opcode: 0b1010011,
      segments: funct_rfp_rm(0b000, 0b01, 0b10100),
      run: |inst, cpu| {
        let RFP { rs2, rs1, rd } = inst.rfp();
        let a = cpu.fregs[rs1];
        let b = cpu.fregs[rs2];
        cpu.regs.set(rd, if a <= b { 1 } else { 0 });
        InstructorResult::Success
      },
    },

    Instructor {
      name: "FCLASS.D",
      opcode: 0b1010011,
      segments: funct_rfp_rs2_rm(0b001, 0b00000, 0b01, 0b11100),
      run: |inst, cpu| {
        let RFP { rs2: _, rs1, rd } = inst.rfp();
        let num = cpu.fregs[rs1];
        let res = match num.classify() {
          FpCategory::Infinite => if num.is_sign_negative() { 0 } else { 7 },
          FpCategory::Normal => if num.is_sign_negative() { 1 } else { 6 },
          FpCategory::Subnormal => if num.is_sign_negative() { 2 } else { 5 },
          FpCategory::Zero => if num.is_sign_negative() { 3 } else { 4 },
          FpCategory::Nan => if num.is_sign_negative() { 4 } else { 3 },
        };
        cpu.regs.set(rd, res);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "FCVT.W.D",
      opcode: 0b1010011,
      segments: funct_rfp_rs2(0b00000, 0b01, 0b11000),
      run: |inst, cpu| {
        let RFP { rs2: _, rs1, rd } = inst.rfp();
        cpu.regs.set(rd, cpu.fregs[rs1] as i64 as u64);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "FCVT.WU.D",
      opcode: 0b1010011,
      segments: funct_rfp_rs2(0b00001, 0b01, 0b11000),
      run: |inst, cpu| {
        let RFP { rs2: _, rs1, rd } = inst.rfp();
        cpu.regs.set(rd, cpu.fregs[rs1] as u64);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "FCVT.D.W",
      opcode: 0b1010011,
      segments: funct_rfp_rs2(0b00000, 0b01, 0b11010),
      run: |inst, cpu| {
        let RFP { rs2: _, rs1, rd } = inst.rfp();
        cpu.fregs.set(rd, cpu.regs[rs1] as i32 as f64);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "FCVT.D.WU",
      opcode: 0b1010011,
      segments: funct_rfp_rs2(0b00001, 0b01, 0b11010),
      run: |inst, cpu| {
        let RFP { rs2: _, rs1, rd } = inst.rfp();
        cpu.fregs.set(rd, cpu.regs[rs1] as u32 as f64);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "FCVT.L.D",
      opcode: 0b1010011,
      segments: funct_rfp_rs2(0b00010, 0b01, 0b11000),
      run: |inst, cpu| {
        let RFP { rs2: _, rs1, rd } = inst.rfp();
        cpu.regs.set(rd, cpu.fregs[rs1] as u64);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "FCVT.LU.D",
      opcode: 0b1010011,
      segments: funct_rfp_rs2(0b00011, 0b01, 0b11000),
      run: |inst, cpu| {
        let RFP { rs2: _, rs1, rd } = inst.rfp();
        cpu.regs.set(rd, cpu.fregs[rs1] as u64);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "FMV.X.D",
      opcode: 0b1010011,
      segments: funct_rfp_rs2_rm(0b000, 0b00000, 0b01, 0b11100),
      run: |inst, cpu| {
        let RFP { rs2: _, rs1, rd } = inst.rfp();
        cpu.regs.set(rd, cpu.fregs[rs1].to_bits());
        InstructorResult::Success
      },
    },

    Instructor {
      name: "FCVT.D.L",
      opcode: 0b1010011,
      segments: funct_rfp_rs2(0b00010, 0b01, 0b11010),
      run: |inst, cpu| {
        let RFP { rs2: _, rs1, rd } = inst.rfp();
        cpu.fregs.set(rd, cpu.regs[rs1] as i64 as f64);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "FCVT.D.LU",
      opcode: 0b1010011,
      segments: funct_rfp_rs2(0b00011, 0b01, 0b11010),
      run: |inst, cpu| {
        let RFP { rs2: _, rs1, rd } = inst.rfp();
        cpu.fregs.set(rd, cpu.regs[rs1] as f64);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "FMV.D.X",
      opcode: 0b1010011,
      segments: funct_rfp_rs2_rm(0b000, 0b00000, 0b01, 0b11110),
      run: |inst, cpu| {
        let RFP { rs2: _, rs1, rd } = inst.rfp();
        cpu.fregs.set(rd, f64::from_bits(cpu.regs[rs1]));
        InstructorResult::Success
      },
    },
  ])
}