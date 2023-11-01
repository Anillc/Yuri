use std::sync::{atomic::Ordering, Arc, Mutex};

use crate::{trap::Exception, hart::Hart};

use super::{Device, memory::{Memory, MEMORY_START, MEMORY_END}, aclint::{Aclint, ACLINT_START, ACLINT_END}};

#[derive(Debug, Clone)]
pub(crate) struct Bus {
  memory: Memory,
  aclint: Arc<Mutex<Aclint>>,
}

impl Bus {
  pub(crate) fn new() -> Bus {
    Bus {
      memory: Memory::new(),
      aclint: Arc::new(Mutex::new(Aclint::new())),
    }
  }
  #[inline]
  fn device<T, F>(&self, address: u64, run: F) -> Result<T, Exception>
  where
    F: for<'a> FnOnce(&'a dyn Device) -> Result<T, Exception>
  {
    match address {
      MEMORY_START..=MEMORY_END => Ok(run(&self.memory)?),
      ACLINT_START..=ACLINT_END => Ok(run(&*self.aclint.lock().unwrap())?),
      _ => Err(Exception::LoadAccessFault(address))
    }
  }
  #[inline]
  fn device_mut<T, F>(&mut self, address: u64, run: F) -> Result<T, Exception>
  where
    F: for<'a> FnOnce(&'a mut dyn Device) -> Result<T, Exception>
  {
    match address {
      MEMORY_START..=MEMORY_END => Ok(run(&mut self.memory)?),
      ACLINT_START..=ACLINT_END => Ok(run(&mut *self.aclint.lock().unwrap())?),
      _ => Err(Exception::LoadAccessFault(address))
    }
  }
}

impl Device for Bus {
  fn step(&mut self, hart: &mut Hart) {
    self.memory.step(hart);
    self.aclint.lock().unwrap().step(hart);
  }
  fn read8(&self, address: u64) -> Result<u8, Exception> { self.device(address, |device| device.read8(address)) }
  fn read16(&self, address: u64) -> Result<u16, Exception> { self.device(address, |device| device.read16(address)) }
  fn read32(&self, address: u64) -> Result<u32, Exception> { self.device(address, |device| device.read32(address)) }
  fn read64(&self, address: u64) -> Result<u64, Exception> { self.device(address, |device| device.read64(address)) }
  fn write8(&mut self, address: u64, data: u8) -> Result<(), Exception> { self.device_mut(address, |device| device.write8(address, data)) }
  fn write16(&mut self, address: u64, data: u16) -> Result<(), Exception> { self.device_mut(address, |device| device.write16(address, data)) }
  fn write32(&mut self, address: u64, data: u32) -> Result<(), Exception> { self.device_mut(address, |device| device.write32(address, data)) }
  fn write64(&mut self, address: u64, data: u64) -> Result<(), Exception> { self.device_mut(address, |device| device.write64(address, data)) }
  fn atomic_swap32(&mut self, address: u64, val: u32, ordering: Ordering) -> Result<u32, Exception> { self.device_mut(address, |device| device.atomic_swap32(address, val, ordering)) }
  fn atomic_swap64(&mut self, address: u64, val: u64, ordering: Ordering) -> Result<u64, Exception> { self.device_mut(address, |device| device.atomic_swap64(address, val, ordering)) }
  fn atomic_add32(&mut self, address: u64, val: u32, ordering: Ordering) -> Result<u32, Exception> { self.device_mut(address, |device| device.atomic_add32(address, val, ordering)) }
  fn atomic_add64(&mut self, address: u64, val: u64, ordering: Ordering) -> Result<u64, Exception> { self.device_mut(address, |device| device.atomic_add64(address, val, ordering)) }
  fn atomic_xor32(&mut self, address: u64, val: u32, ordering: Ordering) -> Result<u32, Exception> { self.device_mut(address, |device| device.atomic_xor32(address, val, ordering)) }
  fn atomic_xor64(&mut self, address: u64, val: u64, ordering: Ordering) -> Result<u64, Exception> { self.device_mut(address, |device| device.atomic_xor64(address, val, ordering)) }
  fn atomic_and32(&mut self, address: u64, val: u32, ordering: Ordering) -> Result<u32, Exception> { self.device_mut(address, |device| device.atomic_and32(address, val, ordering)) }
  fn atomic_and64(&mut self, address: u64, val: u64, ordering: Ordering) -> Result<u64, Exception> { self.device_mut(address, |device| device.atomic_and64(address, val, ordering)) }
  fn atomic_or32(&mut self, address: u64, val: u32, ordering: Ordering) -> Result<u32, Exception> { self.device_mut(address, |device| device.atomic_or32(address, val, ordering)) }
  fn atomic_or64(&mut self, address: u64, val: u64, ordering: Ordering) -> Result<u64, Exception> { self.device_mut(address, |device| device.atomic_or64(address, val, ordering)) }
  fn atomic_min_i32(&mut self, address: u64, val: i32, ordering: Ordering) -> Result<i32, Exception> { self.device_mut(address, |device| device.atomic_min_i32(address, val, ordering)) }
  fn atomic_min_i64(&mut self, address: u64, val: i64, ordering: Ordering) -> Result<i64, Exception> { self.device_mut(address, |device| device.atomic_min_i64(address, val, ordering)) }
  fn atomic_max_i32(&mut self, address: u64, val: i32, ordering: Ordering) -> Result<i32, Exception> { self.device_mut(address, |device| device.atomic_max_i32(address, val, ordering)) }
  fn atomic_max_i64(&mut self, address: u64, val: i64, ordering: Ordering) -> Result<i64, Exception> { self.device_mut(address, |device| device.atomic_max_i64(address, val, ordering)) }
  fn atomic_min_u32(&mut self, address: u64, val: u32, ordering: Ordering) -> Result<u32, Exception> { self.device_mut(address, |device| device.atomic_min_u32(address, val, ordering)) }
  fn atomic_min_u64(&mut self, address: u64, val: u64, ordering: Ordering) -> Result<u64, Exception> { self.device_mut(address, |device| device.atomic_min_u64(address, val, ordering)) }
  fn atomic_max_u32(&mut self, address: u64, val: u32, ordering: Ordering) -> Result<u32, Exception> { self.device_mut(address, |device| device.atomic_max_u32(address, val, ordering)) }
  fn atomic_max_u64(&mut self, address: u64, val: u64, ordering: Ordering) -> Result<u64, Exception> { self.device_mut(address, |device| device.atomic_max_u64(address, val, ordering)) }
}