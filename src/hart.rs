use crate::{register::{Registers, FRegisters}, csrs::{CsrRegistry, MIEP}, instructions::{parse, extensions::c::decompress, Instructor, InstructionLen, InstructionWithType}, trap::{Exception, Trap, Interrupt}, mmu::MMU};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Mode {
  User, Supervisor, Machine,
}

impl Mode {
  pub(crate) fn as_u8(&self) -> u16 {
    match self {
      Mode::User => 0b00,
      Mode::Supervisor => 0b01,
      Mode::Machine => 0b11,
    }
  }

  pub(crate) fn from_u8(num: u8) -> Mode {
    match num {
      0b00 => Mode::User,
      0b01 => Mode::Supervisor,
      0b11 => Mode::Machine,
      _ => unreachable!(),
    }
  }
}

pub struct Hart {
  pub(crate) regs: Registers,
  pub(crate) fregs: FRegisters,
  pub(crate) pc: u64,
  pub(crate) csr: CsrRegistry,
  pub(crate) mode: Mode,
  pub(crate) wfi: bool,
}

impl Hart {
  pub fn new() -> Hart {
    Hart {
      regs: Registers::new(),
      fregs: FRegisters::new(),
      pc: 0,
      csr: CsrRegistry::new(),
      mode: Mode::Machine,
      wfi: false,
    }
  }

  pub(crate) fn step(&mut self, mmu: &mut MMU) {
    let interrupt = self.check_interrupt();
    if let Some(interrupt) = interrupt {
      self.handle_trap(Trap::Interrupt(interrupt));
    }
    match self.instruct(mmu) {
      Ok(len) => self.pc = self.pc.wrapping_add(len),
      Err(exception) => self.handle_trap(Trap::Exception(exception)),
    };
  }

  fn instruct(&mut self, mmu: &mut MMU) -> Result<InstructionLen, Exception> {
    if self.wfi {
      return Ok(0);
    }
    let inst = mmu.fetch(self, self.pc)?;
    let parsed: Option<(&Instructor, u32, InstructionLen)> = try {
      let (inst, len) = match inst {
        InstructionWithType::L32(inst) => {
          // println!("{:x} {:?}: {:x}", self.pc, self.mode, inst);
          (inst, 4)
        },
        InstructionWithType::L16(inst) => {
          // println!("{:x} {:?}: {:x}", self.pc, self.mode, inst);
          (decompress(inst)?, 2)
        },
      };
      (parse(inst)?, inst, len)
    };
    let (instructor, inst, len) = parsed
      .ok_or(Exception::IllegalInstruction)?;
    (instructor.run)(inst, len, mmu, self)?;
    Ok(len)
  }

  fn check_interrupt(&mut self) -> Option<Interrupt> {
    let mie = self.csr.read_mie();
    let mip = self.csr.read_mip();
    fn check_mode(hart: &mut Hart, interrupt_mode: Mode) -> bool {
      // resume hart here, as mie and mip have benn checked
      // WFI ignores MIE and SIE in mstatus
      hart.wfi = false;
      match (interrupt_mode, hart.mode) {
        (Mode::Machine, Mode::Machine) => hart.csr.read_mstatus_mie(),
        (Mode::Supervisor, Mode::Supervisor) => hart.csr.read_mstatus_sie(),
        (Mode::Machine, Mode::Supervisor) => true,
        (Mode::Supervisor, Mode::Machine) => false,
        _ => unreachable!(),
      }
    }
    match (mie, mip) {
      (MIEP { ms: true, .. }, MIEP { ms: true, .. })
        if check_mode(self, Mode::Machine) => Some(Interrupt::MachineSoftware),
      (MIEP { mt: true, .. }, MIEP { mt: true, .. })
        if check_mode(self, Mode::Machine) => Some(Interrupt::MachineTimer),
      (MIEP { me: true, .. }, MIEP { me: true, .. })
        if check_mode(self, Mode::Machine) => Some(Interrupt::MachineExternal),
      (MIEP { ss: true, .. }, MIEP { ss: true, .. })
        if check_mode(self, Mode::Supervisor) => Some(Interrupt::SupervisorSoftware),
      (MIEP { mt: true, .. }, MIEP { st: true, .. })
        if check_mode(self, Mode::Supervisor) => Some(Interrupt::SupervisorTimer),
      (MIEP { se: true, .. }, MIEP { se: true, .. })
        if check_mode(self, Mode::Supervisor) => Some(Interrupt::SupervisorExternal),
      _ => None,
    }
  }

  fn handle_trap(&mut self, trap: Trap) {
    let code = trap.code();
    let (delegation, cause) = match trap {
      Trap::Interrupt(_) => (self.csr.read_mideleg(), (1 << 63) | code),
      Trap::Exception(_) => (self.csr.read_medeleg(), code),
    };
    let mode = match self.mode {
      // Traps never transition from a more-privileged mode to a less-privileged mode.
      Mode::Machine => Mode::Machine,
      Mode::Supervisor | Mode::User => if (delegation >> code) & 0b1 == 1 {
        Mode::Supervisor
      } else {
        Mode::Machine
      },
    };
    let trap_value = match trap {
        Trap::Exception(Exception::InstructionAccessFault(value))
      | Trap::Exception(Exception::Breakpoint(value))
      | Trap::Exception(Exception::LoadAddressMisaligned(value))
      | Trap::Exception(Exception::LoadAccessFault(value))
      | Trap::Exception(Exception::StoreAMOAddressMisaligned(value))
      | Trap::Exception(Exception::StoreAMOAccessFault(value))
      | Trap::Exception(Exception::InstructionPageFault(value))
      | Trap::Exception(Exception::LoadPageFault(value))
      | Trap::Exception(Exception::StoreAMOPageFault(value)) => value,
      _ => 0,
    };
    let vec = match mode {
      Mode::Machine => {
        self.csr.trap_into_machine(self.mode);
        self.csr.write_mepc(self.pc & !1);
        self.csr.write_mcause(cause);
        self.csr.write_mtval(trap_value);
        self.csr.read_mtvec()
      },
      Mode::Supervisor => {
        self.csr.trap_into_supervisor(self.mode);
        self.csr.write_sepc(self.pc & !1);
        self.csr.write_scause(cause);
        self.csr.write_stval(trap_value);
        self.csr.read_stvec()
      },
      _ => unreachable!(),
    };
    self.mode = mode;
    match vec & 0b11 {
      0 => self.pc = vec,
      1 => self.pc = (vec & !0b11) + 4 * code,
      _ => unreachable!(),
    }
  }
}
