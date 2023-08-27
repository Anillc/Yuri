

pub(crate) struct Memory<'a> {
  mem: &'a mut [u8],
}

impl<'a> Memory<'a> {
  pub(crate) fn new(mem: &'a mut [u8]) -> Memory<'a> {
    Memory { mem }
  }

  pub(crate) fn read8(&self, address: u64) -> u8 {
    let address = address as usize;
    self.mem[address]
  }

  pub(crate) fn read16(&self, address: u64) -> u16 {
    let address = address as usize;
    self.mem[address] as u16 | (self.mem[address + 1] as u16) << 8
  }

  pub(crate) fn read32(&self, address: u64) -> u32 {
    let address = address as usize;
    self.mem[address] as u32
      | (self.mem[address + 1] as u32) << 8
      | (self.mem[address + 2] as u32) << 16
      | (self.mem[address + 3] as u32) << 24
  }

  pub(crate) fn read64(&self, address: u64) -> u64 {
    let address = address as usize;
    self.mem[address] as u64
      | (self.mem[address + 1] as u64) << 8
      | (self.mem[address + 2] as u64) << 16
      | (self.mem[address + 3] as u64) << 24
      | (self.mem[address + 4] as u64) << 32
      | (self.mem[address + 5] as u64) << 40
      | (self.mem[address + 6] as u64) << 48
      | (self.mem[address + 7] as u64) << 56
  }

  pub(crate) fn write8(&mut self, address: u64, data: u8) {
    let address = address as usize;
    self.mem[address] = data;
  }

  pub(crate) fn write16(&mut self, address: u64, data: u16) {
    let address = address as usize;
    let bytes = u16::to_le_bytes(data);
    self.mem[address] = bytes[0];
    self.mem[address + 1] = bytes[1];
  }

  pub(crate) fn write32(&mut self, address: u64, data: u32) {
    let address = address as usize;
    let bytes = u32::to_le_bytes(data);
    self.mem[address] = bytes[0];
    self.mem[address + 1] = bytes[1];
    self.mem[address + 2] = bytes[2];
    self.mem[address + 3] = bytes[3];
  }

  pub(crate) fn write64(&mut self, address: u64, data: u64) {
    let address = address as usize;
    let bytes = u64::to_le_bytes(data);
    self.mem[address] = bytes[0];
    self.mem[address + 1] = bytes[1];
    self.mem[address + 2] = bytes[2];
    self.mem[address + 3] = bytes[3];
    self.mem[address + 4] = bytes[4];
    self.mem[address + 5] = bytes[5];
    self.mem[address + 6] = bytes[6];
    self.mem[address + 7] = bytes[7];
  }
}
