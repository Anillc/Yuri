use std::sync::{Mutex, atomic::{AtomicU32, AtomicU64, AtomicI32, AtomicI64, Ordering}, Arc};

use crate::{trap::Exception, hart::Hart};

use super::{Device, bus::Bus};

pub(crate) const MEMORY_SIZE: usize = 1024 * 1024;
pub(crate) const MEMORY_START: u64 = 0x80000000;
pub(crate) const MEMORY_END: u64 = MEMORY_START + MEMORY_SIZE as u64 - 1;

#[derive(Debug, Clone)]
pub(crate) struct Memory {
  mem: *mut u8,
  _boxed: Arc<Mutex<Box<[u8]>>>,
}

impl Memory {
  pub(crate) fn new() -> Memory {
    let mut mem: Box<[u8]> = vec![0; MEMORY_SIZE].into_boxed_slice();
    Memory {
      mem: &mut mem[0],
      _boxed: Arc::new(Mutex::new(mem)),
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
  fn step(&mut self, _bus: &mut Bus, _hart: &mut Hart) {}

  fn read8(&mut self, address: u64) -> Result<u8, Exception> {
    let address = address - MEMORY_START;
    Ok(unsafe { *(self.mem.wrapping_add(address as usize)) })
  }

  fn read16(&mut self, address: u64) -> Result<u16, Exception> {
    let address = address - MEMORY_START;
    Ok(u16::from_le(unsafe { *(self.mem.wrapping_add(address as usize) as *const _) }))
  }

  fn read32(&mut self, address: u64) -> Result<u32, Exception> {
    let address = address - MEMORY_START;
    Ok(u32::from_le(unsafe { *(self.mem.wrapping_add(address as usize) as *const _) }))
  }

  fn read64(&mut self, address: u64) -> Result<u64, Exception> {
    let address = address - MEMORY_START;
    Ok(u64::from_le(unsafe { *(self.mem.wrapping_add(address as usize) as *const _) }))
  }

  fn write8(&mut self, address: u64, data: u8) -> Result<(), Exception> {
    let address = address - MEMORY_START;
    unsafe { *(self.mem.wrapping_add(address as usize)) = data; };
    Ok(())
  }

  fn write16(&mut self, address: u64, data: u16) -> Result<(), Exception> {
    let address = address - MEMORY_START;
    unsafe { *(self.mem.wrapping_add(address as usize) as *mut _) = data.to_le(); };
    Ok(())
  }

  fn write32(&mut self, address: u64, data: u32) -> Result<(), Exception> {
    let address = address - MEMORY_START;
    unsafe { *(self.mem.wrapping_add(address as usize) as *mut _) = data.to_le(); };
    Ok(())
  }

  fn write64(&mut self, address: u64, data: u64) -> Result<(), Exception> {
    let address = address - MEMORY_START;
    unsafe { *(self.mem.wrapping_add(address as usize) as *mut _) = data.to_le(); };
    Ok(())
  }

  fn atomic_swap32(&mut self, address: u64, val: u32, ordering: Ordering) -> Result<u32, Exception> {
    let address = address - MEMORY_START;
    let atomic = self.atomic_u32(address);
    Ok(atomic.swap(val, ordering))
  }

  fn atomic_swap64(&mut self, address: u64, val: u64, ordering: Ordering) -> Result<u64, Exception> {
    let address = address - MEMORY_START;
    let atomic = self.atomic_u64(address);
    Ok(atomic.swap(val, ordering))
  }

  fn atomic_add32(&mut self, address: u64, val: u32, ordering: Ordering) -> Result<u32, Exception> {
    let address = address - MEMORY_START;
    let atomic = self.atomic_u32(address);
    Ok(atomic.fetch_add(val, ordering))
  }

  fn atomic_add64(&mut self, address: u64, val: u64, ordering: Ordering) -> Result<u64, Exception> {
    let address = address - MEMORY_START;
    let atomic = self.atomic_u64(address);
    Ok(atomic.fetch_add(val, ordering))
  }

  fn atomic_xor32(&mut self, address: u64, val: u32, ordering: Ordering) -> Result<u32, Exception> {
    let address = address - MEMORY_START;
    let atomic = self.atomic_u32(address);
    Ok(atomic.fetch_xor(val, ordering))
  }

  fn atomic_xor64(&mut self, address: u64, val: u64, ordering: Ordering) -> Result<u64, Exception> {
    let address = address - MEMORY_START;
    let atomic = self.atomic_u64(address);
    Ok(atomic.fetch_xor(val, ordering))
  }

  fn atomic_and32(&mut self, address: u64, val: u32, ordering: Ordering) -> Result<u32, Exception> {
    let address = address - MEMORY_START;
    let atomic = self.atomic_u32(address);
    Ok(atomic.fetch_and(val, ordering))
  }

  fn atomic_and64(&mut self, address: u64, val: u64, ordering: Ordering) -> Result<u64, Exception> {
    let address = address - MEMORY_START;
    let atomic = self.atomic_u64(address);
    Ok(atomic.fetch_and(val, ordering))
  }

  fn atomic_or32(&mut self, address: u64, val: u32, ordering: Ordering) -> Result<u32, Exception> {
    let address = address - MEMORY_START;
    let atomic = self.atomic_u32(address);
    Ok(atomic.fetch_or(val, ordering))
  }

  fn atomic_or64(&mut self, address: u64, val: u64, ordering: Ordering) -> Result<u64, Exception> {
    let address = address - MEMORY_START;
    let atomic = self.atomic_u64(address);
    Ok(atomic.fetch_or(val, ordering))
  }

  fn atomic_min_i32(&mut self, address: u64, val: i32, ordering: Ordering) -> Result<i32, Exception> {
    let address = address - MEMORY_START;
    let atomic = self.atomic_i32(address);
    Ok(atomic.fetch_min(val, ordering))
  }

  fn atomic_min_i64(&mut self, address: u64, val: i64, ordering: Ordering) -> Result<i64, Exception> {
    let address = address - MEMORY_START;
    let atomic = self.atomic_i64(address);
    Ok(atomic.fetch_min(val, ordering))
  }

  fn atomic_max_i32(&mut self, address: u64, val: i32, ordering: Ordering) -> Result<i32, Exception> {
    let address = address - MEMORY_START;
    let atomic = self.atomic_i32(address);
    Ok(atomic.fetch_max(val, ordering))
  }

  fn atomic_max_i64(&mut self, address: u64, val: i64, ordering: Ordering) -> Result<i64, Exception> {
    let address = address - MEMORY_START;
    let atomic = self.atomic_i64(address);
    Ok(atomic.fetch_max(val, ordering))
  }

  fn atomic_min_u32(&mut self, address: u64, val: u32, ordering: Ordering) -> Result<u32, Exception> {
    let address = address - MEMORY_START;
    let atomic = self.atomic_u32(address);
    Ok(atomic.fetch_min(val, ordering))
  }

  fn atomic_min_u64(&mut self, address: u64, val: u64, ordering: Ordering) -> Result<u64, Exception> {
    let address = address - MEMORY_START;
    let atomic = self.atomic_u64(address);
    Ok(atomic.fetch_min(val, ordering))
  }

  fn atomic_max_u32(&mut self, address: u64, val: u32, ordering: Ordering) -> Result<u32, Exception> {
    let address = address - MEMORY_START;
    let atomic = self.atomic_u32(address);
    Ok(atomic.fetch_max(val, ordering))
  }

  fn atomic_max_u64(&mut self, address: u64, val: u64, ordering: Ordering) -> Result<u64, Exception> {
    let address = address - MEMORY_START;
    let atomic = self.atomic_u64(address);
    Ok(atomic.fetch_max(val, ordering))
  }
}

