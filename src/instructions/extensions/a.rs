use std::sync::atomic::Ordering;

use crate::instructions::{Instructor, types::{funct_ra, funct_ra_rs2, RA, InstructionParser}};

// TODO: address align
pub(crate) fn a() -> Vec<Instructor> {
  Vec::from([
    Instructor {
      name: "LR.W",
      opcode: 0b0101111,
      segments: funct_ra_rs2(0b010, 0b00010),
      run: |inst, cpu| {
        let RA { rs1, rd, .. } = inst.ra();
        let address = cpu.regs[rs1];
        let data = cpu.mem.read32(address) as i32 as i64 as u64;
        cpu.regs.set(rd, data);
        cpu.mem.lock_addr(address);
      },
    },

    Instructor {
      name: "SC.W",
      opcode: 0b0101111,
      segments: funct_ra(0b010, 0b00011),
      run: |inst, cpu| {
        let RA { rs2, rs1, rd, .. } = inst.ra();
        let address = cpu.regs[rs1];
        if cpu.mem.unlock_addr(address) {
          cpu.mem.write32(address, cpu.regs[rs2] as u32);
          cpu.regs.set(rd, 0);
        } else {
          cpu.regs.set(rd, 1);
        }
      },
    },

    Instructor {
      name: "AMOSWAP.W",
      opcode: 0b0101111,
      segments: funct_ra(0b010, 0b00001),
      run: |inst, cpu| {
        let RA { aq, rl, rs2, rs1, rd } = inst.ra();
        let address = cpu.regs[rs1];
        let src = cpu.mem.read32(cpu.regs[rs2]);
        let atomic = cpu.mem.atomic32(address);
        let res = atomic.swap(src, ordering(aq, rl));
        cpu.regs.set(rd, res as u64);
      },
    },

    Instructor {
      name: "AMOADD.W",
      opcode: 0b0101111,
      segments: funct_ra(0b010, 0b00000),
      run: |inst, cpu| {
        let RA { aq, rl, rs2, rs1, rd } = inst.ra();
        let address = cpu.regs[rs1];
        let src = cpu.mem.read32(cpu.regs[rs2]);
        let atomic = cpu.mem.atomic32(address);
        let res = atomic.fetch_add(src, ordering(aq, rl));
        cpu.regs.set(rd, res as u64);
      },
    },

    Instructor {
      name: "AMOXOR.W",
      opcode: 0b0101111,
      segments: funct_ra(0b010, 0b00100),
      run: |inst, cpu| {
        let RA { aq, rl, rs2, rs1, rd } = inst.ra();
        let address = cpu.regs[rs1];
        let src = cpu.mem.read32(cpu.regs[rs2]);
        let atomic = cpu.mem.atomic32(address);
        let res = atomic.fetch_xor(src, ordering(aq, rl));
        cpu.regs.set(rd, res as u64);
      },
    },

    Instructor {
      name: "AMOAND.W",
      opcode: 0b0101111,
      segments: funct_ra(0b010, 0b01100),
      run: |inst, cpu| {
        let RA { aq, rl, rs2, rs1, rd } = inst.ra();
        let address = cpu.regs[rs1];
        let src = cpu.mem.read32(cpu.regs[rs2]);
        let atomic = cpu.mem.atomic32(address);
        let res = atomic.fetch_and(src, ordering(aq, rl));
        cpu.regs.set(rd, res as u64);
      },
    },

    Instructor {
      name: "AMOOR.W",
      opcode: 0b0101111,
      segments: funct_ra(0b010, 0b01000),
      run: |inst, cpu| {
        let RA { aq, rl, rs2, rs1, rd } = inst.ra();
        let address = cpu.regs[rs1];
        let src = cpu.mem.read32(cpu.regs[rs2]);
        let atomic = cpu.mem.atomic32(address);
        let res = atomic.fetch_or(src, ordering(aq, rl));
        cpu.regs.set(rd, res as u64);
      },
    },

    Instructor {
      name: "AMOMIN.W",
      opcode: 0b0101111,
      segments: funct_ra(0b010, 0b10000),
      run: |inst, cpu| {
        let RA { aq, rl, rs2, rs1, rd } = inst.ra();
        let address = cpu.regs[rs1];
        let src = cpu.mem.read32(cpu.regs[rs2]) as i32;
        let atomic = cpu.mem.atomic32i(address);
        let res = atomic.fetch_min(src, ordering(aq, rl));
        cpu.regs.set(rd, res as u64);
      },
    },

    Instructor {
      name: "AMOMAX.W",
      opcode: 0b0101111,
      segments: funct_ra(0b010, 0b10100),
      run: |inst, cpu| {
        let RA { aq, rl, rs2, rs1, rd } = inst.ra();
        let address = cpu.regs[rs1];
        let src = cpu.mem.read32(cpu.regs[rs2]) as i32;
        let atomic = cpu.mem.atomic32i(address);
        let res = atomic.fetch_max(src, ordering(aq, rl));
        cpu.regs.set(rd, res as u64);
      },
    },

    Instructor {
      name: "AMOMINU.W",
      opcode: 0b0101111,
      segments: funct_ra(0b010, 0b11000),
      run: |inst, cpu| {
        let RA { aq, rl, rs2, rs1, rd } = inst.ra();
        let address = cpu.regs[rs1];
        let src = cpu.mem.read32(cpu.regs[rs2]);
        let atomic = cpu.mem.atomic32(address);
        let res = atomic.fetch_min(src, ordering(aq, rl));
        cpu.regs.set(rd, res as u64);
      },
    },

    Instructor {
      name: "AMOMAXU.W",
      opcode: 0b0101111,
      segments: funct_ra(0b010, 0b11100),
      run: |inst, cpu| {
        let RA { aq, rl, rs2, rs1, rd } = inst.ra();
        let address = cpu.regs[rs1];
        let src = cpu.mem.read32(cpu.regs[rs2]);
        let atomic = cpu.mem.atomic32(address);
        let res = atomic.fetch_max(src, ordering(aq, rl));
        cpu.regs.set(rd, res as u64);
      },
    },

    Instructor {
      name: "LR.D",
      opcode: 0b0101111,
      segments: funct_ra_rs2(0b011, 0b00010),
      run: |inst, cpu| {
        let RA { rs1, rd, .. } = inst.ra();
        let address = cpu.regs[rs1];
        let data = cpu.mem.read64(address);
        cpu.regs.set(rd, data);
        cpu.mem.lock_addr(address);
      },
    },

    Instructor {
      name: "SC.D",
      opcode: 0b0101111,
      segments: funct_ra(0b011, 0b00011),
      run: |inst, cpu| {
        let RA { rs2, rs1, rd, .. } = inst.ra();
        let address = cpu.regs[rs1];
        if cpu.mem.unlock_addr(address) {
          cpu.mem.write64(address, cpu.regs[rs2]);
          cpu.regs.set(rd, 0);
        } else {
          cpu.regs.set(rd, 1);
        }
      },
    },

    Instructor {
      name: "AMOSWAP.D",
      opcode: 0b0101111,
      segments: funct_ra(0b011, 0b00001),
      run: |inst, cpu| {
        let RA { aq, rl, rs2, rs1, rd } = inst.ra();
        let address = cpu.regs[rs1];
        let src = cpu.mem.read64(cpu.regs[rs2]);
        let atomic = cpu.mem.atomic64(address);
        let res = atomic.swap(src, ordering(aq, rl));
        cpu.regs.set(rd, res);
      },
    },

    Instructor {
      name: "AMOADD.D",
      opcode: 0b0101111,
      segments: funct_ra(0b011, 0b00000),
      run: |inst, cpu| {
        let RA { aq, rl, rs2, rs1, rd } = inst.ra();
        let address = cpu.regs[rs1];
        let src = cpu.mem.read64(cpu.regs[rs2]);
        let atomic = cpu.mem.atomic64(address);
        let res = atomic.fetch_add(src, ordering(aq, rl));
        cpu.regs.set(rd, res);
      },
    },

    Instructor {
      name: "AMOXOR.D",
      opcode: 0b0101111,
      segments: funct_ra(0b011, 0b00100),
      run: |inst, cpu| {
        let RA { aq, rl, rs2, rs1, rd } = inst.ra();
        let address = cpu.regs[rs1];
        let src = cpu.mem.read64(cpu.regs[rs2]);
        let atomic = cpu.mem.atomic64(address);
        let res = atomic.fetch_xor(src, ordering(aq, rl));
        cpu.regs.set(rd, res);
      },
    },

    Instructor {
      name: "AMOAND.D",
      opcode: 0b0101111,
      segments: funct_ra(0b011, 0b01100),
      run: |inst, cpu| {
        let RA { aq, rl, rs2, rs1, rd } = inst.ra();
        let address = cpu.regs[rs1];
        let src = cpu.mem.read64(cpu.regs[rs2]);
        let atomic = cpu.mem.atomic64(address);
        let res = atomic.fetch_and(src, ordering(aq, rl));
        cpu.regs.set(rd, res);
      },
    },

    Instructor {
      name: "AMOOR.D",
      opcode: 0b0101111,
      segments: funct_ra(0b011, 0b01000),
      run: |inst, cpu| {
        let RA { aq, rl, rs2, rs1, rd } = inst.ra();
        let address = cpu.regs[rs1];
        let src = cpu.mem.read64(cpu.regs[rs2]);
        let atomic = cpu.mem.atomic64(address);
        let res = atomic.fetch_or(src, ordering(aq, rl));
        cpu.regs.set(rd, res);
      },
    },

    Instructor {
      name: "AMOMIN.D",
      opcode: 0b0101111,
      segments: funct_ra(0b011, 0b10000),
      run: |inst, cpu| {
        let RA { aq, rl, rs2, rs1, rd } = inst.ra();
        let address = cpu.regs[rs1];
        let src = cpu.mem.read64(cpu.regs[rs2]) as i64;
        let atomic = cpu.mem.atomic64i(address);
        let res = atomic.fetch_min(src, ordering(aq, rl));
        cpu.regs.set(rd, res as u64);
      },
    },

    Instructor {
      name: "AMOMAX.D",
      opcode: 0b0101111,
      segments: funct_ra(0b011, 0b10100),
      run: |inst, cpu| {
        let RA { aq, rl, rs2, rs1, rd } = inst.ra();
        let address = cpu.regs[rs1];
        let src = cpu.mem.read64(cpu.regs[rs2]) as i64;
        let atomic = cpu.mem.atomic64i(address);
        let res = atomic.fetch_max(src, ordering(aq, rl));
        cpu.regs.set(rd, res as u64);
      },
    },

    Instructor {
      name: "AMOMINU.D",
      opcode: 0b0101111,
      segments: funct_ra(0b011, 0b11000),
      run: |inst, cpu| {
        let RA { aq, rl, rs2, rs1, rd } = inst.ra();
        let address = cpu.regs[rs1];
        let src = cpu.mem.read64(cpu.regs[rs2]);
        let atomic = cpu.mem.atomic64(address);
        let res = atomic.fetch_min(src, ordering(aq, rl));
        cpu.regs.set(rd, res);
      },
    },

    Instructor {
      name: "AMOMAXU.D",
      opcode: 0b0101111,
      segments: funct_ra(0b011, 0b11100),
      run: |inst, cpu| {
        let RA { aq, rl, rs2, rs1, rd } = inst.ra();
        let address = cpu.regs[rs1];
        let src = cpu.mem.read64(cpu.regs[rs2]);
        let atomic = cpu.mem.atomic64(address);
        let res = atomic.fetch_max(src, ordering(aq, rl));
        cpu.regs.set(rd, res);
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