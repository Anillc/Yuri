use crate::{register::{Registers, FRegisters}, memory::Memory, csr::Csr, instructions::parse};

pub struct Cpu<'a> {
  pub(crate) mem: Memory<'a>,
  pub(crate) regs: Registers,
  pub(crate) fregs: FRegisters,
  pub(crate) pc: u64,
  pub(crate) csr: Csr,
}

impl<'a> Cpu<'a> {
  pub fn new(mem: &'a mut [u8]) -> Cpu<'a> {
    Cpu {
      mem: Memory::new(mem),
      regs: Registers::new(),
      fregs: FRegisters::new(),
      pc: 0,
      csr: Csr::new(),
    }
  }
  pub(crate) fn step(&mut self) {
    let inst = self.mem.read32(self.pc);
    let instructor = parse(inst);
    (instructor.run)(inst, self);
    self.pc += 4;
  }
}
