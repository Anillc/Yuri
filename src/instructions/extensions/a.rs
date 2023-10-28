use std::sync::atomic::Ordering;

use crate::{instructions::Instructor, trap::Exception};

use super::{funct_ra, funct_ra_rs2, RA, InstructionParser};

pub(crate) fn a() -> Vec<Instructor> {
  Vec::from([
    Instructor {
      name: "LR.W",
      opcode: 0b0101111,
      segments: funct_ra_rs2(0b010, 0b00010),
      run: |inst, _len, hart| {
        let RA { rs1, rd, .. } = inst.ra();
        let address = hart.regs[rs1];
        align32(address)?;
        let data = hart.mem.read32(address) as i32 as i64 as u64;
        hart.regs.set(rd, data);
        hart.mem.lock_addr(address);
        Ok(())
      },
    },

    Instructor {
      name: "SC.W",
      opcode: 0b0101111,
      segments: funct_ra(0b010, 0b00011),
      run: |inst, _len, hart| {
        let RA { rs2, rs1, rd, .. } = inst.ra();
        let address = hart.regs[rs1];
        align32(address)?;
        if hart.mem.unlock_addr(address) {
          hart.mem.write32(address, hart.regs[rs2] as u32);
          hart.regs.set(rd, 0);
        } else {
          hart.regs.set(rd, 1);
        }
        Ok(())
      },
    },

    Instructor {
      name: "AMOSWAP.W",
      opcode: 0b0101111,
      segments: funct_ra(0b010, 0b00001),
      run: |inst, _len, hart| {
        let RA { aq, rl, rs2, rs1, rd } = inst.ra();
        let address = hart.regs[rs1];
        align32(address)?;
        let atomic = hart.mem.atomic32(address);
        let res = atomic.swap(hart.regs[rs2] as u32, ordering(aq, rl));
        hart.regs.set(rd, res as i32 as i64 as u64);
        Ok(())
      },
    },

    Instructor {
      name: "AMOADD.W",
      opcode: 0b0101111,
      segments: funct_ra(0b010, 0b00000),
      run: |inst, _len, hart| {
        let RA { aq, rl, rs2, rs1, rd } = inst.ra();
        let address = hart.regs[rs1];
        align32(address)?;
        let atomic = hart.mem.atomic32(address);
        let res = atomic.fetch_add(hart.regs[rs2] as u32, ordering(aq, rl));
        hart.regs.set(rd, res as i32 as i64 as u64);
        Ok(())
      },
    },

    Instructor {
      name: "AMOXOR.W",
      opcode: 0b0101111,
      segments: funct_ra(0b010, 0b00100),
      run: |inst, _len, hart| {
        let RA { aq, rl, rs2, rs1, rd } = inst.ra();
        let address = hart.regs[rs1];
        align32(address)?;
        let atomic = hart.mem.atomic32(address);
        let res = atomic.fetch_xor(hart.regs[rs2] as u32, ordering(aq, rl));
        hart.regs.set(rd, res as i32 as i64 as u64);
        Ok(())
      },
    },

    Instructor {
      name: "AMOAND.W",
      opcode: 0b0101111,
      segments: funct_ra(0b010, 0b01100),
      run: |inst, _len, hart| {
        let RA { aq, rl, rs2, rs1, rd } = inst.ra();
        let address = hart.regs[rs1];
        align32(address)?;
        let atomic = hart.mem.atomic32(address);
        let res = atomic.fetch_and(hart.regs[rs2] as u32, ordering(aq, rl));
        hart.regs.set(rd, res as i32 as i64 as u64);
        Ok(())
      },
    },

    Instructor {
      name: "AMOOR.W",
      opcode: 0b0101111,
      segments: funct_ra(0b010, 0b01000),
      run: |inst, _len, hart| {
        let RA { aq, rl, rs2, rs1, rd } = inst.ra();
        let address = hart.regs[rs1];
        align32(address)?;
        let atomic = hart.mem.atomic32(address);
        let res = atomic.fetch_or(hart.regs[rs2] as u32, ordering(aq, rl));
        hart.regs.set(rd, res as i32 as i64 as u64);
        Ok(())
      },
    },

    Instructor {
      name: "AMOMIN.W",
      opcode: 0b0101111,
      segments: funct_ra(0b010, 0b10000),
      run: |inst, _len, hart| {
        let RA { aq, rl, rs2, rs1, rd } = inst.ra();
        let address = hart.regs[rs1];
        align32(address)?;
        let atomic = hart.mem.atomic32i(address);
        let res = atomic.fetch_min(hart.regs[rs2] as i32, ordering(aq, rl));
        hart.regs.set(rd, res as i32 as i64 as u64);
        Ok(())
      },
    },

    Instructor {
      name: "AMOMAX.W",
      opcode: 0b0101111,
      segments: funct_ra(0b010, 0b10100),
      run: |inst, _len, hart| {
        let RA { aq, rl, rs2, rs1, rd } = inst.ra();
        let address = hart.regs[rs1];
        align32(address)?;
        let atomic = hart.mem.atomic32i(address);
        let res = atomic.fetch_max(hart.regs[rs2] as i32, ordering(aq, rl));
        hart.regs.set(rd, res as i32 as i64 as u64);
        Ok(())
      },
    },

    Instructor {
      name: "AMOMINU.W",
      opcode: 0b0101111,
      segments: funct_ra(0b010, 0b11000),
      run: |inst, _len, hart| {
        let RA { aq, rl, rs2, rs1, rd } = inst.ra();
        let address = hart.regs[rs1];
        align32(address)?;
        let atomic = hart.mem.atomic32(address);
        let res = atomic.fetch_min(hart.regs[rs2] as u32, ordering(aq, rl));
        hart.regs.set(rd, res as i32 as i64 as u64);
        Ok(())
      },
    },

    Instructor {
      name: "AMOMAXU.W",
      opcode: 0b0101111,
      segments: funct_ra(0b010, 0b11100),
      run: |inst, _len, hart| {
        let RA { aq, rl, rs2, rs1, rd } = inst.ra();
        let address = hart.regs[rs1];
        align32(address)?;
        let atomic = hart.mem.atomic32(address);
        let res = atomic.fetch_max(hart.regs[rs2] as u32, ordering(aq, rl));
        hart.regs.set(rd, res as i32 as i64 as u64);
        Ok(())
      },
    },

    Instructor {
      name: "LR.D",
      opcode: 0b0101111,
      segments: funct_ra_rs2(0b011, 0b00010),
      run: |inst, _len, hart| {
        let RA { rs1, rd, .. } = inst.ra();
        let address = hart.regs[rs1];
        align64(address)?;
        let data = hart.mem.read64(address);
        hart.regs.set(rd, data);
        hart.mem.lock_addr(address);
        Ok(())
      },
    },

    Instructor {
      name: "SC.D",
      opcode: 0b0101111,
      segments: funct_ra(0b011, 0b00011),
      run: |inst, _len, hart| {
        let RA { rs2, rs1, rd, .. } = inst.ra();
        let address = hart.regs[rs1];
        align64(address)?;
        if hart.mem.unlock_addr(address) {
          hart.mem.write64(address, hart.regs[rs2]);
          hart.regs.set(rd, 0);
        } else {
          hart.regs.set(rd, 1);
        }
        Ok(())
      },
    },

    Instructor {
      name: "AMOSWAP.D",
      opcode: 0b0101111,
      segments: funct_ra(0b011, 0b00001),
      run: |inst, _len, hart| {
        let RA { aq, rl, rs2, rs1, rd } = inst.ra();
        let address = hart.regs[rs1];
        align64(address)?;
        let atomic = hart.mem.atomic64(address);
        let res = atomic.swap(hart.regs[rs2], ordering(aq, rl));
        hart.regs.set(rd, res);
        Ok(())
      },
    },

    Instructor {
      name: "AMOADD.D",
      opcode: 0b0101111,
      segments: funct_ra(0b011, 0b00000),
      run: |inst, _len, hart| {
        let RA { aq, rl, rs2, rs1, rd } = inst.ra();
        let address = hart.regs[rs1];
        align64(address)?;
        let atomic = hart.mem.atomic64(address);
        let res = atomic.fetch_add(hart.regs[rs2], ordering(aq, rl));
        hart.regs.set(rd, res);
        Ok(())
      },
    },

    Instructor {
      name: "AMOXOR.D",
      opcode: 0b0101111,
      segments: funct_ra(0b011, 0b00100),
      run: |inst, _len, hart| {
        let RA { aq, rl, rs2, rs1, rd } = inst.ra();
        let address = hart.regs[rs1];
        align64(address)?;
        let atomic = hart.mem.atomic64(address);
        let res = atomic.fetch_xor(hart.regs[rs2], ordering(aq, rl));
        hart.regs.set(rd, res);
        Ok(())
      },
    },

    Instructor {
      name: "AMOAND.D",
      opcode: 0b0101111,
      segments: funct_ra(0b011, 0b01100),
      run: |inst, _len, hart| {
        let RA { aq, rl, rs2, rs1, rd } = inst.ra();
        let address = hart.regs[rs1];
        align64(address)?;
        let atomic = hart.mem.atomic64(address);
        let res = atomic.fetch_and(hart.regs[rs2], ordering(aq, rl));
        hart.regs.set(rd, res);
        Ok(())
      },
    },

    Instructor {
      name: "AMOOR.D",
      opcode: 0b0101111,
      segments: funct_ra(0b011, 0b01000),
      run: |inst, _len, hart| {
        let RA { aq, rl, rs2, rs1, rd } = inst.ra();
        let address = hart.regs[rs1];
        align64(address)?;
        let atomic = hart.mem.atomic64(address);
        let res = atomic.fetch_or(hart.regs[rs2], ordering(aq, rl));
        hart.regs.set(rd, res);
        Ok(())
      },
    },

    Instructor {
      name: "AMOMIN.D",
      opcode: 0b0101111,
      segments: funct_ra(0b011, 0b10000),
      run: |inst, _len, hart| {
        let RA { aq, rl, rs2, rs1, rd } = inst.ra();
        let address = hart.regs[rs1];
        align64(address)?;
        let atomic = hart.mem.atomic64i(address);
        let res = atomic.fetch_min(hart.regs[rs2] as i64, ordering(aq, rl));
        hart.regs.set(rd, res as u64);
        Ok(())
      },
    },

    Instructor {
      name: "AMOMAX.D",
      opcode: 0b0101111,
      segments: funct_ra(0b011, 0b10100),
      run: |inst, _len, hart| {
        let RA { aq, rl, rs2, rs1, rd } = inst.ra();
        let address = hart.regs[rs1];
        align64(address)?;
        let atomic = hart.mem.atomic64i(address);
        let res = atomic.fetch_max(hart.regs[rs2] as i64, ordering(aq, rl));
        hart.regs.set(rd, res as u64);
        Ok(())
      },
    },

    Instructor {
      name: "AMOMINU.D",
      opcode: 0b0101111,
      segments: funct_ra(0b011, 0b11000),
      run: |inst, _len, hart| {
        let RA { aq, rl, rs2, rs1, rd } = inst.ra();
        let address = hart.regs[rs1];
        align64(address)?;
        let atomic = hart.mem.atomic64(address);
        let res = atomic.fetch_min(hart.regs[rs2], ordering(aq, rl));
        hart.regs.set(rd, res);
        Ok(())
      },
    },

    Instructor {
      name: "AMOMAXU.D",
      opcode: 0b0101111,
      segments: funct_ra(0b011, 0b11100),
      run: |inst, _len, hart| {
        let RA { aq, rl, rs2, rs1, rd } = inst.ra();
        let address = hart.regs[rs1];
        align64(address)?;
        let atomic = hart.mem.atomic64(address);
        let res = atomic.fetch_max(hart.regs[rs2], ordering(aq, rl));
        hart.regs.set(rd, res);
        Ok(())
      },
    },
  ])
}

fn ordering(aq: bool, rl: bool) -> Ordering {
  match (aq, rl) {
    (false, false) => Ordering::Relaxed,
    (true, false) => Ordering::Acquire,
    (false, true) => Ordering::Release,
    (true, true) => Ordering::AcqRel,
  }
}

fn align32(address: u64) -> Result<(), Exception>{
  if address % 4 != 0 {
    Err(Exception::LoadAddressMisaligned(address))
  } else {
    Ok(())
  }
}

fn align64(address: u64) -> Result<(), Exception>{
  if address % 8 != 0 {
    Err(Exception::LoadAddressMisaligned(address))
  } else {
    Ok(())
  }
}
