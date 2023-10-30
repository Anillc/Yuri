use crate::{hart::Hart, devices::{bus::Bus, Device}, mmu::MMU};

pub(crate) struct Cpu {
  pub(crate) bus: Bus,
  pub(crate) mmu: MMU,
  pub(crate) hart: Hart,
}

impl Cpu {
  pub(crate) fn new(mem: Box<[u8]>) -> Cpu {
    let bus = Bus::new(mem);
    let mmu = MMU::new(bus.clone());
    Cpu {
      mmu,
      bus: bus.clone(),
      hart: Hart::new(),
    }
  }

  pub(crate) fn run(&mut self) {
    loop {
      self.hart.step(&mut self.mmu);
      let fromvm = self.bus.read64(0x0000000080001000);
      if fromvm != 0 {
        self.bus.write64(0x0000000080001000, 0);
        if fromvm >> 32 == 0x01010000 {
          print!("{}", char::from_u32(fromvm as u32).unwrap());
        } else if fromvm == 1 {
          println!("passed");
          break;
        } else {
          println!("{:x}", fromvm);
        }
      }
    }
  }
}
