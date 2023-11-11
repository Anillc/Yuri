use std::array;

use static_init::dynamic;

use crate::{hart::Hart, trap::Exception, mmu::MMU};

use self::extensions::{i::i, zifenci::zifenci, zicsr::zicsr, m::m, a::a, f::f, d::d, sm::sm};

pub(crate) mod extensions;

// 0, 2, 4
pub(crate) type InstructionLen = u64;

#[derive(Debug)]
pub(crate) enum InstructionWithType {
  L32(u32), L16(u16),
}

//                         (mask, comp, instructor), index is opcode
type InstructionMap = [Vec<(u32, u32, Instructor)>; 128];

#[dynamic]
static INSTRUCTORS: InstructionMap = {
  let mut res: InstructionMap = array::from_fn(|_| vec![]);
  let mut instructors: Vec<Instructor> = Vec::new();
  instructors.extend(i());
  instructors.extend(zifenci());
  instructors.extend(zicsr());
  instructors.extend(m());
  instructors.extend(a());
  instructors.extend(f());
  instructors.extend(d());
  instructors.extend(sm());
  for instructor in instructors {
    res.get_mut(instructor.opcode).unwrap()
      .push((instructor.mask(), instructor.comp(), instructor));
  }
  res
};

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
  pub(crate) opcode: usize,
  pub(crate) segments: Vec<InstructionSegment>,
  pub(crate) run: fn(inst: u32, len: InstructionLen, mmu: &mut MMU, hart: &mut Hart) -> Result<(), Exception>
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
  let set = INSTRUCTORS.get((inst & 0b1111111) as usize).unwrap();
  for (mask, comp, instructor) in set {
    if (inst & *mask) == *comp {
      return Some(instructor);
    }
  }
  None
}
