pub(crate) fn extend_sign(origin: u64, length: usize) -> i64 {
  let pos = origin & (1 << (length - 1)) == 0;
  if pos {
    origin as i64
  } else {
    let mask = ((1 << (64 - length)) - 1) << length;
    mask | origin as i64
  }
}
