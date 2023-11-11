use std::{fs, path::PathBuf};

use elf::{ElfBytes, endian::LittleEndian};

use crate::{hart::Hart, devices::{bus::{Bus, DeviceController}, Device}, mmu::MMU, utils::channel::{Receiver, Sender}};

pub(crate) struct Cpu {
  pub(crate) bus: Bus,
  pub(crate) mmu: MMU,
  pub(crate) hart: Hart,
  bus_step: u32,
}

impl Cpu {
  pub(crate) fn new() -> (Cpu, DeviceController) {
    let (bus, controller) = Bus::new();
    let mmu = MMU::new(bus.clone());
    (Cpu {
      mmu,
      bus: bus.clone(),
      hart: Hart::new(),
      bus_step: 0,
    }, controller)
  }

  pub(crate) fn run_elf(&mut self, file: PathBuf) {
    let file = fs::read(file).unwrap();
    let elf = ElfBytes::<LittleEndian>::minimal_parse(&file).unwrap();
    for segment in elf.segments().unwrap() {
      for i in 0..segment.p_filesz {
        if segment.p_type != elf::abi::PT_LOAD { continue; }
        self.bus.write8(segment.p_paddr + i, file[(segment.p_offset + i) as usize]).unwrap();
      }
    }
    self.hart.pc = elf.ehdr.e_entry;

    let mut bus = self.bus.clone();
    loop {
      self.hart.step(&mut self.mmu);
      if self.bus_step > 1000 || self.hart.wfi {
        self.bus_step = 0;
        self.bus.step(&mut bus, &mut self.hart);
      } else {
        self.bus_step += 1;
      }
    }
  }

  pub(crate) fn run_htif(&mut self, file: PathBuf, stdin: Receiver<i32>, stdout: Sender<i32>) {
    let file = fs::read(file).unwrap();
    let elf = ElfBytes::<LittleEndian>::minimal_parse(&file).unwrap();
    for segment in elf.segments().unwrap() {
      for i in 0..segment.p_filesz {
        if segment.p_type != elf::abi::PT_LOAD { continue; }
        self.bus.write8(segment.p_paddr + i, file[(segment.p_offset + i) as usize]).unwrap();
      }
    }
    self.hart.pc = elf.ehdr.e_entry;
    let (parsing_table, string_table) = elf.symbol_table().unwrap().unwrap();
    let fromhost = parsing_table.iter().find(|symbol|
      string_table.get(symbol.st_name as usize).unwrap() == "fromhost")
      .unwrap().st_value;
    let tohost = parsing_table.iter().find(|symbol|
      string_table.get(symbol.st_name as usize).unwrap() == "tohost")
      .unwrap().st_value;

    let mut bus = self.bus.clone();
    loop {
      self.hart.step(&mut self.mmu);
      self.bus.step(&mut bus, &mut self.hart);
      let tovm = self.bus.read64(fromhost).unwrap();
      if tovm != 0 { continue; }
      let fromvm = self.bus.read64(tohost).unwrap();
      if fromvm != 0 {
        self.bus.write64(tohost, 0).unwrap();
        let dev = fromvm >> 56;
        let cmd = fromvm << 8 >> 56;
        let data = fromvm << 16 >> 16;
        match dev {
          0 if cmd == 0 && data == 1 => break,
          1 if cmd == 0 => self.bus.write64(fromhost, (1 << 56) | (stdin.recv() as u64)).unwrap(),
          1 if cmd == 1 => stdout.send(data as i32),
          _ => panic!("terminated with code {:x}", fromvm),
        };
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use std::fs;
  use crate::utils::channel::channel;
  use super::Cpu;

  #[test]
  fn riscv_tests() {
    let dir = fs::read_dir("tests").unwrap();
    for file in dir {
      let (_, htif_receiver) = channel::<i32>();
      let (htif_sender, _) = channel::<i32>();
      let file = file.unwrap().path();
      let (mut cpu, _) = Cpu::new();
      cpu.run_htif(file.clone(), htif_receiver, htif_sender);
      println!("'{}' passed", file.to_str().unwrap());
    }
  }
}
