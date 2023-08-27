use crate::{register::Registers, memory::Memory, csr::Csr};

pub struct Cpu<'a> {
  pub(crate) mem: Memory<'a>,
  pub(crate) regs: Registers,
  pub(crate) pc: u64,
  pub(crate) csr: Csr,
}

impl<'a> Cpu<'a> {
  pub fn new(mem: &'a mut [u8]) -> Cpu<'a> {
    Cpu { mem: Memory::new(mem), regs: Registers::new(), pc: 0, csr: Csr::new() }
  }
  fn step() {}
}
