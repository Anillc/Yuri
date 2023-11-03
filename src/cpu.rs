use std::fs;

use elf::{ElfBytes, endian::LittleEndian};

use crate::{hart::Hart, devices::{bus::Bus, Device}, mmu::MMU, utils::channel::{Sender, Receiver}};

pub(crate) struct Cpu {
  pub(crate) bus: Bus,
  pub(crate) mmu: MMU,
  pub(crate) hart: Hart,
  // for riscv-tests
  pub(crate) tohost: u64,
}

impl Cpu {
  pub(crate) fn new() -> (Cpu, Sender<u8>, Receiver<u8>) {
    let (bus, sender, receiver) = Bus::new();
    let mmu = MMU::new(bus.clone());
    (Cpu {
      mmu,
      bus: bus.clone(),
      hart: Hart::new(),
      tohost: 0,
    }, sender, receiver)
  }

  pub(crate) fn run(&mut self) {
    let mut bus = self.bus.clone();
    loop {
      self.hart.step(&mut self.mmu);
      self.bus.step(&mut bus, &mut self.hart);
      // riscv-tests
      let fromvm = self.bus.read64(self.tohost).unwrap();
      if fromvm != 0 {
        self.bus.write64(self.tohost, 0).unwrap();
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

  pub(crate) fn load_elf_test(&mut self, file: &str) {
    let file = fs::read(file).unwrap();
    let elf = ElfBytes::<LittleEndian>::minimal_parse(&file).unwrap();
    for segment in elf.segments().unwrap() {
      for i in 0..segment.p_filesz {
        if segment.p_type != elf::abi::PT_LOAD { continue; }
        self.bus.write8(segment.p_paddr + i, file[(segment.p_offset + i) as usize]).unwrap();
      }
    }
    self.hart.pc = elf.ehdr.e_entry;
    self.hart.regs.set(2, 0x80000000 + 0x6f00);
    let (parsing_table, string_table) = elf.symbol_table().unwrap().unwrap();
    let tohost = parsing_table.iter().find(|symbol|
      string_table.get(symbol.st_name as usize).unwrap() == "tohost").unwrap();
    self.tohost = tohost.st_value;
  }
}
