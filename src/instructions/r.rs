use std::collections::HashMap;
use super::{Instructor, InstructionType};

pub(crate) fn r_instructions() -> HashMap<u32, Instructor> {
  let mut map = HashMap::new();

  map.insert(0b00000000000000000000000000110011, Instructor {
    opcode: 0b0110011,
    funct: 0,
    inst_type: InstructionType::R,
    name: "add",
    run: |inst| {
      
    }
  });

  map
}
