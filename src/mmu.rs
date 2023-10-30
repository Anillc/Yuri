use std::sync::{Arc, Mutex, atomic::Ordering};

use crate::{devices::{bus::Bus, Device}, hart::Hart};

#[derive(Debug, Clone)]
pub(crate) struct MMU {
  bus: Bus,
  reservation: Arc<Mutex<Vec<u64>>>,
}

impl MMU {
  pub(crate) fn new(bus: Bus) -> MMU {
    MMU {
      bus,
      reservation: Arc::new(Mutex::new(Vec::new())),
    }
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

  pub(crate) fn translate(&self, _address: u64, _hart: &Hart) -> u64 {
    todo!()
  }

  pub(crate) fn read8(&self, hart: &Hart, address: u64) -> u8 { self.bus.read8(self.translate(address, hart)) }
  pub(crate) fn read16(&self, hart: &Hart, address: u64) -> u16 { self.bus.read16(self.translate(address, hart)) }
  pub(crate) fn read32(&self, hart: &Hart, address: u64) -> u32 { self.bus.read32(self.translate(address, hart)) }
  pub(crate) fn read64(&self, hart: &Hart, address: u64) -> u64 { self.bus.read64(self.translate(address, hart)) }
  pub(crate) fn write8(&mut self, hart: &Hart, address: u64, data: u8) { self.bus.write8(self.translate(address, hart), data) }
  pub(crate) fn write16(&mut self, hart: &Hart, address: u64, data: u16) { self.bus.write16(self.translate(address, hart), data) }
  pub(crate) fn write32(&mut self, hart: &Hart, address: u64, data: u32) { self.bus.write32(self.translate(address, hart), data) }
  pub(crate) fn write64(&mut self, hart: &Hart, address: u64, data: u64) { self.bus.write64(self.translate(address, hart), data) }
  pub(crate) fn atomic_swap32(&mut self, hart: &Hart, address: u64, val: u32, ordering: Ordering) -> u32 { self.bus.atomic_swap32(self.translate(address, hart), val, ordering) }
  pub(crate) fn atomic_swap64(&mut self, hart: &Hart, address: u64, val: u64, ordering: Ordering) -> u64 { self.bus.atomic_swap64(self.translate(address, hart), val, ordering) }
  pub(crate) fn atomic_add32(&mut self, hart: &Hart, address: u64, val: u32, ordering: Ordering) -> u32 { self.bus.atomic_add32(self.translate(address, hart), val, ordering) }
  pub(crate) fn atomic_add64(&mut self, hart: &Hart, address: u64, val: u64, ordering: Ordering) -> u64 { self.bus.atomic_add64(self.translate(address, hart), val, ordering) }
  pub(crate) fn atomic_xor32(&mut self, hart: &Hart, address: u64, val: u32, ordering: Ordering) -> u32 { self.bus.atomic_xor32(self.translate(address, hart), val, ordering) }
  pub(crate) fn atomic_xor64(&mut self, hart: &Hart, address: u64, val: u64, ordering: Ordering) -> u64 { self.bus.atomic_xor64(self.translate(address, hart), val, ordering) }
  pub(crate) fn atomic_and32(&mut self, hart: &Hart, address: u64, val: u32, ordering: Ordering) -> u32 { self.bus.atomic_and32(self.translate(address, hart), val, ordering) }
  pub(crate) fn atomic_and64(&mut self, hart: &Hart, address: u64, val: u64, ordering: Ordering) -> u64 { self.bus.atomic_and64(self.translate(address, hart), val, ordering) }
  pub(crate) fn atomic_or32(&mut self, hart: &Hart, address: u64, val: u32, ordering: Ordering) -> u32 { self.bus.atomic_or32(self.translate(address, hart), val, ordering) }
  pub(crate) fn atomic_or64(&mut self, hart: &Hart, address: u64, val: u64, ordering: Ordering) -> u64 { self.bus.atomic_or64(self.translate(address, hart), val, ordering) }
  pub(crate) fn atomic_min_i32(&mut self, hart: &Hart, address: u64, val: i32, ordering: Ordering) -> i32 { self.bus.atomic_min_i32(self.translate(address, hart), val, ordering) }
  pub(crate) fn atomic_min_i64(&mut self, hart: &Hart, address: u64, val: i64, ordering: Ordering) -> i64 { self.bus.atomic_min_i64(self.translate(address, hart), val, ordering) }
  pub(crate) fn atomic_max_i32(&mut self, hart: &Hart, address: u64, val: i32, ordering: Ordering) -> i32 { self.bus.atomic_max_i32(self.translate(address, hart), val, ordering) }
  pub(crate) fn atomic_max_i64(&mut self, hart: &Hart, address: u64, val: i64, ordering: Ordering) -> i64 { self.bus.atomic_max_i64(self.translate(address, hart), val, ordering) }
  pub(crate) fn atomic_min_u32(&mut self, hart: &Hart, address: u64, val: u32, ordering: Ordering) -> u32 { self.bus.atomic_min_u32(self.translate(address, hart), val, ordering) }
  pub(crate) fn atomic_min_u64(&mut self, hart: &Hart, address: u64, val: u64, ordering: Ordering) -> u64 { self.bus.atomic_min_u64(self.translate(address, hart), val, ordering) }
  pub(crate) fn atomic_max_u32(&mut self, hart: &Hart, address: u64, val: u32, ordering: Ordering) -> u32 { self.bus.atomic_max_u32(self.translate(address, hart), val, ordering) }
  pub(crate) fn atomic_max_u64(&mut self, hart: &Hart, address: u64, val: u64, ordering: Ordering) -> u64 { self.bus.atomic_max_u64(self.translate(address, hart), val, ordering) }
}
