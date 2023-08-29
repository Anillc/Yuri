use std::ops::Index;

pub(crate) struct Registers {
  regs: [u64; 32],
}

// TODO: check overflow
impl Registers {
  pub(crate) fn new() -> Registers {
    Registers { regs: [0; 32] }
  }

  pub(crate) fn set(&mut self, index: usize, value: u64) {
    if index == 0 { return; }
    self.regs[index] = value;
  }
}

impl Index<usize> for Registers {
  type Output = u64;

  fn index(&self, index: usize) -> &Self::Output {
    &self.regs[index]
  }
}

pub(crate) struct FRegisters {
  regs: [f64; 32],
}

impl FRegisters {
  pub(crate) fn new() -> FRegisters {
    FRegisters { regs: [0f64; 32] }
  }

  pub(crate) fn set(&mut self, index: usize, value: f64) {
    self.regs[index] = value;
  }
}

impl Index<usize> for FRegisters {
  type Output = f64;

  fn index(&self, index: usize) -> &Self::Output {
    &self.regs[index]
  }
}
