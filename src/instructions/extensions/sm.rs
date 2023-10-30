// Machine-Mode and Supervisor-Mode Privileged Instructions
use crate::{instructions::{Instructor, InstructionSegment}, hart::Mode, trap::Exception};

pub(crate) fn sm() -> Vec<Instructor> {
  Vec::from([
    Instructor {
      name: "SRET",
      opcode: 0b1110011,
      segments: vec![
        InstructionSegment { start: 7, end: 31, comp: 0b0001000000100000000000000 },
      ],
      run: |_inst, _len, _mmu, hart| {
        if hart.mode.as_u8() < Mode::Supervisor.as_u8() {
          return Err(Exception::IllegalInstruction);
        }
        if hart.mode == Mode::Supervisor && hart.csr.read_mstatus_tsr() {
          return Err(Exception::IllegalInstruction);
        }
        let (pc, mode) = hart.csr.sret();
        hart.pc = pc;
        hart.mode = mode;
        Ok(())
      }
    },

    Instructor {
      name: "MRET",
      opcode: 0b1110011,
      segments: vec![
        InstructionSegment { start: 7, end: 31, comp: 0b0011000000100000000000000 },
      ],
      run: |_inst, _len, _mmu, hart| {
        if hart.mode.as_u8() < Mode::Machine.as_u8() {
          return Err(Exception::IllegalInstruction);
        }
        let (pc, mode) = hart.csr.mret();
        hart.pc = pc;
        hart.mode = mode;
        Ok(())
      }
    },

    Instructor {
      name: "WFI",
      opcode: 0b1110011,
      segments: vec![
        InstructionSegment { start: 7, end: 31, comp: 0b0001000001010000000000000 },
      ],
      run: |_inst, _len, _mmu, hart| {
        hart.wfi = true;
        Ok(())
      }
    },

    Instructor {
      name: "SFENCE.VMA",
      opcode: 0b1110011,
      segments: vec![
        InstructionSegment { start: 7, end: 14, comp: 0b00000000 },
        InstructionSegment { start: 25, end: 31, comp: 0b0001001 },
      ],
      run: |_inst, _len, _mmu, _hart| {
        // do nothing
        Ok(())
      }
    },
  ])
}