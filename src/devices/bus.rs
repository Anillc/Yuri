use std::sync::{atomic::Ordering, Arc, Mutex};

use crate::{trap::Exception, hart::Hart, utils::channel::{Sender, Receiver}};

use super::{Device, memory::{Memory, MEMORY_START, MEMORY_END}, aclint::{Aclint, ACLINT_START, ACLINT_END}, plic::{Plic, PLIC_START, PLIC_END}, uart::{UART_START, UART_END, Uart}, ysyx::{Ysyx, YSYX_START, YSYX_END}};

#[derive(Debug, Clone)]
pub(crate) struct Bus {
  pub(crate) memory: Memory,
  pub(crate) aclint: Arc<Mutex<Aclint>>,
  pub(crate) plic: Arc<Mutex<Plic>>,
  pub(crate) uart: Arc<Mutex<Uart>>,
  pub(crate) ysyx: Arc<Mutex<Ysyx>>,
}

#[derive(Debug)]
pub(crate) struct DeviceController {
  pub(crate) uart_sender: Sender<u8>,
  pub(crate) uart_receiver: Receiver<u8>,
}

impl Bus {
  pub(crate) fn new() -> (Bus, DeviceController) {
    let (uart, sender, receiver) = Uart::new();
    (Bus {
      memory: Memory::new(),
      aclint: Arc::new(Mutex::new(Aclint::new())),
      plic: Arc::new(Mutex::new(Plic::new())),
      uart: Arc::new(Mutex::new(uart)),
      ysyx: Arc::new(Mutex::new(Ysyx::new())),
    }, DeviceController {
      uart_sender: sender,
      uart_receiver: receiver,
    })
  }
  #[inline]
  fn device_read<T, F>(&mut self, address: u64, run: F) -> Result<T, Exception>
  where
    F: for<'a> FnOnce(&'a mut dyn Device) -> Result<T, Exception>
  {
    match address {
      MEMORY_START..=MEMORY_END => Ok(run(&mut self.memory)?),
      ACLINT_START..=ACLINT_END => Ok(run(&mut *self.aclint.lock().unwrap())?),
      PLIC_START..=PLIC_END => Ok(run(&mut *self.plic.lock().unwrap())?),
      UART_START..=UART_END => Ok(run(&mut *self.uart.lock().unwrap())?),
      YSYX_START..=YSYX_END => Ok(run(&mut *self.ysyx.lock().unwrap())?),
      _ => Err(Exception::LoadAccessFault(address))
    }
  }
  #[inline]
  fn device_write<T, F>(&mut self, address: u64, run: F) -> Result<T, Exception>
  where
    F: for<'a> FnOnce(&'a mut dyn Device) -> Result<T, Exception>
  {
    match address {
      MEMORY_START..=MEMORY_END => Ok(run(&mut self.memory)?),
      ACLINT_START..=ACLINT_END => Ok(run(&mut *self.aclint.lock().unwrap())?),
      PLIC_START..=PLIC_END => Ok(run(&mut *self.plic.lock().unwrap())?),
      UART_START..=UART_END => Ok(run(&mut *self.uart.lock().unwrap())?),
      YSYX_START..=YSYX_END => Ok(run(&mut *self.ysyx.lock().unwrap())?),
      _ => Err(Exception::StoreAMOAccessFault(address))
    }
  }
}

impl Device for Bus {
  fn step(&mut self, bus: &mut Bus, hart: &mut Hart) {
    // TODO: move devices (except plic) to other threads
    self.memory.step(bus, hart);
    self.uart.lock().unwrap().step(bus, hart);
    self.ysyx.lock().unwrap().step(bus, hart);
    self.aclint.lock().unwrap().step(bus, hart);
    self.plic.lock().unwrap().step(bus, hart);
  }

