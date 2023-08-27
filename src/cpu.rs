use crate::register::Registers;

pub struct Cpu<'a> {
  mem: &'a [u8],
  regs: Registers,
  pc: u64,
}

impl<'a> Cpu<'a> {
  fn step() {}
}
