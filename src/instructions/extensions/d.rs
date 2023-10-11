use softfloat_wrapper::{F64, Float, F32};

use crate::{instructions::Instructor, utils::{round_mode, classify, Boxed, FloatFlags}};

use super::{funct3, I, InstructionParser, S, funct_rfp_rs3, RFPRS3, funct_rfp, RFP, funct_rfp_rs2, funct_rfp_rm, funct_rfp_rs2_rm};

const NANBOX: u64 = ((-1i64) as u64) << 32;

pub(crate) fn d() -> Vec<Instructor> {
  Vec::from([
    Instructor {
      name: "FLD",
      opcode: 0b0000111,
      segments: funct3(0b011),
      run: |inst, _len, cpu| {
        let I { imm, rs1, rd } = inst.i();
        let address = cpu.regs[rs1].wrapping_add(imm as u64);
        cpu.fregs.set(rd, cpu.mem.read64(address));
        Ok(())
      },
    },

    Instructor {
      name: "FSD",
      opcode: 0b0100111,
      segments: funct3(0b011),
      run: |inst, _len, cpu| {
        let S { imm, rs2, rs1 } = inst.s();
        let address = cpu.regs[rs1].wrapping_add(imm as u64);
        cpu.mem.write64(address, cpu.fregs[rs2]);
        Ok(())
      },
    },

    Instructor {
      name: "FMADD.D",
      opcode: 0b1000011,
      segments: funct_rfp_rs3(0b01),
      run: |inst, _len, cpu| {
        let RFPRS3 { rs3, rs2, rs1, rm, rd } = inst.rfp_rs3();
        let flags = FloatFlags::new();
        let rm = round_mode(rm, cpu)?;
        let a = F64::from_bits(cpu.fregs[rs1]);
        let b = F64::from_bits(cpu.fregs[rs2]);
        let c = F64::from_bits(cpu.fregs[rs3]);
        cpu.fregs.set(rd, a.fused_mul_add(b, c, rm).to_bits());
        flags.write(&mut cpu.csr, false);
        Ok(())
      },
    },

    Instructor {
      name: "FMSUB.D",
      opcode: 0b1000111,
      segments: funct_rfp_rs3(0b01),
      run: |inst, _len, cpu| {
        let RFPRS3 { rs3, rs2, rs1, rm, rd } = inst.rfp_rs3();
        let flags = FloatFlags::new();
        let rm = round_mode(rm, cpu)?;
        let a = F64::from_bits(cpu.fregs[rs1]);
        let b = F64::from_bits(cpu.fregs[rs2]);
        let c = F64::from_bits(cpu.fregs[rs3]);
        cpu.fregs.set(rd, a.fused_mul_add(b, c.neg(), rm).to_bits());
        flags.write(&mut cpu.csr, false);
        Ok(())
      },
    },

    Instructor {
      name: "FNMSUB.D",
      opcode: 0b1001011,
      segments: funct_rfp_rs3(0b01),
      run: |inst, _len, cpu| {
        let RFPRS3 { rs3, rs2, rs1, rm, rd } = inst.rfp_rs3();
        let flags = FloatFlags::new();
        let rm = round_mode(rm, cpu)?;
        let a = F64::from_bits(cpu.fregs[rs1]);
        let b = F64::from_bits(cpu.fregs[rs2]);
        let c = F64::from_bits(cpu.fregs[rs3]);
        cpu.fregs.set(rd, a.fused_mul_add(b, c.neg(), rm).neg().to_bits());
        flags.write(&mut cpu.csr, false);
        Ok(())
      },
    },

    Instructor {
      name: "FNMADD.D",
      opcode: 0b1001111,
      segments: funct_rfp_rs3(0b01),
      run: |inst, _len, cpu| {
        let RFPRS3 { rs3, rs2, rs1, rm, rd } = inst.rfp_rs3();
        let flags = FloatFlags::new();
        let rm = round_mode(rm, cpu)?;
        let a = F64::from_bits(cpu.fregs[rs1]);
        let b = F64::from_bits(cpu.fregs[rs2]);
        let c = F64::from_bits(cpu.fregs[rs3]);
        cpu.fregs.set(rd, a.neg().fused_mul_add(b, c.neg(), rm).to_bits());
        flags.write(&mut cpu.csr, false);
        Ok(())
      },
    },

    Instructor {
      name: "FADD.D",
      opcode: 0b1010011,
      segments: funct_rfp(0b01, 0b00000),
      run: |inst, _len, cpu| {
        let RFP { rs2, rs1, rm, rd } = inst.rfp();
        let flags = FloatFlags::new();
        let rm = round_mode(rm, cpu)?;
        let a = F64::from_bits(cpu.fregs[rs1]);
        let b = F64::from_bits(cpu.fregs[rs2]);
        cpu.fregs.set(rd, a.add(b, rm).to_bits());
        flags.write(&mut cpu.csr, false);
        Ok(())
      },
    },

    Instructor {
      name: "FSUB.D",
      opcode: 0b1010011,
      segments: funct_rfp(0b01, 0b00001),
      run: |inst, _len, cpu| {
        let RFP { rs2, rs1, rm, rd } = inst.rfp();
        let flags = FloatFlags::new();
        let rm = round_mode(rm, cpu)?;
        let a = F64::from_bits(cpu.fregs[rs1]);
        let b = F64::from_bits(cpu.fregs[rs2]);
        cpu.fregs.set(rd, a.sub(b, rm).to_bits());
        flags.write(&mut cpu.csr, false);
        Ok(())
      },
    },

    Instructor {
      name: "FMUL.D",
      opcode: 0b1010011,
      segments: funct_rfp(0b01, 0b00010),
      run: |inst, _len, cpu| {
        let RFP { rs2, rs1, rm, rd } = inst.rfp();
        let flags = FloatFlags::new();
        let rm = round_mode(rm, cpu)?;
        let a = F64::from_bits(cpu.fregs[rs1]);
        let b = F64::from_bits(cpu.fregs[rs2]);
        cpu.fregs.set(rd, a.mul(b, rm).to_bits());
        flags.write(&mut cpu.csr, false);
        Ok(())
      },
    },

    Instructor {
      name: "FDIV.D",
      opcode: 0b1010011,
      segments: funct_rfp(0b01, 0b00011),
      run: |inst, _len, cpu| {
        let RFP { rs2, rs1, rm, rd } = inst.rfp();
        let flags = FloatFlags::new();
        let rm = round_mode(rm, cpu)?;
        let a = F64::from_bits(cpu.fregs[rs1]);
        let b = F64::from_bits(cpu.fregs[rs2]);
        cpu.fregs.set(rd, a.div(b, rm).to_bits());
        flags.write(&mut cpu.csr, b.is_zero());
        Ok(())
      },
    },

    Instructor {
      name: "FSQRT.D",
      opcode: 0b1010011,
      segments: funct_rfp_rs2(0b00000, 0b01, 0b01011),
      run: |inst, _len, cpu| {
        let RFP { rs2: _, rs1, rm, rd } = inst.rfp();
        let flags = FloatFlags::new();
        let rm = round_mode(rm, cpu)?;
        let num = F64::from_bits(cpu.fregs[rs1]);
        cpu.fregs.set(rd, num.sqrt(rm).to_bits());
        flags.write(&mut cpu.csr, false);
        Ok(())
      },
    },

    Instructor {
      name: "FSGNJ.D",
      opcode: 0b1010011,
      segments: funct_rfp_rm(0b000, 0b01, 0b00100),
      run: |inst, _len, cpu| {
        let RFP { rs2, rs1, rm: _, rd } = inst.rfp();
        let mut a = F64::from_bits(cpu.fregs[rs1]);
        let b = F64::from_bits(cpu.fregs[rs2]);
        a.set_sign(b.sign());
        cpu.fregs.set(rd, a.to_bits());
        Ok(())
      },
    },

    Instructor {
      name: "FSGNJN.D",
      opcode: 0b1010011,
      segments: funct_rfp_rm(0b001, 0b01, 0b00100),
      run: |inst, _len, cpu| {
        let RFP { rs2, rs1, rm: _, rd } = inst.rfp();
        let mut a = F64::from_bits(cpu.fregs[rs1]);
        let b = F64::from_bits(cpu.fregs[rs2]);
        a.set_sign(!b.sign());
        cpu.fregs.set(rd, a.to_bits());
        Ok(())
      },
    },

    Instructor {
      name: "FSGNJX.D",
      opcode: 0b1010011,
      segments: funct_rfp_rm(0b010, 0b01, 0b00100),
      run: |inst, _len, cpu| {
        let RFP { rs2, rs1, rm: _, rd } = inst.rfp();
        let mut a = F64::from_bits(cpu.fregs[rs1]);
        let b = F64::from_bits(cpu.fregs[rs2]);
        a.set_sign(a.sign() ^ b.sign());
        cpu.fregs.set(rd, a.to_bits());
        Ok(())
      },
    },

    Instructor {
      name: "FMIN.D",
      opcode: 0b1010011,
      segments: funct_rfp_rm(0b000, 0b01, 0b00101),
      run: |inst, _len, cpu| {
        let RFP { rs2, rs1, rm: _, rd } = inst.rfp();
        let flags = FloatFlags::new();
        let a = F64::from_bits(cpu.fregs[rs1]);
        let b = F64::from_bits(cpu.fregs[rs2]);
        let less = a.lt_quiet(b) || a.eq(b) && a.sign() != 0;
        let res = if a.is_nan() && b.is_nan() {
          F64::quiet_nan()
        } else if less || b.is_nan() {
          a
        } else {
          b
        };
        cpu.fregs.set(rd, res.to_bits());
        flags.write(&mut cpu.csr, false);
        Ok(())
      },
    },

    Instructor {
      name: "FMAX.D",
      opcode: 0b1010011,
      segments: funct_rfp_rm(0b001, 0b01, 0b00101),
      run: |inst, _len, cpu| {
        let RFP { rs2, rs1, rm: _, rd } = inst.rfp();
        let flags = FloatFlags::new();
        let a = F64::from_bits(cpu.fregs[rs1]);
        let b = F64::from_bits(cpu.fregs[rs2]);
        let greater = b.lt_quiet(a) || b.eq(a) && b.sign() != 0;
        let res = if a.is_nan() && b.is_nan() {
          F64::quiet_nan()
        } else if greater || b.is_nan() {
          a
        } else {
          b
        };
        cpu.fregs.set(rd, res.to_bits());
        flags.write(&mut cpu.csr, false);
        Ok(())
      },
    },

    Instructor {
      name: "FCVT.S.D",
      opcode: 0b1010011,
      segments: funct_rfp_rs2(0b00001, 0b00, 0b01000),
      run: |inst, _len, cpu| {
        let RFP { rs2: _, rs1, rm, rd } = inst.rfp();
        let flags = FloatFlags::new();
        let rm = round_mode(rm, cpu)?;
        let num = F64::from_bits(cpu.fregs[rs1]);
        cpu.fregs.set(rd, num.to_f32(rm).to_bits() as u64 | NANBOX);
        flags.write(&mut cpu.csr, false);
        Ok(())
      },
    },

    Instructor {
      name: "FCVT.D.S",
      opcode: 0b1010011,
      segments: funct_rfp_rs2(0b00000, 0b01, 0b01000),
      run: |inst, _len, cpu| {
        let RFP { rs2: _, rs1, rm, rd } = inst.rfp();
        let flags = FloatFlags::new();
        let rm = round_mode(rm, cpu)?;
        let num = F32::from_bits(cpu.fregs[rs1].unbox());
        cpu.fregs.set(rd, num.to_f64(rm).to_bits());
        flags.write(&mut cpu.csr, false);
        Ok(())
      },
    },

    Instructor {
      name: "FEQ.D",
      opcode: 0b1010011,
      segments: funct_rfp_rm(0b010, 0b01, 0b10100),
      run: |inst, _len, cpu| {
        let RFP { rs2, rs1, rm: _, rd } = inst.rfp();
        let flags = FloatFlags::new();
        let a = F64::from_bits(cpu.fregs[rs1]);
        let b = F64::from_bits(cpu.fregs[rs2]);
        cpu.regs.set(rd, if a.eq(b) { 1 } else { 0 });
        flags.write(&mut cpu.csr, false);
        Ok(())
      },
    },

    Instructor {
      name: "FLT.D",
      opcode: 0b1010011,
      segments: funct_rfp_rm(0b001, 0b01, 0b10100),
      run: |inst, _len, cpu| {
        let RFP { rs2, rs1, rm: _, rd } = inst.rfp();
        let flags = FloatFlags::new();
        let a = F64::from_bits(cpu.fregs[rs1]);
        let b = F64::from_bits(cpu.fregs[rs2]);
        cpu.regs.set(rd, if a.lt(b) { 1 } else { 0 });
        flags.write(&mut cpu.csr, false);
        Ok(())
      },
    },

    Instructor {
      name: "FLE.D",
      opcode: 0b1010011,
      segments: funct_rfp_rm(0b000, 0b01, 0b10100),
      run: |inst, _len, cpu| {
        let RFP { rs2, rs1, rm: _, rd } = inst.rfp();
        let flags = FloatFlags::new();
        let a = F64::from_bits(cpu.fregs[rs1]);
        let b = F64::from_bits(cpu.fregs[rs2]);
        cpu.regs.set(rd, if a.le(b) { 1 } else { 0 });
        flags.write(&mut cpu.csr, false);
        Ok(())
      },
    },

    Instructor {
      name: "FCLASS.D",
      opcode: 0b1010011,
      segments: funct_rfp_rs2_rm(0b001, 0b00000, 0b01, 0b11100),
      run: |inst, _len, cpu| {
        let RFP { rs2: _, rs1, rm: _, rd } = inst.rfp();
        let num = F64::from_bits(cpu.fregs[rs1]);
        cpu.regs.set(rd, classify(num));
        Ok(())
      },
    },

    Instructor {
      name: "FCVT.W.D",
      opcode: 0b1010011,
      segments: funct_rfp_rs2(0b00000, 0b01, 0b11000),
      run: |inst, _len, cpu| {
        let RFP { rs2: _, rs1, rm, rd } = inst.rfp();
        let flags = FloatFlags::new();
        let rm = round_mode(rm, cpu)?;
        let num = F64::from_bits(cpu.fregs[rs1]);
        cpu.regs.set(rd, num.to_i32(rm, true) as i64 as u64);
        flags.write(&mut cpu.csr, false);
        Ok(())
      },
    },

    Instructor {
      name: "FCVT.WU.D",
      opcode: 0b1010011,
      segments: funct_rfp_rs2(0b00001, 0b01, 0b11000),
      run: |inst, _len, cpu| {
        let RFP { rs2: _, rs1, rm, rd } = inst.rfp();
        let flags = FloatFlags::new();
        let rm = round_mode(rm, cpu)?;
        let num = F64::from_bits(cpu.fregs[rs1]);
        cpu.regs.set(rd, num.to_u32(rm, true) as i32 as i64 as u64);
        flags.write(&mut cpu.csr, false);
        Ok(())
      },
    },

    Instructor {
      name: "FCVT.D.W",
      opcode: 0b1010011,
      segments: funct_rfp_rs2(0b00000, 0b01, 0b11010),
      run: |inst, _len, cpu| {
        let RFP { rs2: _, rs1, rm, rd } = inst.rfp();
        let flags = FloatFlags::new();
        let rm = round_mode(rm, cpu)?;
        let num = F64::from_i32(cpu.regs[rs1] as i32, rm);
        cpu.fregs.set(rd, num.to_bits());
        flags.write(&mut cpu.csr, false);
        Ok(())
      },
    },

    Instructor {
      name: "FCVT.D.WU",
      opcode: 0b1010011,
      segments: funct_rfp_rs2(0b00001, 0b01, 0b11010),
      run: |inst, _len, cpu| {
        let RFP { rs2: _, rs1, rm, rd } = inst.rfp();
        let flags = FloatFlags::new();
        let rm = round_mode(rm, cpu)?;
        let num = F64::from_u32(cpu.regs[rs1] as u32, rm);
        cpu.fregs.set(rd, num.to_bits());
        flags.write(&mut cpu.csr, false);
        Ok(())
      },
    },

    Instructor {
      name: "FCVT.L.D",
      opcode: 0b1010011,
      segments: funct_rfp_rs2(0b00010, 0b01, 0b11000),
      run: |inst, _len, cpu| {
        let RFP { rs2: _, rs1, rm, rd } = inst.rfp();
        let flags = FloatFlags::new();
        let rm = round_mode(rm, cpu)?;
        let num = F64::from_bits(cpu.fregs[rs1]);
        cpu.regs.set(rd, num.to_i64(rm, true) as u64);
        flags.write(&mut cpu.csr, false);
        Ok(())
      },
    },

    Instructor {
      name: "FCVT.LU.D",
      opcode: 0b1010011,
      segments: funct_rfp_rs2(0b00011, 0b01, 0b11000),
      run: |inst, _len, cpu| {
        let RFP { rs2: _, rs1, rm, rd } = inst.rfp();
        let flags = FloatFlags::new();
        let rm = round_mode(rm, cpu)?;
        let num = F64::from_bits(cpu.fregs[rs1]);
        cpu.regs.set(rd, num.to_u64(rm, true));
        flags.write(&mut cpu.csr, false);
        Ok(())
      },
    },

    Instructor {
      name: "FMV.X.D",
      opcode: 0b1010011,
      segments: funct_rfp_rs2_rm(0b000, 0b00000, 0b01, 0b11100),
      run: |inst, _len, cpu| {
        let RFP { rs2: _, rs1, rm: _, rd } = inst.rfp();
        cpu.regs.set(rd, cpu.fregs[rs1]);
        Ok(())
      },
    },

    Instructor {
      name: "FCVT.D.L",
      opcode: 0b1010011,
      segments: funct_rfp_rs2(0b00010, 0b01, 0b11010),
      run: |inst, _len, cpu| {
        let RFP { rs2: _, rs1, rm, rd } = inst.rfp();
        let flags = FloatFlags::new();
        let rm = round_mode(rm, cpu)?;
        let num = F64::from_i64(cpu.regs[rs1] as i64, rm);
        cpu.fregs.set(rd, num.to_bits());
        flags.write(&mut cpu.csr, false);
        Ok(())
      },
    },

    Instructor {
      name: "FCVT.D.LU",
      opcode: 0b1010011,
      segments: funct_rfp_rs2(0b00011, 0b01, 0b11010),
      run: |inst, _len, cpu| {
        let RFP { rs2: _, rs1, rm, rd } = inst.rfp();
        let flags = FloatFlags::new();
        let rm = round_mode(rm, cpu)?;
        let num = F64::from_u64(cpu.regs[rs1], rm);
        cpu.fregs.set(rd, num.to_bits());
        flags.write(&mut cpu.csr, false);
        Ok(())
      },
    },

    Instructor {
      name: "FMV.D.X",
      opcode: 0b1010011,
      segments: funct_rfp_rs2_rm(0b000, 0b00000, 0b01, 0b11110),
      run: |inst, _len, cpu| {
        let RFP { rs2: _, rs1, rm: _, rd } = inst.rfp();
        cpu.fregs.set(rd, cpu.regs[rs1]);
        Ok(())
      },
    },
  ])
}
