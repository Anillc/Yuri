use std::sync::{Mutex, atomic::{AtomicU32, AtomicU64, AtomicI32, AtomicI64}, Arc};

#[derive(Debug, Clone)]
pub(crate) struct Memory {
  mem: *mut u8,
  reservation: Arc<Mutex<Vec<u64>>>,
  _boxed: Arc<Mutex<Box<[u8]>>>,
}

impl Memory {
  pub(crate) fn new(mut boxed: Box<[u8]>) -> Memory {
    Memory {
      mem: &mut boxed[0],
      reservation: Arc::new(Mutex::new(Vec::new())),
      _boxed: Arc::new(Mutex::new(boxed)),
    }
  }

  pub(crate) fn read8(&self, address: u64) -> u8 {
    unsafe { *(self.mem.wrapping_add(address as usize)) }
  }

  pub(crate) fn read16(&self, address: u64) -> u16 {
    u16::from_le(unsafe { *(self.mem.wrapping_add(address as usize) as *const _) })
  }

  pub(crate) fn read32(&self, address: u64) -> u32 {
    u32::from_le(unsafe { *(self.mem.wrapping_add(address as usize) as *const _) })
  }

  pub(crate) fn read64(&self, address: u64) -> u64 {
    u64::from_le(unsafe { *(self.mem.wrapping_add(address as usize) as *const _) })
  }

  pub(crate) fn write8(&mut self, address: u64, data: u8) {
    unsafe { *(self.mem.wrapping_add(address as usize)) = data; };
  }

  pub(crate) fn write16(&mut self, address: u64, data: u16) {
    unsafe { *(self.mem.wrapping_add(address as usize) as *mut _) = data.to_le(); };
  }

  pub(crate) fn write32(&mut self, address: u64, data: u32) {
    unsafe { *(self.mem.wrapping_add(address as usize) as *mut _) = data.to_le(); };
  }

  pub(crate) fn write64(&mut self, address: u64, data: u64) {
    unsafe { *(self.mem.wrapping_add(address as usize) as *mut _) = data.to_le(); };
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
    self.mem.wrapping_add(address as usize) as *mut u32
  }

  pub(crate) fn ptr64(&mut self, address: u64) -> *mut u64 {
    self.mem.wrapping_add(address as usize) as *mut u64
  }

  pub(crate) fn ptr32i(&mut self, address: u64) -> *mut i32 {
    self.mem.wrapping_add(address as usize) as *mut i32
  }

  pub(crate) fn ptr64i(&mut self, address: u64) -> *mut i64 {
    self.mem.wrapping_add(address as usize) as *mut i64
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
