use softfloat_wrapper::{RoundingMode, Float, F32, ExceptionFlags};

use crate::{hart::Hart, csrs::CsrRegistry, trap::Exception};

pub(crate) mod channel;

pub(crate) fn extend_sign(origin: u64, length: usize) -> i64 {
  let pos = origin & (1 << (length - 1)) == 0;
  if pos {
    origin as i64
  } else {
    let mask = ((1 << (64 - length)) - 1) << length;
    mask | origin as i64
  }
}

pub(crate) fn round_mode(rm: u8, hart: &Hart) -> Result<RoundingMode, Exception> {
  match rm {
    0b000 => Ok(RoundingMode::TiesToEven),
    0b001 => Ok(RoundingMode::TowardZero),
    0b010 => Ok(RoundingMode::TowardNegative),
    0b011 => Ok(RoundingMode::TowardPositive),
    0b100 => Ok(RoundingMode::TiesToAway),
    0b101 | 0b110 => Err(Exception::IllegalInstruction),
    0b111 => {
      let rm = hart.csr.read_frm();
      match rm {
        0b111 => Err(Exception::IllegalInstruction),
        _ => round_mode(rm, hart),
      }
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

pub(crate) struct FloatFlags {
  flags: ExceptionFlags,
}

impl FloatFlags {
  pub(crate) fn new() -> FloatFlags {
    let flags = FloatFlags { flags: ExceptionFlags::default() };
    flags.flags.set();
    flags
  }

  pub(crate) fn get(self) -> ExceptionFlags {
    let FloatFlags { mut flags } = self;
    flags.get();
    flags
  }

  pub(crate) fn write(self, csr: &mut CsrRegistry, dz: bool) {
    let flags = self.get();
    let mut data = 0;
    data |= (flags.is_invalid() as u64) << 4;
    data |= (dz as u64) << 3;
    data |= (flags.is_overflow() as u64) << 2;
    data |= (flags.is_underflow() as u64) << 1;
    data |= flags.is_inexact() as u64;
    csr.write_fflags(data);
  }
}

pub(crate) fn check_and_set_fs(hart: &mut Hart, set_dirty: bool) -> Result<(), Exception> {
  if hart.csr.read_mstatus_fs() == 0 { return Err(Exception::IllegalInstruction); }
  if set_dirty {
    hart.csr.write_mstatus_fs(0b11);
  }
  Ok(())
}
