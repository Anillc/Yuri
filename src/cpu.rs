use crate::register::Registers;

pub struct Cpu<'a> {
  pub(crate) mem: &'a mut [u8],
  pub(crate) regs: Registers,
  pub(crate) pc: u64,
}

impl<'a> Cpu<'a> {
  pub fn new(mem: &'a mut [u8]) -> Cpu<'a> {
    Cpu { mem, regs: Registers::new(), pc: 0 }
  }
  fn step() {}
}
