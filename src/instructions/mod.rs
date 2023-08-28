use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::cpu::Cpu;

use self::extensions::{i::i, zifenci::zifenci, zicsr::zicsr, m::m};

mod extensions;
mod types;

//                               opcode, (mask, comp, instructor)
static INSTRUCTORS: Lazy<HashMap<u8, Vec<(u32, u32, Instructor)>>> = Lazy::new(|| {
  let mut res: HashMap<u8, Vec<(u32, u32, Instructor)>> = HashMap::new();
  let mut instructors: Vec<Instructor> = Vec::new();
  instructors.extend(i());
  instructors.extend(zifenci());
  instructors.extend(zicsr());
  instructors.extend(m());
  for instructor in instructors {
    let mut mask = 0u32;
    let mut comp = 0u32;
    for InstructionSegment { start, end, comp: segment_comp } in &instructor.segments {
      mask |= ((1 << (end - start + 1)) - 1) << start;
      comp |= segment_comp << start;
    }
    res.entry(instructor.opcode).or_default().push((mask, comp, instructor));
  }
  res
});

#[derive(Debug)]
pub(crate) struct InstructionSegment {
  pub(crate) start: usize,
  pub(crate) end: usize,
  pub(crate) comp: u32,
}

#[derive(Debug)]
pub(crate) struct Instructor {
  #[allow(dead_code)]
  pub(crate) name: &'static str,
  pub(crate) opcode: u8,
  pub(crate) segments: Vec<InstructionSegment>,
  pub(crate) run: fn(inst: u32, cpu: &mut Cpu)
}

pub(crate) fn parse(inst: u32) -> &'static Instructor {
  let instructor = INSTRUCTORS.get(&((inst & 0b1111111) as u8)).unwrap()
    .iter().find(|(mask, comp, _)| (inst & *mask) == *comp);
  let (_, _, instructor) = instructor.unwrap();
  instructor
}
