#![feature(atomic_from_ptr)]
#![feature(try_blocks)]
use std::fs;

use cpu::Cpu;
use elf::{ElfBytes, endian::LittleEndian};

mod cpu;
mod hart;
mod register;
mod mmu;
mod instructions;
mod csrs;
mod utils;
mod trap;
mod devices;

fn run_program(path: &str) {
  let mut mem: Vec<u8> = vec![0; 1024 * 1024 * 1024 * 4];
  let file = fs::read(path).unwrap();
  let elf = ElfBytes::<LittleEndian>::minimal_parse(&file).unwrap();
  for segment in elf.segments().unwrap() {
    for i in 0..segment.p_filesz {
      if segment.p_type != elf::abi::PT_LOAD { continue; }
      mem[(segment.p_vaddr + i) as usize] = file[(segment.p_offset + i) as usize];
    }
  }
  let mut cpu = Cpu::new(mem.into_boxed_slice());
  cpu.hart.pc = elf.ehdr.e_entry;
  cpu.hart.regs.set(2, 0x6f00);
  cpu.hart.run();
  // cpu.hart.step();
  // TODO: this is for test, remove it
  // match x {
  //   Ok(_) => {},
  //   Err(hart::Exception::EnvironmentCallFromMMode) => break,
  //   other => other.unwrap(),
  // }
}

fn main() {
  let dir = fs::read_dir("../riscv-tests/isa").unwrap();
  for file in dir {
    let file = file.unwrap();
    let name = file.file_name();
    let name = name.to_str().unwrap();
    for extension in ["i", "m", "a", "f", "d", "c"] {
      if name.starts_with(&format!("rv64u{extension}-u-")) && !name.contains('.') {
        run_program(file.path().to_str().unwrap());
      }
    }
  }
}
