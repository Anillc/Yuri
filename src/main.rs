#![feature(atomic_from_ptr)]
#![feature(try_blocks)]
use std::fs;

use elf::{ElfBytes, endian::LittleEndian};

use crate::cpu::Cpu;

mod cpu;
mod register;
mod memory;
mod instructions;
mod csr;
mod utils;

fn main() {
  let mut mem: Vec<u8> = vec![0; 1024 * 1024 * 10];
  let file = fs::read("/home/anillc/a.out").unwrap();
  let elf = ElfBytes::<LittleEndian>::minimal_parse(&file).unwrap();
  for section in elf.section_headers().unwrap() {
    for i in 0..section.sh_size {
      mem[(section.sh_addr + i) as usize] = file[(section.sh_offset + i) as usize];
    }
  }
  let mut cpu = Cpu::new(mem.as_mut());
  cpu.pc = elf.ehdr.e_entry;
  cpu.regs.set(2, 114514);
  loop {
    cpu.step();
  };
}
