use crate::{instructions::Instructor, csrs::CsrRegistry};

use super::funct3;

pub(crate) fn zicsr() -> Vec<Instructor> {
  Vec::from([
    Instructor {
      name: "CSRRW",
      opcode: 0b1110011,
      segments: funct3(0b001),
      run: |inst, _len, _mmu, hart| {
        let imm = (inst >> 20) as u16;
        let rs1 = (inst >> 15 & 0b11111) as usize;
        let rd = (inst >> 7 & 0b11111) as usize;
        if rd != 0 {
          hart.regs.set(rd, CsrRegistry::read(hart, imm)?);
        }
        CsrRegistry::write(hart, imm, hart.regs[rs1])?;
        Ok(())
      },
    },

    Instructor {
      name: "CSRRS",
      opcode: 0b1110011,
      segments: funct3(0b010),
      run: |inst, _len, _mmu, hart| {
        let imm = (inst >> 20) as u16;
        let rs1 = (inst >> 15 & 0b11111) as usize;
        let rd = (inst >> 7 & 0b11111) as usize;
        let res = CsrRegistry::read(hart, imm)?;
        hart.regs.set(rd, res);
        if rs1 != 0 {
          CsrRegistry::write(hart, imm, res | hart.regs[rs1])?;
        }
        Ok(())
      },
    },

    Instructor {
      name: "CSRRC",
      opcode: 0b1110011,
      segments: funct3(0b011),
      run: |inst, _len, _mmu, hart| {
        let imm = (inst >> 20) as u16;
        let rs1 = (inst >> 15 & 0b11111) as usize;
        let rd = (inst >> 7 & 0b11111) as usize;
        let res = CsrRegistry::read(hart, imm)?;
        hart.regs.set(rd, res);
        if rs1 != 0 {
          CsrRegistry::write(hart, imm, res & !hart.regs[rs1])?;
        }
        Ok(())
      },
    },

    Instructor {
      name: "CSRRWI",
      opcode: 0b1110011,
      segments: funct3(0b101),
      run: |inst, _len, _mmu, hart| {
        let imm = (inst >> 20) as u16;
        let rs1 = (inst >> 15 & 0b11111) as u64;
        let rd = (inst >> 7 & 0b11111) as usize;
        if rd != 0 {
          hart.regs.set(rd, CsrRegistry::read(hart, imm)?);
        }
        CsrRegistry::write(hart, imm, rs1)?;
        Ok(())
      },
    },

    Instructor {
      name: "CSRRSI",
      opcode: 0b1110011,
      segments: funct3(0b110),
      run: |inst, _len, _mmu, hart| {
        let imm = (inst >> 20) as u16;
        let rs1 = (inst >> 15 & 0b11111) as u64;
        let rd = (inst >> 7 & 0b11111) as usize;
        let res = CsrRegistry::read(hart, imm)?;
        hart.regs.set(rd, res);
        if imm != 0 {
          CsrRegistry::write(hart, imm, res | rs1)?;
        }
        Ok(())
      },
    },

    Instructor {
      name: "CSRRCI",
      opcode: 0b1110011,
      segments: funct3(0b111),
      run: |inst, _len, _mmu, hart| {
        let imm = (inst >> 20) as u16;
        let rs1 = (inst >> 15 & 0b11111) as u64;
        let rd = (inst >> 7 & 0b11111) as usize;
        let res = CsrRegistry::read(hart, imm)?;
        hart.regs.set(rd, res);
        if imm != 0 {
          CsrRegistry::write(hart, imm, res & !rs1)?;
        }
        Ok(())
      },
    },
  ])
}
