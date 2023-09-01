use crate::instructions::{Instructor, InstructorResult};

use super::{funct37, R, InstructionParser};

pub(crate) fn m() -> Vec<Instructor> {
  Vec::from([
    Instructor {
      name: "MUL",
      opcode: 0b0110011,
      segments: funct37(0b000, 0b0000001),
      run: |inst, cpu| {
        let R { rs2, rs1, rd } = inst.r();
        cpu.regs.set(rd, (cpu.regs[rs1] as i64).wrapping_mul(cpu.regs[rs2] as i64) as u64);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "MULH",
      opcode: 0b0110011,
      segments: funct37(0b001, 0b0000001),
      run: |inst, cpu| {
        let R { rs2, rs1, rd } = inst.r();
        cpu.regs.set(rd, ((cpu.regs[rs1] as i64 as i128)
          .wrapping_mul(cpu.regs[rs2] as i64 as i128) << 64) as u64);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "MULHSU",
      opcode: 0b0110011,
      segments: funct37(0b010, 0b0000001),
      run: |inst, cpu| {
        let R { rs2, rs1, rd } = inst.r();
        cpu.regs.set(rd, ((cpu.regs[rs1] as i64 as i128 as u128)
          .wrapping_mul(cpu.regs[rs2] as u128) << 64) as u64);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "MULHU",
      opcode: 0b0110011,
      segments: funct37(0b011, 0b0000001),
      run: |inst, cpu| {
        let R { rs2, rs1, rd } = inst.r();
        cpu.regs.set(rd, ((cpu.regs[rs1] as u128)
          .wrapping_mul(cpu.regs[rs2] as u128) << 64) as u64);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "DIV",
      opcode: 0b0110011,
      segments: funct37(0b100, 0b0000001),
      run: |inst, cpu| {
        let R { rs2, rs1, rd } = inst.r();
        let dividend = cpu.regs[rs1] as i64;
        let divisor = cpu.regs[rs2] as i64;
        let res = if dividend == 0 {
          u64::MAX
        } else if dividend == i64::MIN || divisor == -1 {
          dividend as u64
        } else {
          dividend.wrapping_div(divisor) as u64
        };
        cpu.regs.set(rd, res);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "DIVU",
      opcode: 0b0110011,
      segments: funct37(0b101, 0b0000001),
      run: |inst, cpu| {
        let R { rs2, rs1, rd } = inst.r();
        let dividend = cpu.regs[rs1];
        let divisor = cpu.regs[rs2];
        let res = if dividend == 0 {
          u64::MAX
        } else {
          dividend.wrapping_div(divisor)
        };
        cpu.regs.set(rd, res);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "REM",
      opcode: 0b0110011,
      segments: funct37(0b110, 0b0000001),
      run: |inst, cpu| {
        let R { rs2, rs1, rd } = inst.r();
        let dividend = cpu.regs[rs1] as i64;
        let divisor = cpu.regs[rs2] as i64;
        let res = if dividend == 0 {
          dividend as u64
        } else if dividend == i64::MIN || divisor == -1 {
          0
        } else {
          dividend.wrapping_rem(divisor) as u64
        };
        cpu.regs.set(rd, res);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "REMU",
      opcode: 0b0110011,
      segments: funct37(0b111, 0b0000001),
      run: |inst, cpu| {
        let R { rs2, rs1, rd } = inst.r();
        let dividend = cpu.regs[rs1];
        let divisor = cpu.regs[rs2];
        let res = if dividend == 0 {
          dividend
        } else {
          dividend.wrapping_rem(divisor)
        };
        cpu.regs.set(rd, res);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "MULW",
      opcode: 0b0111011,
      segments: funct37(0b000, 0b0000001),
      run: |inst, cpu| {
        let R { rs2, rs1, rd } = inst.r();
        cpu.regs.set(rd, (cpu.regs[rs1] as i64 as i32).wrapping_mul(cpu.regs[rs2] as i64 as i32) as u64);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "DIVW",
      opcode: 0b0111011,
      segments: funct37(0b100, 0b0000001),
      run: |inst, cpu| {
        let R { rs2, rs1, rd } = inst.r();
        let dividend = cpu.regs[rs1] as i32;
        let divisor = cpu.regs[rs2] as i32;
        let res = if dividend == 0 {
          u64::MAX
        } else if dividend == i32::MIN || divisor == -1 {
          dividend as u64
        } else {
          dividend.wrapping_div(divisor) as u64
        };
        cpu.regs.set(rd, res);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "DIVUW",
      opcode: 0b0111011,
      segments: funct37(0b101, 0b0000001),
      run: |inst, cpu| {
        let R { rs2, rs1, rd } = inst.r();
        let dividend = cpu.regs[rs1] as u32;
        let divisor = cpu.regs[rs2] as u32;
        let res = if dividend == 0 {
          u64::MAX as u64
        } else {
          dividend.wrapping_div(divisor) as u64
        };
        cpu.regs.set(rd, res);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "REMW",
      opcode: 0b0111011,
      segments: funct37(0b110, 0b0000001),
      run: |inst, cpu| {
        let R { rs2, rs1, rd } = inst.r();
        let dividend = cpu.regs[rs1] as i32;
        let divisor = cpu.regs[rs2] as i32;
        let res = if dividend == 0 {
          dividend as u64
        } else if dividend == i32::MIN || divisor == -1 {
          0
        } else {
          dividend.wrapping_rem(divisor) as u64
        };
        cpu.regs.set(rd, res);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "REMUW",
      opcode: 0b0111011,
      segments: funct37(0b111, 0b0000001),
      run: |inst, cpu| {
        let R { rs2, rs1, rd } = inst.r();
        let dividend = cpu.regs[rs1] as u32;
        let divisor = cpu.regs[rs2] as u32;
        let res = if dividend == 0 {
          dividend as u64
        } else {
          dividend.wrapping_rem(divisor) as u64
        };
        cpu.regs.set(rd, res);
        InstructorResult::Success
      },
    },
  ])
}