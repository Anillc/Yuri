#![feature(atomic_from_ptr)]
use crate::{instructions::parse, cpu::Cpu};

mod cpu;
mod register;
mod memory;
mod instructions;
mod csr;
mod utils;

fn main() {
    let mut mem: [u8; 1024] = [0; 1024];
    let mut cpu = Cpu::new(&mut mem);
    let instructor = parse(0x1bf520b7);
    (instructor.run)(0x1bf520b7, &mut cpu);
}
