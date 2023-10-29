// Machine-Mode and Supervisor-Mode Privileged Instructions
use crate::instructions::{Instructor, InstructionSegment};

pub(crate) fn sm() -> Vec<Instructor> {
  Vec::from([
    Instructor {
      name: "SRET",
      opcode: 0b1110011,
      segments: vec![
        InstructionSegment { start: 7, end: 31, comp: 0b0001000000100000000000000 },
      ],
      run: |_inst, _len, hart| {
        Ok(())
      }
    },

    Instructor {
      name: "MRET",
      opcode: 0b1110011,
      segments: vec![
        InstructionSegment { start: 7, end: 31, comp: 0b0011000000100000000000000 },
      ],
      run: |_inst, _len, hart| {
        Ok(())
      }
    },

    Instructor {
      name: "WFI",
      opcode: 0b1110011,
      segments: vec![
        InstructionSegment { start: 7, end: 31, comp: 0b0001000001010000000000000 },
      ],
      run: |_inst, _len, hart| {
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
      run: |_inst, _len, _hart| {
        // do nothing
        Ok(())
      }
    },
  ])
}