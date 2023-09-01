use crate::instructions::{Instructor, InstructorResult};

use super::funct3;

pub(crate) fn zicsr() -> Vec<Instructor> {
  Vec::from([
    Instructor {
      name: "CSRRW",
      opcode: 0b1110011,
      segments: funct3(0b001),
      run: |inst, cpu| {
        let imm = (inst >> 20) as u16;
        let rs1 = (inst >> 15 & 0b11111) as usize;
        let rd = (inst >> 7 & 0b11111) as usize;
        let res = cpu.csr.read(imm);
        cpu.csr.write(imm, cpu.regs[rs1]);
        cpu.regs.set(rd, res);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "CSRRS",
      opcode: 0b1110011,
      segments: funct3(0b010),
      run: |inst, cpu| {
        let imm = (inst >> 20) as u16;
        let rs1 = (inst >> 15 & 0b11111) as usize;
        let rd = (inst >> 7 & 0b11111) as usize;
        let res = cpu.csr.read(imm);
        cpu.csr.write(imm, res | cpu.regs[rs1]);
        cpu.regs.set(rd, res);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "CSRRC",
      opcode: 0b1110011,
      segments: funct3(0b011),
      run: |inst, cpu| {
        let imm = (inst >> 20) as u16;
        let rs1 = (inst >> 15 & 0b11111) as usize;
        let rd = (inst >> 7 & 0b11111) as usize;
        let res = cpu.csr.read(imm);
        cpu.csr.write(imm, res & !cpu.regs[rs1]);
        cpu.regs.set(rd, res);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "CSRRWI",
      opcode: 0b1110011,
      segments: funct3(0b101),
      run: |inst, cpu| {
        let imm = (inst >> 20) as u16;
        let rs1 = (inst >> 15 & 0b11111) as u64;
        let rd = (inst >> 7 & 0b11111) as usize;
        cpu.regs.set(rd, cpu.csr.read(imm));
        cpu.csr.write(imm, rs1);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "CSRRSI",
      opcode: 0b1110011,
      segments: funct3(0b110),
      run: |inst, cpu| {
        let imm = (inst >> 20) as u16;
        let rs1 = (inst >> 15 & 0b11111) as u64;
        let rd = (inst >> 7 & 0b11111) as usize;
        let res = cpu.csr.read(imm);
        cpu.csr.write(imm, res | rs1);
        cpu.regs.set(rd, res);
        InstructorResult::Success
      },
    },

    Instructor {
      name: "CSRRCI",
      opcode: 0b1110011,
      segments: funct3(0b111),
      run: |inst, cpu| {
        let imm = (inst >> 20) as u16;
        let rs1 = (inst >> 15 & 0b11111) as u64;
        let rd = (inst >> 7 & 0b11111) as usize;
        let res = cpu.csr.read(imm);
        cpu.csr.write(imm as u16, res & !rs1);
        cpu.regs.set(rd, res);
        InstructorResult::Success
      },
    },
  ])
}