#![feature(atomic_from_ptr)]
#![feature(try_blocks)]
use std::{path::PathBuf, thread::spawn, io::{stdin, Read}};

use clap::{Parser, ValueEnum};
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

#[derive(Debug, Parser)]
struct Args {
  #[arg(long, default_value = "false")]
  htif: bool,
  file: PathBuf,
}

fn main() {
  let Args { htif, file } = Args::parse();
  let (mut cpu, controller) = Cpu::new();
  let uart_sender = controller.uart_sender.clone();
  spawn(move || loop {
    print!("{}", char::from_u32(controller.uart_receiver.recv() as u32).unwrap());
  });
  spawn(move || {
    for input in std::io::stdin().bytes() {
      let input = input.unwrap();
      uart_sender.send(input);
      if htif { todo!() }
    }
    if htif { todo!() }
  });
  if !htif {
    cpu.load_elf(file);
    cpu.run();
  } else {
    todo!();
  }
}
