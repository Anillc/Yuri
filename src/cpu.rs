use crate::{hart::Hart, memory::Memory};

pub(crate) struct Cpu {
  pub(crate) mem: Memory,
  pub(crate) hart: Hart,
}

impl Cpu {
  pub(crate) fn new(mem: Box<[u8]>) -> Cpu {
    let mem = Memory::new(mem);
    Cpu {
      mem: mem.clone(),
      hart: Hart::new(mem),
    }
  }
}