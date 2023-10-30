use std::sync::{Mutex, atomic::{AtomicU32, AtomicU64, AtomicI32, AtomicI64, Ordering}, Arc};

use super::Device;

#[derive(Debug, Clone)]
pub(crate) struct Memory {
  mem: *mut u8,
  _boxed: Arc<Mutex<Box<[u8]>>>,
}

impl Memory {
  pub(crate) fn new(mut boxed: Box<[u8]>) -> Memory {
    Memory {
      mem: &mut boxed[0],
      _boxed: Arc::new(Mutex::new(boxed)),
    }
  }

  fn atomic_u32(&mut self, address: u64) -> &AtomicU32 {
    let ptr = self.mem.wrapping_add(address as usize) as *mut u32;
    unsafe { AtomicU32::from_ptr(ptr) }
  }

  fn atomic_u64(&mut self, address: u64) -> &AtomicU64 {
    let ptr = self.mem.wrapping_add(address as usize) as *mut u64;
    unsafe { AtomicU64::from_ptr(ptr) }
  }

  fn atomic_i32(&mut self, address: u64) -> &AtomicI32 {
    let ptr = self.mem.wrapping_add(address as usize) as *mut i32;
    unsafe { AtomicI32::from_ptr(ptr) }
  }

  fn atomic_i64(&mut self, address: u64) -> &AtomicI64 {
    let ptr = self.mem.wrapping_add(address as usize) as *mut i64;
    unsafe { AtomicI64::from_ptr(ptr) }
  }
}

impl Device for Memory {
  fn read8(&self, address: u64) -> u8 {
    unsafe { *(self.mem.wrapping_add(address as usize)) }
  }

  fn read16(&self, address: u64) -> u16 {
    u16::from_le(unsafe { *(self.mem.wrapping_add(address as usize) as *const _) })
  }

  fn read32(&self, address: u64) -> u32 {
    u32::from_le(unsafe { *(self.mem.wrapping_add(address as usize) as *const _) })
  }

  fn read64(&self, address: u64) -> u64 {
    u64::from_le(unsafe { *(self.mem.wrapping_add(address as usize) as *const _) })
  }

  fn write8(&mut self, address: u64, data: u8) {
    unsafe { *(self.mem.wrapping_add(address as usize)) = data; };
  }

  fn write16(&mut self, address: u64, data: u16) {
    unsafe { *(self.mem.wrapping_add(address as usize) as *mut _) = data.to_le(); };
  }

  fn write32(&mut self, address: u64, data: u32) {
    unsafe { *(self.mem.wrapping_add(address as usize) as *mut _) = data.to_le(); };
  }

  fn write64(&mut self, address: u64, data: u64) {
    unsafe { *(self.mem.wrapping_add(address as usize) as *mut _) = data.to_le(); };
  }

  fn atomic_swap32(&mut self, address: u64, val: u32, ordering: Ordering) -> u32 {
    let atomic = self.atomic_u32(address);
    atomic.swap(val, ordering)
  }

  fn atomic_swap64(&mut self, address: u64, val: u64, ordering: Ordering) -> u64 {
    let atomic = self.atomic_u64(address);
    atomic.swap(val, ordering)
  }

  fn atomic_add32(&mut self, address: u64, val: u32, ordering: Ordering) -> u32 {
    let atomic = self.atomic_u32(address);
    atomic.fetch_add(val, ordering)
  }

  fn atomic_add64(&mut self, address: u64, val: u64, ordering: Ordering) -> u64 {
    let atomic = self.atomic_u64(address);
    atomic.fetch_add(val, ordering)
  }

  fn atomic_xor32(&mut self, address: u64, val: u32, ordering: Ordering) -> u32 {
    let atomic = self.atomic_u32(address);
    atomic.fetch_xor(val, ordering)
  }

  fn atomic_xor64(&mut self, address: u64, val: u64, ordering: Ordering) -> u64 {
    let atomic = self.atomic_u64(address);
    atomic.fetch_xor(val, ordering)
  }

  fn atomic_and32(&mut self, address: u64, val: u32, ordering: Ordering) -> u32 {
    let atomic = self.atomic_u32(address);
    atomic.fetch_and(val, ordering)
  }

  fn atomic_and64(&mut self, address: u64, val: u64, ordering: Ordering) -> u64 {
    let atomic = self.atomic_u64(address);
    atomic.fetch_and(val, ordering)
  }

  fn atomic_or32(&mut self, address: u64, val: u32, ordering: Ordering) -> u32 {
    let atomic = self.atomic_u32(address);
    atomic.fetch_or(val, ordering)
  }

  fn atomic_or64(&mut self, address: u64, val: u64, ordering: Ordering) -> u64 {
    let atomic = self.atomic_u64(address);
    atomic.fetch_or(val, ordering)
  }

  fn atomic_min_i32(&mut self, address: u64, val: i32, ordering: Ordering) -> i32 {
    let atomic = self.atomic_i32(address);
    atomic.fetch_min(val, ordering)
  }

  fn atomic_min_i64(&mut self, address: u64, val: i64, ordering: Ordering) -> i64 {
    let atomic = self.atomic_i64(address);
    atomic.fetch_min(val, ordering)
  }

  fn atomic_max_i32(&mut self, address: u64, val: i32, ordering: Ordering) -> i32 {
    let atomic = self.atomic_i32(address);
    atomic.fetch_max(val, ordering)
  }

  fn atomic_max_i64(&mut self, address: u64, val: i64, ordering: Ordering) -> i64 {
    let atomic = self.atomic_i64(address);
    atomic.fetch_max(val, ordering)
  }

  fn atomic_min_u32(&mut self, address: u64, val: u32, ordering: Ordering) -> u32 {
    let atomic = self.atomic_u32(address);
    atomic.fetch_min(val, ordering)
  }

  fn atomic_min_u64(&mut self, address: u64, val: u64, ordering: Ordering) -> u64 {
    let atomic = self.atomic_u64(address);
    atomic.fetch_min(val, ordering)
  }

  fn atomic_max_u32(&mut self, address: u64, val: u32, ordering: Ordering) -> u32 {
    let atomic = self.atomic_u32(address);
    atomic.fetch_max(val, ordering)
  }

  fn atomic_max_u64(&mut self, address: u64, val: u64, ordering: Ordering) -> u64 {
    let atomic = self.atomic_u64(address);
    atomic.fetch_max(val, ordering)
  }
}

