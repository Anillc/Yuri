#[derive(Debug)]
pub(crate) enum Exception {
  IllegalInstruction,
  Breakpoint(u64),
  LoadAddressMisaligned(u64),
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

impl Trap {
  pub(crate) fn code(&self) -> u64 {
    match self {
      Trap::Interrupt(Interrupt::SupervisorSoftwareInterrupt) => 1,
      Trap::Interrupt(Interrupt::MachineSoftwareInterrupt) => 3,
      Trap::Interrupt(Interrupt::SupervisorTimerInterrupt) => 5,
      Trap::Interrupt(Interrupt::MachineTimerInterrupt) => 7,
      Trap::Interrupt(Interrupt::SupervisorExternalInterrupt) => 9,
      Trap::Interrupt(Interrupt::MachineExternalInterrupt) => 11,
      Trap::Exception(Exception::IllegalInstruction) => 2,
      Trap::Exception(Exception::Breakpoint(_)) => 3,
      Trap::Exception(Exception::LoadAddressMisaligned(_)) => 4,
      Trap::Exception(Exception::EnvironmentCallFromUMode) => 8,
      Trap::Exception(Exception::EnvironmentCallFromSMode) => 9,
      Trap::Exception(Exception::EnvironmentCallFromMMode) => 11,
    }
  }
}
