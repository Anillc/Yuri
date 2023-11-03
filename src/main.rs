#![feature(atomic_from_ptr)]
#![feature(try_blocks)]
use std::fs;

use cpu::Cpu;

mod cpu;
mod hart;
mod register;
mod mmu;
mod instructions;
mod csrs;
mod utils;
mod trap;
mod devices;

fn main() {
  let (mut cpu, sender, receiver) = Cpu::new();
  // cpu.load_elf_test(path);
  // cpu.run();
}
