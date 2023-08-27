use crate::{register::Registers, memory::Memory};

pub struct Cpu<'a> {
  pub(crate) mem: Memory<'a>,
  pub(crate) regs: Registers,
  pub(crate) pc: u64,
}

impl<'a> Cpu<'a> {
  pub fn new(mem: &'a mut [u8]) -> Cpu<'a> {
    Cpu { mem: Memory::new(mem), regs: Registers::new(), pc: 0 }
  }
  fn step() {}
}
