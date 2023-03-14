//! The device manager.

use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::net::TcpStream;

use protocol::{Message, Protocol};

use crate::numpad::Numpad;

/// Wrapper around the responsibilities of the device.
///
/// For now, just manages the keyboard. Eventually the keyboard should be on a
/// child thread so that the blocking reads won't hold up the other operations.
pub struct Manager {
  prot: Protocol,
  receiver: Receiver<Message>
}

impl Manager {
  /// Create a new manager. Should be a singleton, but not enforced.
  pub fn new(keyboard_name: &str) -> Self {
    let (sender, receiver) = channel();
    let prot = Protocol::new(TcpStream::connect("127.0.0.1:8000").unwrap());

    Numpad::start(keyboard_name, sender);

    Self { prot, receiver }
  }

  /// Let the manager take control of the main thread. 
  ///
  /// All peripheral signals will be sent along over the protocol.
  ///
  /// Some rearchitecting should be done here...
  pub fn run(&mut self) {
    loop {
      let msg = self.prot.read_msg().unwrap();
      use Message::*;
      match msg {
        ReadInput => {
          loop {
            // not how this should be done in the long run...
            let msg = self.receiver.recv().unwrap();
            self.prot.send_msg(msg).expect("Message send failed");
          }
        }
        _ => panic!("Unexpected message"),
      };
    }
  }
}
