use std::{sync::{Arc, Mutex, Condvar}, collections::LinkedList};


#[derive(Debug)]
pub(crate) struct Channel<T> {
  buffer: Arc<Mutex<LinkedList<T>>>,
  condvar: Arc<Condvar>,
}

impl<T> Clone for Channel<T> {
  fn clone(&self) -> Self {
    Self {
      buffer: self.buffer.clone(),
      condvar: self.condvar.clone(),
    }
  }
}

#[derive(Debug, Clone)]
pub(crate) struct Sender<T> {
  channel: Channel<T>,
}

#[derive(Debug)]
pub(crate) struct Receiver<T> {
  channel: Channel<T>,
}

impl<T> Sender<T> {
  pub(crate) fn send(&self, t: T) {
    let mut buffer = self.channel.buffer.lock().unwrap();
    buffer.push_back(t);
    self.channel.condvar.notify_one();
  }
}

impl<T> Receiver<T> {
  pub(crate) fn avaliable(&self) -> bool {
    let buffer = self.channel.buffer.lock().unwrap();
    buffer.len() != 0
  }

  pub(crate) fn clear(&self) {
    let mut buffer = self.channel.buffer.lock().unwrap();
    buffer.clear();
  }

  pub(crate) fn recv(&self) -> T {
    let mut buffer = self.channel.buffer.lock().unwrap();
    if buffer.len() == 0 {
      buffer = self.channel.condvar.wait(buffer).unwrap();
    }
    buffer.pop_front().unwrap()
  }
}

pub(crate) fn channel<T>() -> (Sender<T>, Receiver<T>) {
  let channel: Channel<T> = Channel {
    buffer: Arc::new(Mutex::new(LinkedList::new())),
    condvar: Arc::new(Condvar::new()),
  };
  (Sender { channel: channel.clone() }, Receiver { channel })
}
