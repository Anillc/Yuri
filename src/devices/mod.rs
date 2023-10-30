use std::sync::atomic::Ordering;

pub(crate) mod memory;
pub(crate) mod bus;

#[macro_export]
macro_rules! device_atomic {
  () => {
    use std::sync::atomic::Ordering;
    fn atomic_swap32(&mut self, address:u64, val: u32, _: Ordering) -> u32 {
      let origin = self.read32(address);
      self.write32(address, val);
      origin
    }
    fn atomic_swap64(&mut self, address:u64, val: u64, _: Ordering) -> u64 {
      let origin = self.read64(address);
      self.write64(address, val);
      origin
    }
    fn atomic_add32(&mut self, address:u64, val: u32, _: Ordering) -> u32 {
      let origin = self.read32(address);
      self.write32(address, origin.wrapping_add(val));
      origin
    }
    fn atomic_add64(&mut self, address:u64, val: u64, _: Ordering) -> u64 {
      let origin = self.read64(address);
      self.write64(address, origin.wrapping_add(val));
      origin
    }
    fn atomic_xor32(&mut self, address:u64, val: u32, _: Ordering) -> u32 {
      let origin = self.read32(address);
      self.write32(address, origin ^ val);
      origin
    }
    fn atomic_xor64(&mut self, address:u64, val: u64, _: Ordering) -> u64 {
      let origin = self.read64(address);
      self.write64(address, origin ^ val);
      origin
    }
    fn atomic_and32(&mut self, address:u64, val: u32, _: Ordering) -> u32 {
      let origin = self.read32(address);
      self.write32(address, origin & val);
      origin
    }
    fn atomic_and64(&mut self, address:u64, val: u64, _: Ordering) -> u64 {
      let origin = self.read64(address);
      self.write64(address, origin & val);
      origin
    }
    fn atomic_or32(&mut self, address:u64, val: u32, _: Ordering) -> u32 {
      let origin = self.read32(address);
      self.write32(address, origin | val);
      origin
    }
    fn atomic_or64(&mut self, address:u64, val: u64, _: Ordering) -> u64 {
      let origin = self.read64(address);
      self.write64(address, origin | val);
      origin
    }
    fn atomic_min_i32(&mut self, address:u64, val: i32, _: Ordering) -> i32 {
      let origin = self.read32(address) as i32;
      self.write32(address, if origin < val { origin } else { val } as u32);
      origin
    }
    fn atomic_min_i64(&mut self, address:u64, val: i64, _: Ordering) -> i64 {
      let origin = self.read64(address) as i64;
      self.write64(address, if origin < val { origin } else { val } as u64);
      origin
    }
    fn atomic_max_i32(&mut self, address:u64, val: i32, _: Ordering) -> i32 {
      let origin = self.read32(address) as i32;
      self.write32(address, if origin > val { origin } else { val } as u32);
      origin
    }
    fn atomic_max_i64(&mut self, address:u64, val: i64, _: Ordering) -> i64 {
      let origin = self.read64(address) as i64;
      self.write64(address, if origin > val { origin } else { val } as u64);
      origin
    }
    fn atomic_min_u32(&mut self, address:u64, val: u32, _: Ordering) -> u32 {
      let origin = self.read32(address);
      self.write32(address, if origin < val { origin } else { val });
      origin
    }
    fn atomic_min_u64(&mut self, address:u64, val: u64, _: Ordering) -> u64 {
      let origin = self.read64(address);
      self.write64(address, if origin < val { origin } else { val });
      origin
    }
    fn atomic_max_u32(&mut self, address:u64, val: u32, _: Ordering) -> u32 {
      let origin = self.read32(address);
      self.write32(address, if origin > val { origin } else { val });
      origin
    }
    fn atomic_max_u64(&mut self, address:u64, val: u64, _: Ordering) -> u64 {
      let origin = self.read64(address);
      self.write64(address, if origin > val { origin } else { val });
      origin
    }
  };
}

