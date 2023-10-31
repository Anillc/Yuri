use std::sync::atomic::Ordering;

use crate::trap::Exception;

use super::{Device, memory::{Memory, MEMORY_START, MEMORY_END}};

#[derive(Debug, Clone)]
pub(crate) struct Bus {
  memory: Memory,
}

impl Bus {
  pub(crate) fn new() -> Bus {
    Bus {
      memory: Memory::new(),
    }
  }
  #[inline]
  fn device(&self, address: u64) -> Result<&dyn Device, Exception> {
    match address {
      MEMORY_START..=MEMORY_END => Ok(&self.memory),
      _ => Err(Exception::LoadAccessFault(address))
    }
  }
  #[inline]
  fn device_mut(&mut self, address: u64) -> Result<&mut dyn Device, Exception> {
    match address {
      MEMORY_START..=MEMORY_END => Ok(&mut self.memory),
      _ => Err(Exception::LoadAccessFault(address))
    }
  }
}

impl Device for Bus {
  fn read8(&self, address: u64) -> Result<u8, Exception> { self.device(address)?.read8(address) }
  fn read16(&self, address: u64) -> Result<u16, Exception> { self.device(address)?.read16(address) }
  fn read32(&self, address: u64) -> Result<u32, Exception> { self.device(address)?.read32(address) }
  fn read64(&self, address: u64) -> Result<u64, Exception> { self.device(address)?.read64(address) }
  fn write8(&mut self, address: u64, data: u8) -> Result<(), Exception> { self.device_mut(address)?.write8(address, data) }
  fn write16(&mut self, address: u64, data: u16) -> Result<(), Exception> { self.device_mut(address)?.write16(address, data) }
  fn write32(&mut self, address: u64, data: u32) -> Result<(), Exception> { self.device_mut(address)?.write32(address, data) }
  fn write64(&mut self, address: u64, data: u64) -> Result<(), Exception> { self.device_mut(address)?.write64(address, data) }
  fn atomic_swap32(&mut self, address: u64, val: u32, ordering: Ordering) -> Result<u32, Exception> { self.device_mut(address)?.atomic_swap32(address, val, ordering) }
  fn atomic_swap64(&mut self, address: u64, val: u64, ordering: Ordering) -> Result<u64, Exception> { self.device_mut(address)?.atomic_swap64(address, val, ordering) }
  fn atomic_add32(&mut self, address: u64, val: u32, ordering: Ordering) -> Result<u32, Exception> { self.device_mut(address)?.atomic_add32(address, val, ordering) }
  fn atomic_add64(&mut self, address: u64, val: u64, ordering: Ordering) -> Result<u64, Exception> { self.device_mut(address)?.atomic_add64(address, val, ordering) }
  fn atomic_xor32(&mut self, address: u64, val: u32, ordering: Ordering) -> Result<u32, Exception> { self.device_mut(address)?.atomic_xor32(address, val, ordering) }
  fn atomic_xor64(&mut self, address: u64, val: u64, ordering: Ordering) -> Result<u64, Exception> { self.device_mut(address)?.atomic_xor64(address, val, ordering) }
  fn atomic_and32(&mut self, address: u64, val: u32, ordering: Ordering) -> Result<u32, Exception> { self.device_mut(address)?.atomic_and32(address, val, ordering) }
  fn atomic_and64(&mut self, address: u64, val: u64, ordering: Ordering) -> Result<u64, Exception> { self.device_mut(address)?.atomic_and64(address, val, ordering) }
  fn atomic_or32(&mut self, address: u64, val: u32, ordering: Ordering) -> Result<u32, Exception> { self.device_mut(address)?.atomic_or32(address, val, ordering) }
  fn atomic_or64(&mut self, address: u64, val: u64, ordering: Ordering) -> Result<u64, Exception> { self.device_mut(address)?.atomic_or64(address, val, ordering) }
  fn atomic_min_i32(&mut self, address: u64, val: i32, ordering: Ordering) -> Result<i32, Exception> { self.device_mut(address)?.atomic_min_i32(address, val, ordering) }
  fn atomic_min_i64(&mut self, address: u64, val: i64, ordering: Ordering) -> Result<i64, Exception> { self.device_mut(address)?.atomic_min_i64(address, val, ordering) }
  fn atomic_max_i32(&mut self, address: u64, val: i32, ordering: Ordering) -> Result<i32, Exception> { self.device_mut(address)?.atomic_max_i32(address, val, ordering) }
  fn atomic_max_i64(&mut self, address: u64, val: i64, ordering: Ordering) -> Result<i64, Exception> { self.device_mut(address)?.atomic_max_i64(address, val, ordering) }
  fn atomic_min_u32(&mut self, address: u64, val: u32, ordering: Ordering) -> Result<u32, Exception> { self.device_mut(address)?.atomic_min_u32(address, val, ordering) }
  fn atomic_min_u64(&mut self, address: u64, val: u64, ordering: Ordering) -> Result<u64, Exception> { self.device_mut(address)?.atomic_min_u64(address, val, ordering) }
  fn atomic_max_u32(&mut self, address: u64, val: u32, ordering: Ordering) -> Result<u32, Exception> { self.device_mut(address)?.atomic_max_u32(address, val, ordering) }
  fn atomic_max_u64(&mut self, address: u64, val: u64, ordering: Ordering) -> Result<u64, Exception> { self.device_mut(address)?.atomic_max_u64(address, val, ordering) }
}