use std::sync::{Mutex, atomic::{AtomicU32, AtomicU64, AtomicI32, AtomicI64}};



pub(crate) struct Memory<'a> {
  mem: &'a mut [u8],
  reservation: Mutex<Vec<u64>>,
}

impl<'a> Memory<'a> {
  pub(crate) fn new(mem: &'a mut [u8]) -> Memory<'a> {
    Memory { mem, reservation: Mutex::new(Vec::new()) }
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

  pub(crate) fn lock_addr(&mut self, address: u64) {
    self.reservation.lock().unwrap().push(address);
  }

  // true -> exist
  // false -> non-exist
  pub(crate) fn unlock_addr(&mut self, address: u64) -> bool {
    let mut reservation = self.reservation.lock().unwrap();
    let res = reservation.contains(&address);
    reservation.clear();
    res
  }

  pub(crate) fn ptr32(&mut self, address: u64) -> *mut u32 {
    &mut self.mem[address as usize] as *mut _ as *mut u32
  }

  pub(crate) fn ptr64(&mut self, address: u64) -> *mut u64 {
    &mut self.mem[address as usize] as *mut _ as *mut u64
  }

  pub(crate) fn ptr32i(&mut self, address: u64) -> *mut i32 {
    &mut self.mem[address as usize] as *mut _ as *mut i32
  }

  pub(crate) fn ptr64i(&mut self, address: u64) -> *mut i64 {
    &mut self.mem[address as usize] as *mut _ as *mut i64
  }

  pub(crate) fn atomic32(&mut self, address: u64) -> &AtomicU32 {
    unsafe { AtomicU32::from_ptr(self.ptr32(address)) }
  }

  pub(crate) fn atomic64(&mut self, address: u64) -> &AtomicU64 {
    unsafe { AtomicU64::from_ptr(self.ptr64(address)) }
  }

  pub(crate) fn atomic32i(&mut self, address: u64) -> &AtomicI32 {
    unsafe { AtomicI32::from_ptr(self.ptr32i(address)) }
  }

  pub(crate) fn atomic64i(&mut self, address: u64) -> &AtomicI64 {
    unsafe { AtomicI64::from_ptr(self.ptr64i(address)) }
  }
}
