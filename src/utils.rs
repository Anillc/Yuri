use softfloat_wrapper::{RoundingMode, Float, F32};

use crate::cpu::Cpu;

pub(crate) fn extend_sign(origin: u64, length: usize) -> i64 {
  let pos = origin & (1 << (length - 1)) == 0;
  if pos {
    origin as i64
  } else {
    let mask = ((1 << (64 - length)) - 1) << length;
    mask | origin as i64
  }
}

// TODO: None -> illegal instruction
pub(crate) fn round_mode(rm: u8, _cpu: &Cpu) -> Option<RoundingMode> {
  match rm {
    0b000 => Some(RoundingMode::TiesToEven),
    0b001 => Some(RoundingMode::TowardZero),
    0b010 => Some(RoundingMode::TowardNegative),
    0b011 => Some(RoundingMode::TowardPositive),
    0b100 => Some(RoundingMode::TiesToAway),
    0b101 | 0b110 => None,
    0b111 => {
      // todo!(" dynamic rounding mode")
      Some(RoundingMode::TiesToEven)
    },
    _ => unreachable!(),
  }
}

pub(crate) fn classify<T: Float>(num: T) -> u64 {
  1 << if num.is_negative_infinity() {
    0
  } else if num.is_negative_normal() {
    1
  } else if num.is_negative_subnormal() {
    2
  } else if num.is_negative_zero() {
    3
  } else if num.is_positive_zero() {
    4
  } else if num.is_positive_subnormal() {
    5
  } else if num.is_positive_normal() {
    6
  } else if num.is_positive_infinity() {
    7
  } else if num.is_signaling_nan() {
    8
  } else if num.is_nan() {
    9
  } else {
    unreachable!()
  }
}

pub(crate) trait Boxed {
  fn unbox(&self) -> u32;
}

impl Boxed for u64 {
  fn unbox(&self) -> u32 {
    if ((self >> 32) as u32).wrapping_add(1) == 0 {
      *self as u32
    } else {
      F32::quiet_nan().to_bits()
    }
  }
}
