use super::{Instructor, Funct, Instruction};

pub(crate) fn i_instructions() -> Vec<Instructor> {
  Vec::from([
    Instructor {
      name: "JALR",
      opcode: 0b1100111,
      funct: Funct::I(0b000),
      run: |inst, cpu| match inst {
        Instruction::I { imm, rs1, funct3: _, rd, opcode: _ } => {
          let res = cpu.pc.wrapping_add(4);
          cpu.pc = cpu.regs[rs1].wrapping_add(imm as u64);
          cpu.regs.set(rd, res);
        },
        _ => unreachable!(),
      },
    }
  ])
}
