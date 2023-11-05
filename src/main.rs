#![feature(atomic_from_ptr)]
#![feature(try_blocks)]
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
  let (mut cpu, controller) = Cpu::new();
  std::thread::spawn(move || {
    loop {
      let r = controller.uart_receiver.recv();
      print!("{}", char::from(r));
    }
  });
  cpu.load_elf("fw_payload.elf");
  cpu.run();
}
