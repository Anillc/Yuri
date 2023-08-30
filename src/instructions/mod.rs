use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::cpu::Cpu;

use self::extensions::{i::i, zifenci::zifenci, zicsr::zicsr, m::m, a::a, f::f, d::d};

pub(crate) mod extensions;
pub(crate) mod types;

//                               opcode, (mask, comp, instructor)
static INSTRUCTORS: Lazy<HashMap<u8, Vec<(u32, u32, Instructor)>>> = Lazy::new(|| {
  let mut res: HashMap<u8, Vec<(u32, u32, Instructor)>> = HashMap::new();
  let mut instructors: Vec<Instructor> = Vec::new();
  instructors.extend(i());
  instructors.extend(zifenci());
  instructors.extend(zicsr());
  instructors.extend(m());
  instructors.extend(a());
  instructors.extend(f());
  instructors.extend(d());
  for instructor in instructors {
    res.entry(instructor.opcode).or_default()
      .push((instructor.mask(), instructor.comp(), instructor));
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

impl Instructor {
  pub(crate) fn mask(&self) -> u32 {
    let mut mask = 0u32;
    for InstructionSegment { start, end, comp: _ } in &self.segments {
      mask |= ((1 << (end - start + 1)) - 1) << start;
    }
    mask
  }

  pub(crate) fn comp(&self) -> u32 {
    let mut comp = 0u32;
    for InstructionSegment { start, end: _, comp: segment_comp } in &self.segments {
      comp |= segment_comp << start;
    }
    comp
  }
}

pub(crate) fn parse(inst: u32) -> Option<&'static Instructor> {
  let (_, _, instructor) = INSTRUCTORS.get(&((inst & 0b1111111) as u8))?
    .iter().find(|(mask, comp, _)| (inst & *mask) == *comp)?;
  Some(instructor)
}
