use crate::{register::{Registers, FRegisters}, memory::Memory, csr::Csr, instructions::{parse, extensions::c::decompress, Instructor, InstructorResult}};

pub struct Cpu<'a> {
  pub(crate) mem: Memory<'a>,
  pub(crate) regs: Registers,
  pub(crate) fregs: FRegisters,
  pub(crate) pc: u64,
  pub(crate) csr: Csr,
}

impl<'a> Cpu<'a> {
  pub fn new(mem: &'a mut [u8]) -> Cpu<'a> {
    Cpu {
      mem: Memory::new(mem),
      regs: Registers::new(),
      fregs: FRegisters::new(),
      pc: 0,
      csr: Csr::new(),
    }
  }

  pub(crate) fn step(&mut self) {
    let inst = self.mem.read32(self.pc);
    let parsed: Option<(&Instructor, u32, u64)> = try {
      let (inst, add) = if inst & 0b11 == 0b11 {
        if inst == 0xd8e33303 {
          dbg!(123);
        }
        println!("{:x}", inst);
        (inst, 4)
      } else {
        println!("{:x}", inst as u16);
        (decompress((inst) as u16)?, 2)
      };
      (parse(inst)?, inst, add)
    };
    // TODO: illegal instruction
    let (instructor, inst, pc_add) = parsed.unwrap();
    let result = (instructor.run)(inst, self);
    match result {
      InstructorResult::Success => self.pc += pc_add,
      InstructorResult::Jump => {},
      InstructorResult::Trap => {},
    };
  }
}
