use crate::hart::Mode;

#[derive(Debug)]
pub(crate) enum Exception {
  IllegalInstruction,
  Breakpoint(u64),
  LoadAddressMisaligned(u64),
  StoreAMOAddressMisaligned(u64),
  EnvironmentCallFromUMode,
  EnvironmentCallFromSMode,
  EnvironmentCallFromMMode,
  InstructionPageFault(u64),
  LoadPageFault(u64),
  StoreAMOPageFault(u64),

}

#[derive(Debug)]
pub(crate) enum Interrupt {
  SupervisorSoftware,
  MachineSoftware,
  SupervisorTimer,
  MachineTimer,
  SupervisorExternal,
  MachineExternal,
}

impl Interrupt {
  pub(crate) fn mode(&self) -> Mode {
    match self {
      Interrupt::SupervisorSoftware => Mode::Supervisor,
      Interrupt::MachineSoftware => Mode::Machine,
      Interrupt::SupervisorTimer => Mode::Supervisor,
      Interrupt::MachineTimer => Mode::Machine,
      Interrupt::SupervisorExternal => Mode::Supervisor,
      Interrupt::MachineExternal => Mode::Machine,
    }
  }
}

#[derive(Debug)]
pub(crate) enum Trap {
  Exception(Exception),
  Interrupt(Interrupt),
}

impl Trap {
  pub(crate) fn code(&self) -> u64 {
    match self {
      Trap::Interrupt(Interrupt::SupervisorSoftware) => 1,
      Trap::Interrupt(Interrupt::MachineSoftware) => 3,
      Trap::Interrupt(Interrupt::SupervisorTimer) => 5,
      Trap::Interrupt(Interrupt::MachineTimer) => 7,
      Trap::Interrupt(Interrupt::SupervisorExternal) => 9,
      Trap::Interrupt(Interrupt::MachineExternal) => 11,
      Trap::Exception(Exception::IllegalInstruction) => 2,
      Trap::Exception(Exception::Breakpoint(_)) => 3,
      Trap::Exception(Exception::LoadAddressMisaligned(_)) => 4,
      Trap::Exception(Exception::StoreAMOAddressMisaligned(_)) => 6,
      Trap::Exception(Exception::EnvironmentCallFromUMode) => 8,
      Trap::Exception(Exception::EnvironmentCallFromSMode) => 9,
      Trap::Exception(Exception::EnvironmentCallFromMMode) => 11,
      Trap::Exception(Exception::InstructionPageFault(_)) => 12,
      Trap::Exception(Exception::LoadPageFault(_)) => 13,
      Trap::Exception(Exception::StoreAMOPageFault(_)) => 15,
    }
  }
}
