use crate::{register::{Registers, FRegisters}, memory::Memory, csrs::{CsrRegistry, MIEP}, instructions::{parse, extensions::c::decompress, Instructor}};

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

#[derive(Debug)]
pub(crate) enum Interrupt {
  SupervisorSoftwareInterrupt,
  MachineSoftwareInterrupt,
  SupervisorTimerInterrupt,
  MachineTimerInterrupt,
  SupervisorExternalInterrupt,
  MachineExternalInterrupt,
}

#[derive(Debug)]
pub(crate) enum Trap {
  Exception(Exception),
  Interrupt(Interrupt),
}

pub struct Cpu<'a> {
  pub(crate) mem: Memory<'a>,
  pub(crate) regs: Registers,
  pub(crate) fregs: FRegisters,
  pub(crate) pc: u64,
  pub(crate) csr: CsrRegistry,
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
      csr: CsrRegistry::new(),
      mode: Mode::Machine,
    }
  }

  pub(crate) fn step(&mut self) {
    let interrupt = self.check_interrupt();
    if let Some(interrupt) = interrupt {
      self.handle_trap(Trap::Interrupt(interrupt));
    }
    match self.instruct() {
      Ok(len) => self.pc = self.pc.wrapping_add(if len == 32 { 4 } else { 2 }),
      Err(exception) => self.handle_trap(Trap::Exception(exception)),
    };
  }

  //                               instruction length
  fn instruct(&mut self) -> Result<u64, Exception> {
    let inst = self.mem.read32(self.pc);
    let parsed: Option<(&Instructor, u32, u64)> = try {
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
    (instructor.run)(inst, len, self)?;
    Ok(len)
  }

  fn check_interrupt(&self) -> Option<Interrupt> {
    let mie = self.csr.read_mie();
    let mip = self.csr.read_mip();
    fn check_mode(cpu: &Cpu, interrupt_mode: Mode) -> bool {
      match (interrupt_mode, cpu.mode) {
        (Mode::Machine, Mode::Machine) => cpu.csr.read_mstatus_mie(),
        (Mode::Supervisor, Mode::Supervisor) => cpu.csr.read_mstatus_sie(),
        (Mode::Machine, Mode::Supervisor) => true,
        (Mode::Supervisor, Mode::Machine) => false,
        _ => unreachable!(),
      }
    }
    match (mie, mip) {
      (MIEP { ms: true, .. }, MIEP { ms: true, .. })
        if check_mode(self, Mode::Machine) => Some(Interrupt::MachineSoftwareInterrupt),
      (MIEP { mt: true, .. }, MIEP { mt: true, .. })
        if check_mode(self, Mode::Machine) => Some(Interrupt::MachineTimerInterrupt),
      (MIEP { me: true, .. }, MIEP { me: true, .. })
        if check_mode(self, Mode::Machine) => Some(Interrupt::MachineExternalInterrupt),
      (MIEP { ss: true, .. }, MIEP { ss: true, .. })
        if check_mode(self, Mode::Supervisor) => Some(Interrupt::SupervisorSoftwareInterrupt),
      (MIEP { mt: true, .. }, MIEP { st: true, .. })
        if check_mode(self, Mode::Supervisor) => Some(Interrupt::SupervisorTimerInterrupt),
      (MIEP { se: true, .. }, MIEP { se: true, .. })
        if check_mode(self, Mode::Supervisor) => Some(Interrupt::SupervisorExternalInterrupt),
      _ => None,
    }
  }

  fn handle_trap(&mut self, trap: Trap) {

  }
}
