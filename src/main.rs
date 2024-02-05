#![feature(try_blocks)]
use std::{path::PathBuf, thread::spawn, io::Read};

use clap::Parser;
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
  let (htif_stdin_sender, htif_stdin_receiver) = channel::<i32>();
  let (htif_stdout_sender, htif_stdout_receiver) = channel::<i32>();
  spawn(move || loop {
    print!("{}", char::from_u32(controller.uart_receiver.recv() as u32).unwrap());
  });
  spawn(move || loop {
    print!("{}", char::from_u32(htif_stdout_receiver.recv() as u32).unwrap());
  });
  spawn(move || {
    for input in std::io::stdin().bytes() {
      let input = input.unwrap();
      uart_sender.send(input);
      if htif { htif_stdin_sender.send(input as i32); }
    }
    if htif { htif_stdin_sender.send(-1); }
  });
  if !htif {
    cpu.run_elf(file);
  } else {
    cpu.run_htif(file, htif_stdin_receiver, htif_stdout_sender);
  }
}