  fn read8(&mut self, address: u64) -> Result<u8, Exception> { self.device_read(address, |device| device.read8(address)) }
  fn read16(&mut self, address: u64) -> Result<u16, Exception> { self.device_read(address, |device| device.read16(address)) }
  fn read32(&mut self, address: u64) -> Result<u32, Exception> { self.device_read(address, |device| device.read32(address)) }
  fn read64(&mut self, address: u64) -> Result<u64, Exception> { self.device_read(address, |device| device.read64(address)) }
  fn write8(&mut self, address: u64, data: u8) -> Result<(), Exception> { self.device_write(address, |device| device.write8(address, data)) }
  fn write16(&mut self, address: u64, data: u16) -> Result<(), Exception> { self.device_write(address, |device| device.write16(address, data)) }
  fn write32(&mut self, address: u64, data: u32) -> Result<(), Exception> { self.device_write(address, |device| device.write32(address, data)) }
  fn write64(&mut self, address: u64, data: u64) -> Result<(), Exception> { self.device_write(address, |device| device.write64(address, data)) }
  fn atomic_swap32(&mut self, address: u64, val: u32, ordering: Ordering) -> Result<u32, Exception> { self.device_write(address, |device| device.atomic_swap32(address, val, ordering)) }
  fn atomic_swap64(&mut self, address: u64, val: u64, ordering: Ordering) -> Result<u64, Exception> { self.device_write(address, |device| device.atomic_swap64(address, val, ordering)) }
  fn atomic_add32(&mut self, address: u64, val: u32, ordering: Ordering) -> Result<u32, Exception> { self.device_write(address, |device| device.atomic_add32(address, val, ordering)) }
  fn atomic_add64(&mut self, address: u64, val: u64, ordering: Ordering) -> Result<u64, Exception> { self.device_write(address, |device| device.atomic_add64(address, val, ordering)) }
  fn atomic_xor32(&mut self, address: u64, val: u32, ordering: Ordering) -> Result<u32, Exception> { self.device_write(address, |device| device.atomic_xor32(address, val, ordering)) }
  fn atomic_xor64(&mut self, address: u64, val: u64, ordering: Ordering) -> Result<u64, Exception> { self.device_write(address, |device| device.atomic_xor64(address, val, ordering)) }
  fn atomic_and32(&mut self, address: u64, val: u32, ordering: Ordering) -> Result<u32, Exception> { self.device_write(address, |device| device.atomic_and32(address, val, ordering)) }
  fn atomic_and64(&mut self, address: u64, val: u64, ordering: Ordering) -> Result<u64, Exception> { self.device_write(address, |device| device.atomic_and64(address, val, ordering)) }
  fn atomic_or32(&mut self, address: u64, val: u32, ordering: Ordering) -> Result<u32, Exception> { self.device_write(address, |device| device.atomic_or32(address, val, ordering)) }
  fn atomic_or64(&mut self, address: u64, val: u64, ordering: Ordering) -> Result<u64, Exception> { self.device_write(address, |device| device.atomic_or64(address, val, ordering)) }
  fn atomic_min_i32(&mut self, address: u64, val: i32, ordering: Ordering) -> Result<i32, Exception> { self.device_write(address, |device| device.atomic_min_i32(address, val, ordering)) }
  fn atomic_min_i64(&mut self, address: u64, val: i64, ordering: Ordering) -> Result<i64, Exception> { self.device_write(address, |device| device.atomic_min_i64(address, val, ordering)) }
  fn atomic_max_i32(&mut self, address: u64, val: i32, ordering: Ordering) -> Result<i32, Exception> { self.device_write(address, |device| device.atomic_max_i32(address, val, ordering)) }
  fn atomic_max_i64(&mut self, address: u64, val: i64, ordering: Ordering) -> Result<i64, Exception> { self.device_write(address, |device| device.atomic_max_i64(address, val, ordering)) }
  fn atomic_min_u32(&mut self, address: u64, val: u32, ordering: Ordering) -> Result<u32, Exception> { self.device_write(address, |device| device.atomic_min_u32(address, val, ordering)) }
  fn atomic_min_u64(&mut self, address: u64, val: u64, ordering: Ordering) -> Result<u64, Exception> { self.device_write(address, |device| device.atomic_min_u64(address, val, ordering)) }
  fn atomic_max_u32(&mut self, address: u64, val: u32, ordering: Ordering) -> Result<u32, Exception> { self.device_write(address, |device| device.atomic_max_u32(address, val, ordering)) }
  fn atomic_max_u64(&mut self, address: u64, val: u64, ordering: Ordering) -> Result<u64, Exception> { self.device_write(address, |device| device.atomic_max_u64(address, val, ordering)) }
}
