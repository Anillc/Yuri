

pub(crate) trait Device {
  fn read(&self, address: u64) -> u8;
  fn write(&mut self, address: u64, data: u8);
}