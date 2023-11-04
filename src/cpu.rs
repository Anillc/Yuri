use std::fs;

use elf::{ElfBytes, endian::LittleEndian};

use crate::{hart::Hart, devices::{bus::{Bus, DeviceController}, Device}, mmu::MMU};

pub(crate) struct Cpu {
  pub(crate) bus: Bus,
  pub(crate) mmu: MMU,
  pub(crate) hart: Hart,
}

impl Cpu {
  pub(crate) fn new() -> (Cpu, DeviceController) {
    let (bus, controller) = Bus::new();
    let mmu = MMU::new(bus.clone());
    (Cpu {
      mmu,
      bus: bus.clone(),
      hart: Hart::new(),
    }, controller)
  }

  pub(crate) fn run(&mut self) {
    let mut bus = self.bus.clone();
    loop {
      self.hart.step(&mut self.mmu);
      self.bus.step(&mut bus, &mut self.hart);
    }
  }

  pub(crate) fn load_elf(&mut self, file: &str) {
    let file = fs::read(file).unwrap();
    let elf = ElfBytes::<LittleEndian>::minimal_parse(&file).unwrap();
    for segment in elf.segments().unwrap() {
      for i in 0..segment.p_filesz {
        if segment.p_type != elf::abi::PT_LOAD { continue; }
        self.bus.write8(segment.p_paddr + i, file[(segment.p_offset + i) as usize]).unwrap();
      }
    }
    self.hart.pc = elf.ehdr.e_entry;
  }
}
