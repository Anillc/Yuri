use crate::{register::{Registers, FRegisters}, memory::Memory, csrs::{CsrRegistry, MIEP}, instructions::{parse, extensions::c::decompress, Instructor}, trap::{Exception, Trap, Interrupt}};

#[derive(Debug, Clone, Copy)]
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

  pub(crate) fn from_u8(num: u8) -> Option<Mode> {
    match num {
      0b00 => Some(Mode::User),
      0b01 => Some(Mode::Supervisor),
      0b11 => Some(Mode::Machine),
      _ => None,
    }
  }
}

pub struct Hart {
  pub(crate) mem: Memory,
  pub(crate) regs: Registers,
  pub(crate) fregs: FRegisters,
  pub(crate) pc: u64,
  pub(crate) csr: CsrRegistry,
  pub(crate) mode: Mode,
}

impl Hart {
  pub fn new(mem: Memory) -> Hart {
    Hart {
      mem,
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
    fn check_mode(hart: &Hart, interrupt_mode: Mode) -> bool {
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
    let (cause, delegation) = match trap {
      Trap::Exception(_) => (code, self.csr.read_medeleg()),
      Trap::Interrupt(_) => ((1 << 63) | code, self.csr.read_mideleg()),
    };
    let mode = if (delegation >> code) & 0b1 == 0 {
      Mode::Machine
    } else {
      Mode::Supervisor
    };
    let trap_value = match trap {
        Trap::Exception(Exception::Breakpoint(value))
      | Trap::Exception(Exception::LoadAddressMisaligned(value)) => value,
      _ => 0,
    };
    let vec = match mode {
      Mode::Machine => {
        self.csr.trap_into_machine(self.mode);
        self.csr.write_mepc(self.pc);
        self.csr.write_mcause(cause);
        self.csr.write_mtval(trap_value);
        self.csr.read_mtvec()
      },
      Mode::Supervisor => {
        self.csr.trap_into_supervisor(self.mode);
        self.csr.write_sepc(self.pc);
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
