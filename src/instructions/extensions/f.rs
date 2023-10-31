use softfloat_wrapper::{F32, Float};

use crate::{instructions::Instructor, utils::{round_mode, classify, Boxed, FloatFlags, check_and_set_fs}};

use super::{funct3, I, InstructionParser, S, funct_rfp_rs3, RFPRS3, funct_rfp, RFP, funct_rfp_rs2, funct_rfp_rm, funct_rfp_rs2_rm};

const NANBOX: u64 = ((-1i64) as u64) << 32;

pub(crate) fn f() -> Vec<Instructor> {
  Vec::from([
    Instructor {
      name: "FLW",
      opcode: 0b0000111,
      segments: funct3(0b010),
      run: |inst, _len, mmu, hart| {
        check_and_set_fs(hart, true)?;
        let I { imm, rs1, rd } = inst.i();
        let address = hart.regs[rs1].wrapping_add(imm as u64);
        hart.fregs.set(rd, mmu.read32(hart, address)? as u64 | NANBOX);
        Ok(())
      },
    },

    Instructor {
      name: "FSW",
      opcode: 0b0100111,
      segments: funct3(0b010),
      run: |inst, _len, mmu, hart| {
        check_and_set_fs(hart, false)?;
        let S { imm, rs2, rs1 } = inst.s();
        let address = hart.regs[rs1].wrapping_add(imm as u64);
        mmu.write32(hart, address, hart.fregs[rs2] as u32)?;
        Ok(())
      },
    },

    Instructor {
      name: "FMADD.S",
      opcode: 0b1000011,
      segments: funct_rfp_rs3(0b00),
      run: |inst, _len, _mmu, hart| {
        check_and_set_fs(hart, true)?;
        let RFPRS3 { rs3, rs2, rs1, rm, rd } = inst.rfp_rs3();
        let flags = FloatFlags::new();
        let rm = round_mode(rm, hart)?;
        let a = F32::from_bits(hart.fregs[rs1].unbox());
        let b = F32::from_bits(hart.fregs[rs2].unbox());
        let c = F32::from_bits(hart.fregs[rs3].unbox());
        hart.fregs.set(rd, a.fused_mul_add(b, c, rm).to_bits() as u64 | NANBOX);
        flags.write(&mut hart.csr, false);
        Ok(())
      },
    },

    Instructor {
      name: "FMSUB.S",
      opcode: 0b1000111,
      segments: funct_rfp_rs3(0b00),
      run: |inst, _len, _mmu, hart| {
        check_and_set_fs(hart, true)?;
        let RFPRS3 { rs3, rs2, rs1, rm, rd } = inst.rfp_rs3();
        let flags = FloatFlags::new();
        let rm = round_mode(rm, hart)?;
        let a = F32::from_bits(hart.fregs[rs1].unbox());
        let b = F32::from_bits(hart.fregs[rs2].unbox());
        let c = F32::from_bits(hart.fregs[rs3].unbox());
        hart.fregs.set(rd, a.fused_mul_add(b, c.neg(), rm).to_bits() as u64 | NANBOX);
        flags.write(&mut hart.csr, false);
        Ok(())
      },
    },

    Instructor {
      name: "FNMSUB.S",
      opcode: 0b1001011,
      segments: funct_rfp_rs3(0b00),
      run: |inst, _len, _mmu, hart| {
        check_and_set_fs(hart, true)?;
        let RFPRS3 { rs3, rs2, rs1, rm, rd } = inst.rfp_rs3();
        let flags = FloatFlags::new();
        let rm = round_mode(rm, hart)?;
        let a = F32::from_bits(hart.fregs[rs1].unbox());
        let b = F32::from_bits(hart.fregs[rs2].unbox());
        let c = F32::from_bits(hart.fregs[rs3].unbox());
        hart.fregs.set(rd, a.fused_mul_add(b, c.neg(), rm).neg().to_bits() as u64 | NANBOX);
        flags.write(&mut hart.csr, false);
        Ok(())
      },
    },

    Instructor {
      name: "FNMADD.S",
      opcode: 0b1001111,
      segments: funct_rfp_rs3(0b00),
      run: |inst, _len, _mmu, hart| {
        check_and_set_fs(hart, true)?;
        let RFPRS3 { rs3, rs2, rs1, rm, rd } = inst.rfp_rs3();
        let flags = FloatFlags::new();
        let rm = round_mode(rm, hart)?;
        let a = F32::from_bits(hart.fregs[rs1].unbox());
        let b = F32::from_bits(hart.fregs[rs2].unbox());
        let c = F32::from_bits(hart.fregs[rs3].unbox());
        hart.fregs.set(rd, a.neg().fused_mul_add(b, c.neg(), rm).to_bits() as u64 | NANBOX);
        flags.write(&mut hart.csr, false);
        Ok(())
      },
    },

    Instructor {
      name: "FADD.S",
      opcode: 0b1010011,
      segments: funct_rfp(0b00, 0b00000),
      run: |inst, _len, _mmu, hart| {
        check_and_set_fs(hart, true)?;
        let RFP { rs2, rs1, rm, rd } = inst.rfp();
        let flags = FloatFlags::new();
        let rm = round_mode(rm, hart)?;
        let a = F32::from_bits(hart.fregs[rs1].unbox());
        let b = F32::from_bits(hart.fregs[rs2].unbox());
        hart.fregs.set(rd, a.add(b, rm).to_bits() as u64 | NANBOX);
        flags.write(&mut hart.csr, false);
        Ok(())
      },
    },

    Instructor {
      name: "FSUB.S",
      opcode: 0b1010011,
      segments: funct_rfp(0b00, 0b00001),
      run: |inst, _len, _mmu, hart| {
        check_and_set_fs(hart, true)?;
        let RFP { rs2, rs1, rm, rd } = inst.rfp();
        let flags = FloatFlags::new();
        let rm = round_mode(rm, hart)?;
        let a = F32::from_bits(hart.fregs[rs1].unbox());
        let b = F32::from_bits(hart.fregs[rs2].unbox());
        hart.fregs.set(rd, a.sub(b, rm).to_bits() as u64 | NANBOX);
        flags.write(&mut hart.csr, false);
        Ok(())
      },
    },

    Instructor {
      name: "FMUL.S",
      opcode: 0b1010011,
      segments: funct_rfp(0b00, 0b00010),
      run: |inst, _len, _mmu, hart| {
        check_and_set_fs(hart, true)?;
        let RFP { rs2, rs1, rm, rd } = inst.rfp();
        let flags = FloatFlags::new();
        let rm = round_mode(rm, hart)?;
        let a = F32::from_bits(hart.fregs[rs1].unbox());
        let b = F32::from_bits(hart.fregs[rs2].unbox());
        hart.fregs.set(rd, a.mul(b, rm).to_bits() as u64 | NANBOX);
        flags.write(&mut hart.csr, false);
        Ok(())
      },
    },

    Instructor {
      name: "FDIV.S",
      opcode: 0b1010011,
      segments: funct_rfp(0b00, 0b00011),
      run: |inst, _len, _mmu, hart| {
        check_and_set_fs(hart, true)?;
        let RFP { rs2, rs1, rm, rd } = inst.rfp();
        let flags = FloatFlags::new();
        let rm = round_mode(rm, hart)?;
        let a = F32::from_bits(hart.fregs[rs1].unbox());
        let b = F32::from_bits(hart.fregs[rs2].unbox());
        hart.fregs.set(rd, a.div(b, rm).to_bits() as u64 | NANBOX);
        flags.write(&mut hart.csr, b.is_zero());
        Ok(())
      },
    },

    Instructor {
      name: "FSQRT.S",
      opcode: 0b1010011,
      segments: funct_rfp_rs2(0b00000, 0b00, 0b01011),
      run: |inst, _len, _mmu, hart| {
        check_and_set_fs(hart, true)?;
        let RFP { rs2: _, rs1, rm, rd } = inst.rfp();
        let flags = FloatFlags::new();
        let rm = round_mode(rm, hart)?;
        let num = F32::from_bits(hart.fregs[rs1].unbox());
        hart.fregs.set(rd, num.sqrt(rm).to_bits() as u64 | NANBOX);
        flags.write(&mut hart.csr, false);
        Ok(())
      },
    },

    Instructor {
      name: "FSGNJ.S",
      opcode: 0b1010011,
      segments: funct_rfp_rm(0b000, 0b00, 0b00100),
      run: |inst, _len, _mmu, hart| {
        check_and_set_fs(hart, true)?;
        let RFP { rs2, rs1, rm: _, rd } = inst.rfp();
        let mut a = F32::from_bits(hart.fregs[rs1].unbox());
        let b = F32::from_bits(hart.fregs[rs2].unbox());
        a.set_sign(b.sign());
        hart.fregs.set(rd, a.to_bits() as u64 | NANBOX);
        Ok(())
      },
    },

    Instructor {
      name: "FSGNJN.S",
      opcode: 0b1010011,
      segments: funct_rfp_rm(0b001, 0b00, 0b00100),
      run: |inst, _len, _mmu, hart| {
        check_and_set_fs(hart, true)?;
        let RFP { rs2, rs1, rm: _, rd } = inst.rfp();
        let mut a = F32::from_bits(hart.fregs[rs1].unbox());
        let b = F32::from_bits(hart.fregs[rs2].unbox());
        a.set_sign(!b.sign());
        hart.fregs.set(rd, a.to_bits() as u64 | NANBOX);
        Ok(())
      },
    },

    Instructor {
      name: "FSGNJX.S",
      opcode: 0b1010011,
      segments: funct_rfp_rm(0b010, 0b00, 0b00100),
      run: |inst, _len, _mmu, hart| {
        check_and_set_fs(hart, true)?;
        let RFP { rs2, rs1, rm: _, rd } = inst.rfp();
        let mut a = F32::from_bits(hart.fregs[rs1].unbox());
        let b = F32::from_bits(hart.fregs[rs2].unbox());
        a.set_sign(a.sign() ^ b.sign());
        hart.fregs.set(rd, a.to_bits() as u64 | NANBOX);
        Ok(())
      },
    },

    Instructor {
      name: "FMIN.S",
      opcode: 0b1010011,
      segments: funct_rfp_rm(0b000, 0b00, 0b00101),
      run: |inst, _len, _mmu, hart| {
        check_and_set_fs(hart, true)?;
        let RFP { rs2, rs1, rm: _, rd } = inst.rfp();
        let flags = FloatFlags::new();
        let a = F32::from_bits(hart.fregs[rs1].unbox());
        let b = F32::from_bits(hart.fregs[rs2].unbox());
        let less = a.lt_quiet(b) || a.eq(b) && a.sign() != 0;
        let res = if a.is_nan() && b.is_nan() {
          F32::quiet_nan()
        } else if less || b.is_nan() {
          a
        } else {
          b
        };
        hart.fregs.set(rd, res.to_bits() as u64 | NANBOX);
        flags.write(&mut hart.csr, false);
        Ok(())
      },
    },

    Instructor {
      name: "FMAX.S",
      opcode: 0b1010011,
      segments: funct_rfp_rm(0b001, 0b00, 0b00101),
      run: |inst, _len, _mmu, hart| {
        check_and_set_fs(hart, true)?;
        let RFP { rs2, rs1, rm: _, rd } = inst.rfp();
        let flags = FloatFlags::new();
        let a = F32::from_bits(hart.fregs[rs1].unbox());
        let b = F32::from_bits(hart.fregs[rs2].unbox());
        let greater = b.lt_quiet(a) || b.eq(a) && b.sign() != 0;
        let res = if a.is_nan() && b.is_nan() {
          F32::quiet_nan()
        } else if greater || b.is_nan() {
          a
        } else {
          b
        };
        hart.fregs.set(rd, res.to_bits() as u64 | NANBOX);
        flags.write(&mut hart.csr, false);
        Ok(())
      },
    },

    Instructor {
      name: "FCVT.W.S",
      opcode: 0b1010011,
      segments: funct_rfp_rs2(0b00000, 0b00, 0b11000),
      run: |inst, _len, _mmu, hart| {
        check_and_set_fs(hart, true)?;
        let RFP { rs2: _, rs1, rm, rd } = inst.rfp();
        let flags = FloatFlags::new();
        let rm = round_mode(rm, hart)?;
        let num = F32::from_bits(hart.fregs[rs1].unbox());
        hart.regs.set(rd, num.to_i32(rm, true) as i64 as u64);
        flags.write(&mut hart.csr, false);
        Ok(())
      },
    },

    Instructor {
      name: "FCVT.WU.S",
      opcode: 0b1010011,
      segments: funct_rfp_rs2(0b00001, 0b00, 0b11000),
      run: |inst, _len, _mmu, hart| {
        check_and_set_fs(hart, true)?;
        let RFP { rs2: _, rs1, rm, rd } = inst.rfp();
        let flags = FloatFlags::new();
        let rm = round_mode(rm, hart)?;
        let num = F32::from_bits(hart.fregs[rs1].unbox());
        hart.regs.set(rd, num.to_u32(rm, true) as i32 as i64 as u64);
        flags.write(&mut hart.csr, false);
        Ok(())
      },
    },

    Instructor {
      name: "FMV.X.W",
      opcode: 0b1010011,
      segments: funct_rfp_rs2_rm(0b000, 0b00000, 0b00, 0b11100),
      run: |inst, _len, _mmu, hart| {
        check_and_set_fs(hart, false)?;
        let RFP { rs2: _, rs1, rm: _, rd } = inst.rfp();
        hart.regs.set(rd, hart.fregs[rs1] as u32 as i32 as i64 as u64);
        Ok(())
      },
    },

    Instructor {
      name: "FEQ.S",
      opcode: 0b1010011,
      segments: funct_rfp_rm(0b010, 0b00, 0b10100),
      run: |inst, _len, _mmu, hart| {
        check_and_set_fs(hart, true)?;
        let RFP { rs2, rs1, rm: _, rd } = inst.rfp();
        let flags = FloatFlags::new();
        let a = F32::from_bits(hart.fregs[rs1].unbox());
        let b = F32::from_bits(hart.fregs[rs2].unbox());
        hart.regs.set(rd, if a.eq(b) { 1 } else { 0 });
        flags.write(&mut hart.csr, false);
        Ok(())
      },
    },

    Instructor {
      name: "FLT.S",
      opcode: 0b1010011,
      segments: funct_rfp_rm(0b001, 0b00, 0b10100),
      run: |inst, _len, _mmu, hart| {
        check_and_set_fs(hart, true)?;
        let RFP { rs2, rs1, rm: _, rd } = inst.rfp();
        let flags = FloatFlags::new();
        let a = F32::from_bits(hart.fregs[rs1].unbox());
        let b = F32::from_bits(hart.fregs[rs2].unbox());
        hart.regs.set(rd, if a.lt(b) { 1 } else { 0 });
        flags.write(&mut hart.csr, false);
        Ok(())
      },
    },

    Instructor {
      name: "FLE.S",
      opcode: 0b1010011,
      segments: funct_rfp_rm(0b000, 0b00, 0b10100),
      run: |inst, _len, _mmu, hart| {
        check_and_set_fs(hart, true)?;
        let RFP { rs2, rs1, rm: _, rd } = inst.rfp();
        let flags = FloatFlags::new();
        let a = F32::from_bits(hart.fregs[rs1].unbox());
        let b = F32::from_bits(hart.fregs[rs2].unbox());
        hart.regs.set(rd, if a.le(b) { 1 } else { 0 });
        flags.write(&mut hart.csr, false);
        Ok(())
      },
    },

    Instructor {
      name: "FCLASS.S",
      opcode: 0b1010011,
      segments: funct_rfp_rs2_rm(0b001, 0b00000, 0b00, 0b11100),
      run: |inst, _len, _mmu, hart| {
        check_and_set_fs(hart, false)?;
        let RFP { rs2: _, rs1, rm: _, rd } = inst.rfp();
        let num = F32::from_bits(hart.fregs[rs1].unbox());
        hart.regs.set(rd, classify(num));
        Ok(())
      },
    },

    Instructor {
      name: "FCVT.S.W",
      opcode: 0b1010011,
      segments: funct_rfp_rs2(0b00000, 0b00, 0b11010),
      run: |inst, _len, _mmu, hart| {
        check_and_set_fs(hart, true)?;
        let RFP { rs2: _, rs1, rm, rd } = inst.rfp();
        let flags = FloatFlags::new();
        let rm = round_mode(rm, hart)?;
        let num = F32::from_i32(hart.regs[rs1] as i32, rm);
        hart.fregs.set(rd, num.to_bits() as u64 | NANBOX);
        flags.write(&mut hart.csr, false);
        Ok(())
      },
    },

    Instructor {
      name: "FCVT.S.WU",
      opcode: 0b1010011,
      segments: funct_rfp_rs2(0b00001, 0b00, 0b11010),
      run: |inst, _len, _mmu, hart| {
        check_and_set_fs(hart, true)?;
        let RFP { rs2: _, rs1, rm, rd } = inst.rfp();
        let flags = FloatFlags::new();
        let rm = round_mode(rm, hart)?;
        let num = F32::from_u32(hart.regs[rs1] as u32, rm);
        hart.fregs.set(rd, num.to_bits() as u64 | NANBOX);
        flags.write(&mut hart.csr, false);
        Ok(())
      },
    },

    Instructor {
      name: "FMV.W.X",
      opcode: 0b1010011,
      segments: funct_rfp_rs2_rm(0b000, 0b00000, 0b00, 0b11110),
      run: |inst, _len, _mmu, hart| {
        check_and_set_fs(hart, true)?;
        let RFP { rs2: _, rs1, rm: _, rd } = inst.rfp();
        hart.fregs.set(rd, hart.regs[rs1] as u32 as u64 | NANBOX);
        Ok(())
      },
    },

    Instructor {
      name: "FCVT.L.S",
      opcode: 0b1010011,
      segments: funct_rfp_rs2(0b00010, 0b00, 0b11000),
      run: |inst, _len, _mmu, hart| {
        check_and_set_fs(hart, true)?;
        let RFP { rs2: _, rs1, rm, rd } = inst.rfp();
        let flags = FloatFlags::new();
        let rm = round_mode(rm, hart)?;
        let num = F32::from_bits(hart.fregs[rs1].unbox());
        hart.regs.set(rd, num.to_i64(rm, true) as u64);
        flags.write(&mut hart.csr, false);
        Ok(())
      },
    },

    Instructor {
      name: "FCVT.LU.S",
      opcode: 0b1010011,
      segments: funct_rfp_rs2(0b00011, 0b00, 0b11000),
      run: |inst, _len, _mmu, hart| {
        check_and_set_fs(hart, true)?;
        let RFP { rs2: _, rs1, rm, rd } = inst.rfp();
        let flags = FloatFlags::new();
        let rm = round_mode(rm, hart)?;
        let num = F32::from_bits(hart.fregs[rs1].unbox());
        hart.regs.set(rd, num.to_u64(rm, true) as u64);
        flags.write(&mut hart.csr, false);
        Ok(())
      },
    },

    Instructor {
      name: "FCVT.S.L",
      opcode: 0b1010011,
      segments: funct_rfp_rs2(0b00010, 0b00, 0b11010),
      run: |inst, _len, _mmu, hart| {
        check_and_set_fs(hart, true)?;
        let RFP { rs2: _, rs1, rm, rd } = inst.rfp();
        let flags = FloatFlags::new();
        let rm = round_mode(rm, hart)?;
        let num = F32::from_i64(hart.regs[rs1] as i64, rm);
        hart.fregs.set(rd, num.to_bits() as u64 | NANBOX);
        flags.write(&mut hart.csr, false);
        Ok(())
      },
    },

    Instructor {
      name: "FCVT.S.LU",
      opcode: 0b1010011,
      segments: funct_rfp_rs2(0b00011, 0b00, 0b11010),
      run: |inst, _len, _mmu, hart| {
        check_and_set_fs(hart, true)?;
        let RFP { rs2: _, rs1, rm, rd } = inst.rfp();
        let flags = FloatFlags::new();
        let rm = round_mode(rm, hart)?;
        let num = F32::from_u64(hart.regs[rs1], rm);
        hart.fregs.set(rd, num.to_bits() as u64 | NANBOX);
        flags.write(&mut hart.csr, false);
        Ok(())
      },
    },
  ])
}
