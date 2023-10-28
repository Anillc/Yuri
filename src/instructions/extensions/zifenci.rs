use crate::instructions::Instructor;

use super::funct3;

pub(crate) fn zifenci() -> Vec<Instructor> {
  Vec::from([
    Instructor {
      name: "FENCE.I",
      opcode: 0b0001111,
      segments: funct3(0b001),
      run: |_inst, _len, _hart| {
        // do nothing
        Ok(())
      },
    },
  ])
}