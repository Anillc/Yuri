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

#[cfg(test)]
mod tests {
  use std::fs;
  use elf::{ElfBytes, endian::LittleEndian};
  use crate::devices::Device;
  use super::Cpu;

  #[test]
  fn riscv_tests() {
    let dir = fs::read_dir("tests").unwrap();
    for file in dir {
      let file = file.unwrap();
      let (mut cpu, _) = Cpu::new();
      cpu.run_test(file.path().to_str().unwrap());
    }
  }

  impl Cpu {
    pub(crate) fn run_test(&mut self, filepath: &str) {
      let file = fs::read(filepath).unwrap();
      let elf = ElfBytes::<LittleEndian>::minimal_parse(&file).unwrap();
      for segment in elf.segments().unwrap() {
        for i in 0..segment.p_filesz {
          if segment.p_type != elf::abi::PT_LOAD { continue; }
          self.bus.write8(segment.p_paddr + i, file[(segment.p_offset + i) as usize]).unwrap();
        }
      }
      self.hart.pc = elf.ehdr.e_entry;
      let (parsing_table, string_table) = elf.symbol_table().unwrap().unwrap();
      let tohost = parsing_table.iter().find(|symbol|
        string_table.get(symbol.st_name as usize).unwrap() == "tohost")
        .unwrap().st_value;

      let mut bus = self.bus.clone();
      loop {
        self.hart.step(&mut self.mmu);
        self.bus.step(&mut bus, &mut self.hart);
        let fromvm = self.bus.read64(tohost).unwrap();
        if fromvm != 0 {
          self.bus.write64(tohost, 0).unwrap();
          if fromvm >> 32 == 0x01010000 {
            print!("{}", char::from_u32(fromvm as u32).unwrap());
          } else if fromvm == 1 {
            println!("'{}' passed", filepath);
            break;
          } else {
            println!("terminated by {:x}", fromvm);
            break;
          }
        }
      }
    }
  }
}
