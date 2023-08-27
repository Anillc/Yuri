use super::{Instructor, Funct, Instruction};

pub(crate) fn j_instructions() -> Vec<Instructor> {
  Vec::from([
    Instructor {
      name: "JAL",
      opcode: 0b1101111,
      funct: Funct::J,
      run: |inst, cpu| match inst {
        Instruction::J { imm, rd, opcode: _ } => {
          let res = cpu.pc.wrapping_add(4);
          cpu.pc = cpu.pc.wrapping_add(imm as u64);
          cpu.regs.set(rd, res);
        },
        _ => unreachable!(),
      }
    }
  ])
}
