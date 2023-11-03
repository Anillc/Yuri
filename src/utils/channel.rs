use std::{sync::{Arc, Mutex}, collections::LinkedList};


#[derive(Debug)]
pub(crate) struct Channel<T> {
  buffer: LinkedList<T>,
}

#[derive(Debug, Clone)]
pub(crate) struct Sender<T> {
  channel: Arc<Mutex<Channel<T>>>,
}

#[derive(Debug)]
pub(crate) struct Receiver<T> {
  channel: Arc<Mutex<Channel<T>>>,
}

impl<T> Sender<T> {
  pub(crate) fn send(&self, t: T) {
    let mut channel = self.channel.lock().unwrap();
    channel.buffer.push_back(t);
  }
}

impl<T> Receiver<T> {
  pub(crate) fn avaliable(&self) -> bool {
    let channel = self.channel.lock().unwrap();
    channel.buffer.len() != 0
  }

  pub(crate) fn clear(&self) {
    let mut channel = self.channel.lock().unwrap();
    channel.buffer.clear();
  }

  pub(crate) fn recv(&self) -> Option<T> {
    let mut channel = self.channel.lock().unwrap();
    channel.buffer.pop_front()
  }
}

pub(crate) fn channel<T>() -> (Sender<T>, Receiver<T>) {
  let channel: Channel<T> = Channel {
    buffer: LinkedList::new(),
  };
  let channel = Arc::new(Mutex::new(channel));
  (Sender { channel: channel.clone() }, Receiver { channel })
}
