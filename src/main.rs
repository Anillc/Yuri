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
  let mut mem: Vec<u8> = vec![0; 1024 * 1024 * 1024 * 4];
  let file = fs::read("../riscv-tests/isa/rv64ui-u-addw").unwrap();
  let elf = ElfBytes::<LittleEndian>::minimal_parse(&file).unwrap();
  for segment in elf.segments().unwrap() {
    for i in 0..segment.p_filesz {
      if segment.p_type != elf::abi::PT_LOAD { continue; }
      mem[(segment.p_vaddr + i) as usize] = file[(segment.p_offset + i) as usize];
    }
  }
  let mut cpu = Cpu::new(mem.as_mut());
  cpu.pc = elf.ehdr.e_entry;
  cpu.regs.set(2, 0x6f00);
  loop {
    cpu.step();
  };
}
