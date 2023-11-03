use crate::{device_atomic, device_rw, trap::Exception, hart::Hart, utils::channel::{Receiver, Sender, channel}};

use super::{Device, bus::Bus};

// NS16550A

pub(crate) const UART_START: u64 = 0x10000000;
pub(crate) const UART_END: u64 = UART_START + 8 - 1;
pub(crate) const INTERRUPT_ID: u32 = 1;

const UART_RBR_DLL: u64 = UART_START;
const UART_THR: u64 = UART_START;

const UART_IER_ILM: u64 = UART_START + 1;
const UART_IER_RDI: u8 = 0b00000001;
const UART_IER_THRI: u8 = 0b00000010;

const UART_IIR: u64 = UART_START + 2;
const UART_FCR: u64 = UART_START + 2;
const UART_IIR_NO_INT: u8 = 0b00000001;
const UART_IIR_THRI: u8 = 0b00000010;
const UART_IIR_RDI: u8 = 0b00000100;
const UART_FCR_ENABLE_FIFO: u8 = 0b00000001;
const UART_FCR_CLEAR_RCVR: u8 = 0b00000010;
const UART_FCR_CLEAR_XMIT: u8 = 0b00000100;

const UART_LCR: u64 = UART_START + 3;
const UART_LCR_DLAB: u8 = 0b10000000;

const UART_MCR: u64 = UART_START + 4;
const UART_MCR_LOOP: u8 = 0b00010000;

const UART_LSR: u64 = UART_START + 5;
const UART_LSR_DR: u8 = 0b00000001;
const UART_LSR_OE: u8 = 0b00000010;
const UART_LSR_BI: u8 = 0b00010000;
const UART_LSR_THRE: u8 = 0b00100000;
const UART_LSR_TEMT: u8 = 0b01000000;

const _UART_MSR: u64 = UART_START + 6;
const UART_MCR_OUT2: u8 = 0b00001000;

const UART_SCR: u64 = UART_START + 7;

#[derive(Debug)]
pub(crate) struct Uart {
  receiver: Receiver<u8>,
  loop_sender: Sender<u8>,
  sender: Sender<u8>,
  lcr: u8,
  dll: u8,
  dlm: u8,
  ier: u8,
  iir: u8,
  mcr: u8,
  lsr: u8,
  scr: u8,
  fcr: u8,
}

impl Uart {
  pub(crate) fn new() -> (Uart, Sender<u8>, Receiver<u8>) {
    let (recv_send, recv) = channel();
    let (send, send_recv) = channel();
    (Uart {
      receiver: recv,
      loop_sender: recv_send.clone(),
      sender: send,
      lcr: 0,
      dll: 0x0c,
      dlm: 0,
      ier: 0,
      iir: UART_IIR_NO_INT,
      mcr: UART_MCR_OUT2,
      lsr: UART_LSR_TEMT | UART_LSR_TEMT,
      scr: 0,
      fcr: 0,
    }, recv_send, send_recv)
  }
}

impl Device for Uart {
  device_atomic!();
  device_rw!();

  fn step(&mut self, bus: &mut Bus, _hart: &mut Hart) {
    // TODO: backoff counter?
    if self.receiver.avaliable() {
      self.lsr |= UART_LSR_DR;
    }

    if self.lcr & UART_FCR_CLEAR_RCVR != 0 {
      self.receiver.clear();
      self.lsr &= !UART_LSR_DR & !UART_FCR_CLEAR_RCVR;
    }

    if self.lcr & UART_FCR_CLEAR_XMIT != 0 {
      self.lcr &= !UART_FCR_CLEAR_XMIT;
      self.lsr |= UART_LSR_TEMT | UART_LSR_THRE;
    }

    let mut interrupts: u8 = 0;
    if (self.ier & UART_IER_RDI) != 0 && (self.lsr & UART_LSR_DR) != 0 {
      interrupts |= UART_IIR_RDI;
    }

    if (self.ier & UART_IER_THRI) != 0 && (self.lsr & UART_LSR_TEMT) != 0 {
      interrupts |= UART_IIR_THRI;
    }

    if interrupts != 0 {
      self.iir = UART_IIR_NO_INT;
      bus.plic.lock().unwrap().irq(INTERRUPT_ID, false);
    } else {
      self.iir = interrupts;
      bus.plic.lock().unwrap().irq(INTERRUPT_ID, true);
    }

    if self.ier & UART_IER_THRI == 0 {
      self.lsr |= UART_LSR_TEMT | UART_LSR_THRE;
    }
  }

  fn read8(&mut self, address: u64) -> Result<u8, Exception> {
    match address {
      UART_RBR_DLL => {
        let res = if self.lcr & UART_LCR_DLAB != 0 {
          self.dll
        } else if self.lsr & UART_LSR_BI != 0 {
          0
        } else if let Some(data) = self.receiver.recv() {
          self.lsr &= !UART_LSR_OE;
          data
        } else {
          0
        };
        Ok(res)
      },
      UART_IER_ILM => if self.lcr & UART_LCR_DLAB != 0 {
        Ok(self.dlm)
      } else {
        Ok(self.ier)
      },
      UART_IIR => Ok(self.iir),
      UART_LCR => Ok(self.lcr),
      UART_MCR => Ok(self.mcr),
      UART_LSR => Ok(self.lsr),
      UART_SCR => Ok(self.scr),
      _ => Err(Exception::LoadAccessFault(address)),
    }
  }

  fn write8(&mut self, address: u64, data: u8) -> Result<(), Exception> {
    match address {
      UART_THR => {
        if self.lcr & UART_LCR_DLAB != 0 {
          self.dll = data;
        } else if self.fcr & UART_FCR_ENABLE_FIFO == 0 && self.receiver.avaliable() {
          self.lsr |= UART_LSR_OE;
        } else {
          self.lsr |= UART_LSR_TEMT | UART_LSR_THRE;
          if self.mcr & UART_MCR_LOOP != 0 {
            self.loop_sender.send(data);
          } else {
            self.sender.send(data);
          }
        }
      },
      UART_IER_ILM => if self.lcr & UART_LCR_DLAB != 0 {
        self.dlm = data;
      } else {
        self.ier = data & 0b1111;
      },
      UART_FCR => self.fcr = data,
      UART_LCR => self.lcr = data,
      UART_MCR => self.mcr = data & 0b11111,
      UART_SCR => self.scr = data,
      _ => return Err(Exception::StoreAMOAccessFault(address)),
    };
    Ok(())
  }
}
