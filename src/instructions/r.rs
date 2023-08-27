use super::{Instructor, Funct};

pub(crate) fn r_instructions() -> Vec<Instructor> {
  vec![
    Instructor {
      name: "add",
      opcode: 0b0110011,
      funct: Funct::R(0, 0),
      run: |inst, cpu| {}
    },
  ]
}
