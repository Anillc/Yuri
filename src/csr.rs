

pub(crate) struct Csr {
  csr: [u64; 4096]
}

impl Csr {

  pub(crate) fn new() -> Csr {
    Csr { csr: [0; 4096] }
  }

  pub(crate) fn read(&self, address: u16) -> u64 {
    self.csr[address as usize]
  }

  pub(crate) fn write(&mut self, address: u16, data: u64) {
    self.csr[address as usize] = data;
  }
}
