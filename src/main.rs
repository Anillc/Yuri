#![feature(atomic_from_ptr)]
#![feature(try_blocks)]
use std::io::Read;

use cpu::Cpu;
use utils::channel::channel;

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
  let (mut cpu, _controller) = Cpu::new();
  let (sender, receiver) = channel::<i32>();
  std::thread::spawn(move || {
    for i in std::io::stdin().bytes() {
      sender.send(i.unwrap() as i32);
    }
    sender.send(-1);
  });
  cpu.run_htif("tests/amtest-yuri.elf", receiver);
}
