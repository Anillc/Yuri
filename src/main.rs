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

fn run_program(path: &str) {
  let mut cpu = Cpu::new();
  cpu.load_elf_test(path);
  cpu.run();
}

fn main() {
  let dir = fs::read_dir("../riscv-tests/run").unwrap();
  for file in dir {
    let file = file.unwrap();
    let name = file.file_name();
    let name = name.to_str().unwrap();
    if name == "rv64mi-p-csr" {
      run_program(file.path().to_str().unwrap());
    }
  }
}
