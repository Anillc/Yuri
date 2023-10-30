use crate::{hart::Hart, devices::bus::Bus, mmu::MMU};

pub(crate) struct Cpu {
  pub(crate) _bus: Bus,
  pub(crate) hart: Hart,
}

impl Cpu {
  pub(crate) fn new(mem: Box<[u8]>) -> Cpu {
    let bus = Bus::new(mem);
    let mmu = MMU::new(bus.clone());
    Cpu {
      _bus: bus.clone(),
      hart: Hart::new(mmu),
    }
  }
}