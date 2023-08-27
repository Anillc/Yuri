

pub(crate) struct Memory<'a> {
  mem: &'a [u8],
}

impl<'a> Memory<'a> {
  pub(crate) fn new(mem: &'a [u8]) -> Memory<'a> {
    Memory { mem }
  }

  pub(crate) fn read8(&self, address: u64) -> u8 {
    self.mem[address as usize]
  }

  pub(crate) fn read16(&self, address: u64) -> u16 {
    let address = address as usize;
    self.mem[address] as u16 | (self.mem[address + 1] << 8) as u16
  }

  pub(crate) fn read32(&self, address: u64) -> u32 {
    let address = address as usize;
    self.mem[address] as u32
      | self.mem[address + 1] as u32
      | self.mem[address + 2] as u32
      | self.mem[address + 3] as u32
  }

  pub(crate) fn read64(&self, address: u64) -> u64 {
    let address = address as usize;
    self.mem[address] as u64
      | self.mem[address + 1] as u64
      | self.mem[address + 2] as u64
      | self.mem[address + 3] as u64
      | self.mem[address + 4] as u64
      | self.mem[address + 5] as u64
      | self.mem[address + 6] as u64
      | self.mem[address + 7] as u64
  }
}