#[macro_export]
macro_rules! device_rw {
  () => {
    fn read16(&self, address: u64) -> u16 {
      u16::from_le_bytes([
        self.read8(address),
        self.read8(address.wrapping_add(1)),
      ])
    }
    fn read32(&self, address: u64) -> u32 {
      u32::from_le_bytes([
        self.read8(address),
        self.read8(address.wrapping_add(1)),
        self.read8(address.wrapping_add(2)),
        self.read8(address.wrapping_add(3)),
      ])
    }
    fn read64(&self, address: u64) -> u64 {
      u64::from_le_bytes([
        self.read8(address),
        self.read8(address.wrapping_add(1)),
        self.read8(address.wrapping_add(2)),
        self.read8(address.wrapping_add(3)),
        self.read8(address.wrapping_add(4)),
        self.read8(address.wrapping_add(5)),
        self.read8(address.wrapping_add(6)),
        self.read8(address.wrapping_add(7)),
      ])
    }
    fn write16(&mut self, address: u64, data: u16) {
      let data = data.to_le_bytes();
      self.write8(address, data[0]);
      self.write8(address.wrapping_add(1), data[1]);
    }
    fn write32(&mut self, address: u64, data: u32) {
      let data = data.to_le_bytes();
      self.write8(address, data[0]);
      self.write8(address.wrapping_add(1), data[1]);
      self.write8(address.wrapping_add(2), data[2]);
      self.write8(address.wrapping_add(3), data[3]);
    }
    fn write64(&mut self, address: u64, data: u64) {
      let data = data.to_le_bytes();
      self.write8(address, data[0]);
      self.write8(address.wrapping_add(1), data[1]);
      self.write8(address.wrapping_add(2), data[2]);
      self.write8(address.wrapping_add(3), data[3]);
      self.write8(address.wrapping_add(4), data[4]);
      self.write8(address.wrapping_add(5), data[5]);
      self.write8(address.wrapping_add(6), data[6]);
      self.write8(address.wrapping_add(7), data[7]);
    }
  };
}

pub(crate) trait Device {
  fn read8(&self, address: u64) -> u8;
  fn read16(&self, address: u64) -> u16;
  fn read32(&self, address: u64) -> u32;
  fn read64(&self, address: u64) -> u64;
  fn write8(&mut self, address: u64, data: u8);
  fn write16(&mut self, address: u64, data: u16);
  fn write32(&mut self, address: u64, data: u32);
  fn write64(&mut self, address: u64, data: u64);
  fn atomic_swap32(&mut self, address: u64, val: u32, ordering: Ordering) -> u32;
  fn atomic_swap64(&mut self, address: u64, val: u64, ordering: Ordering) -> u64;
  fn atomic_add32(&mut self, address: u64, val: u32, ordering: Ordering) -> u32;
  fn atomic_add64(&mut self, address: u64, val: u64, ordering: Ordering) -> u64;
  fn atomic_xor32(&mut self, address: u64, val: u32, ordering: Ordering) -> u32;
  fn atomic_xor64(&mut self, address: u64, val: u64, ordering: Ordering) -> u64;
  fn atomic_and32(&mut self, address: u64, val: u32, ordering: Ordering) -> u32;
  fn atomic_and64(&mut self, address: u64, val: u64, ordering: Ordering) -> u64;
  fn atomic_or32(&mut self, address: u64, val: u32, ordering: Ordering) -> u32;
  fn atomic_or64(&mut self, address: u64, val: u64, ordering: Ordering) -> u64;
  fn atomic_min_i32(&mut self, address: u64, val: i32, ordering: Ordering) -> i32;
  fn atomic_min_i64(&mut self, address: u64, val: i64, ordering: Ordering) -> i64;
  fn atomic_max_i32(&mut self, address: u64, val: i32, ordering: Ordering) -> i32;
  fn atomic_max_i64(&mut self, address: u64, val: i64, ordering: Ordering) -> i64;
  fn atomic_min_u32(&mut self, address: u64, val: u32, ordering: Ordering) -> u32;
  fn atomic_min_u64(&mut self, address: u64, val: u64, ordering: Ordering) -> u64;
  fn atomic_max_u32(&mut self, address: u64, val: u32, ordering: Ordering) -> u32;
  fn atomic_max_u64(&mut self, address: u64, val: u64, ordering: Ordering) -> u64;
}