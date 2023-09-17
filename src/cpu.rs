use std::{cell::RefCell, rc::Rc};

use crate::{register::{Registers, FRegisters}, memory::Memory, csrs::CsrRegistry, instructions::{parse, extensions::c::decompress, Instructor}};

#[derive(Debug, Clone, Copy)]
pub(crate) enum Mode {
  User, Supervisor, Machine,
}

impl Mode {
  pub(crate) fn into_u16(&self) -> u16 {
    match self {
      Mode::User => 0b00,
      Mode::Supervisor => 0b01,
      Mode::Machine => 0b11,
    }
  }

  pub(crate) fn from_u8(num: u8) -> Option<Mode> {
    match num {
      0b00 => Some(Mode::User),
      0b01 => Some(Mode::Supervisor),
      0b11 => Some(Mode::Machine),
      _ => None,
    }
  }
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
  pub(crate) csr: Rc<RefCell<CsrRegistry>>,
  // TODO: optimize to number?
  pub(crate) mode: Mode,
}

impl<'a> Cpu<'a> {
  pub fn new(mem: &'a mut [u8]) -> Cpu<'a> {
    Cpu {
      mem: Memory::new(mem),
      regs: Registers::new(),
      fregs: FRegisters::new(),
      pc: 0,
      csr: Rc::new(RefCell::new(CsrRegistry::new())),
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
