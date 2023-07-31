

pub struct Cpu<'a> {
  mem: &'a [u8],
  regs: [u64; 32],
  pc: u64,
}

impl<'a> Cpu<'a> {
  fn step() {}
}
