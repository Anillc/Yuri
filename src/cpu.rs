use crate::{register::{Registers, FRegisters}, memory::Memory, csr::Csr, instructions::{parse, extensions::c::decompress, Instructor}};

#[derive(Debug)]
pub(crate) enum Mode {
  User, Supervisor, Machine,
}

#[derive(Debug)]
pub(crate) enum Exception {
  IllegalInstruction,
  Breakpoint,
  LoadAddressMisaligned,
  EnvironmentCallFromUMode,
  EnvironmentCallFromSMode,
  EnvironmentCallFromMMode,
}

pub struct Cpu<'a> {
  pub(crate) mem: Memory<'a>,
  pub(crate) regs: Registers,
  pub(crate) fregs: FRegisters,
  pub(crate) pc: u64,
  pub(crate) csr: Csr,
  pub(crate) mode: Mode,
}

impl<'a> Cpu<'a> {
  pub fn new(mem: &'a mut [u8]) -> Cpu<'a> {
    Cpu {
      mem: Memory::new(mem),
      regs: Registers::new(),
      fregs: FRegisters::new(),
      pc: 0,
      csr: Csr::new(),
      mode: Mode::Machine,
    }
  }

  pub(crate) fn step(&mut self) -> Result<(), Exception> {
    let inst = self.mem.read32(self.pc);
    let parsed: Option<(&Instructor, u32, usize)> = try {
      let (inst, add) = if inst & 0b11 == 0b11 {
        println!("{:x}", inst);
        (inst, 32)
      } else {
        println!("{:x}", inst as u16);
        (decompress((inst) as u16)?, 16)
      };
      (parse(inst)?, inst, add)
    };
    let (instructor, inst, len) = match parsed {
        Some(parsed) => parsed,
        None => Err(Exception::IllegalInstruction)?,
    };
    let result = (instructor.run)(inst, len, self);
    self.pc = self.pc.wrapping_add(if len == 32 { 4 } else { 2 });
    result
  }
}
