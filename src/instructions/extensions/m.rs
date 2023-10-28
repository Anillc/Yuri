use crate::instructions::Instructor;

use super::{funct37, R, InstructionParser};

pub(crate) fn m() -> Vec<Instructor> {
  Vec::from([
    Instructor {
      name: "MUL",
      opcode: 0b0110011,
      segments: funct37(0b000, 0b0000001),
      run: |inst, _len, hart| {
        let R { rs2, rs1, rd } = inst.r();
        hart.regs.set(rd, (hart.regs[rs1] as i64).wrapping_mul(hart.regs[rs2] as i64) as u64);
        Ok(())
      },
    },

    Instructor {
      name: "MULH",
      opcode: 0b0110011,
      segments: funct37(0b001, 0b0000001),
      run: |inst, _len, hart| {
        let R { rs2, rs1, rd } = inst.r();
        hart.regs.set(rd, ((hart.regs[rs1] as i64 as i128)
          .wrapping_mul(hart.regs[rs2] as i64 as i128) >> 64) as u64);
        Ok(())
      },
    },

    Instructor {
      name: "MULHSU",
      opcode: 0b0110011,
      segments: funct37(0b010, 0b0000001),
      run: |inst, _len, hart| {
        let R { rs2, rs1, rd } = inst.r();
        hart.regs.set(rd, ((hart.regs[rs1] as i64 as i128 as u128)
          .wrapping_mul(hart.regs[rs2] as u128) >> 64) as u64);
        Ok(())
      },
    },

    Instructor {
      name: "MULHU",
      opcode: 0b0110011,
      segments: funct37(0b011, 0b0000001),
      run: |inst, _len, hart| {
        let R { rs2, rs1, rd } = inst.r();
        hart.regs.set(rd, ((hart.regs[rs1] as u128)
          .wrapping_mul(hart.regs[rs2] as u128) >> 64) as u64);
        Ok(())
      },
    },

    Instructor {
      name: "DIV",
      opcode: 0b0110011,
      segments: funct37(0b100, 0b0000001),
      run: |inst, _len, hart| {
        let R { rs2, rs1, rd } = inst.r();
        let dividend = hart.regs[rs1] as i64;
        let divisor = hart.regs[rs2] as i64;
        let res = if divisor == 0 {
          u64::MAX
        } else if dividend == i64::MIN || divisor == -1 {
          dividend as u64
        } else {
          dividend.wrapping_div(divisor) as u64
        };
        hart.regs.set(rd, res);
        Ok(())
      },
    },

    Instructor {
      name: "DIVU",
      opcode: 0b0110011,
      segments: funct37(0b101, 0b0000001),
      run: |inst, _len, hart| {
        let R { rs2, rs1, rd } = inst.r();
        let dividend = hart.regs[rs1];
        let divisor = hart.regs[rs2];
        let res = if divisor == 0 {
          u64::MAX
        } else {
          dividend.wrapping_div(divisor)
        };
        hart.regs.set(rd, res);
        Ok(())
      },
    },

    Instructor {
      name: "REM",
      opcode: 0b0110011,
      segments: funct37(0b110, 0b0000001),
      run: |inst, _len, hart| {
        let R { rs2, rs1, rd } = inst.r();
        let dividend = hart.regs[rs1] as i64;
        let divisor = hart.regs[rs2] as i64;
        let res = if divisor == 0 {
          dividend as u64
        } else if dividend == i64::MIN || divisor == -1 {
          0
        } else {
          dividend.wrapping_rem(divisor) as u64
        };
        hart.regs.set(rd, res);
        Ok(())
      },
    },

    Instructor {
      name: "REMU",
      opcode: 0b0110011,
      segments: funct37(0b111, 0b0000001),
      run: |inst, _len, hart| {
        let R { rs2, rs1, rd } = inst.r();
        let dividend = hart.regs[rs1];
        let divisor = hart.regs[rs2];
        let res = if divisor == 0 {
          dividend
        } else {
          dividend.wrapping_rem(divisor)
        };
        hart.regs.set(rd, res);
        Ok(())
      },
    },

    Instructor {
      name: "MULW",
      opcode: 0b0111011,
      segments: funct37(0b000, 0b0000001),
      run: |inst, _len, hart| {
        let R { rs2, rs1, rd } = inst.r();
        hart.regs.set(rd, (hart.regs[rs1] as i64 as i32).wrapping_mul(hart.regs[rs2] as i64 as i32) as u64);
        Ok(())
      },
    },

    Instructor {
      name: "DIVW",
      opcode: 0b0111011,
      segments: funct37(0b100, 0b0000001),
      run: |inst, _len, hart| {
        let R { rs2, rs1, rd } = inst.r();
        let dividend = hart.regs[rs1] as i32;
        let divisor = hart.regs[rs2] as i32;
        let res = if divisor == 0 {
          u64::MAX
        } else if dividend == i32::MIN || divisor == -1 {
          dividend as u64
        } else {
          dividend.wrapping_div(divisor) as i64 as u64
        };
        hart.regs.set(rd, res);
        Ok(())
      },
    },

    Instructor {
      name: "DIVUW",
      opcode: 0b0111011,
      segments: funct37(0b101, 0b0000001),
      run: |inst, _len, hart| {
        let R { rs2, rs1, rd } = inst.r();
        let dividend = hart.regs[rs1] as u32;
        let divisor = hart.regs[rs2] as u32;
        let res = if divisor == 0 {
          u64::MAX
        } else {
          dividend.wrapping_div(divisor) as i32 as i64 as u64
        };
        hart.regs.set(rd, res);
        Ok(())
      },
    },

    Instructor {
      name: "REMW",
      opcode: 0b0111011,
      segments: funct37(0b110, 0b0000001),
      run: |inst, _len, hart| {
        let R { rs2, rs1, rd } = inst.r();
        let dividend = hart.regs[rs1] as i32;
        let divisor = hart.regs[rs2] as i32;
        let res = if divisor == 0 {
          dividend as u64
        } else if dividend == i32::MIN || divisor == -1 {
          0
        } else {
          dividend.wrapping_rem(divisor) as i64 as u64
        };
        hart.regs.set(rd, res);
        Ok(())
      },
    },

    Instructor {
      name: "REMUW",
      opcode: 0b0111011,
      segments: funct37(0b111, 0b0000001),
      run: |inst, _len, hart| {
        let R { rs2, rs1, rd } = inst.r();
        let dividend = hart.regs[rs1] as u32;
        let divisor = hart.regs[rs2] as u32;
        let res = if divisor == 0 {
          dividend as i32 as i64 as u64
        } else {
          dividend.wrapping_rem(divisor) as i32 as i64 as u64
        };
        hart.regs.set(rd, res);
        Ok(())
      },
    },
  ])
}