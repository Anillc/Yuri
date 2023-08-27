use super::{Instructor, Funct, Instruction};

pub(crate) fn j_instructions() -> Vec<Instructor> {
  Vec::from([
    Instructor{
      name: "JAL",
      opcode: 0b1101111,
      funct: Funct::J,
      run: |inst, cpu| match inst {
        Instruction::J { imm, rd, opcode: _ } => {
          cpu.regs.set(rd, cpu.pc.wrapping_add(4));
          cpu.pc = cpu.pc.wrapping_add(imm as i32 as i64 as u64);
        },
        _ => unreachable!(),
      }
    }
  ])
}
