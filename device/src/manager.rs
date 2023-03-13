//! The overall device manager.

use std::{fs::File, io::BufRead};
use std::io::BufReader;
use std::net::TcpStream;

use protocol::{Message, Protocol};

/// Wrapper around the responsibilities of the device.
///
/// For now, just manages the keyboard. Eventually the keyboard should be on a
/// child thread so that the blocking reads won't hold up the other operations.
pub struct Manager {
  keyboard: BufReader<File>,
  prot: Protocol,
}

impl Manager {
  /// Create a new manager. Should be a singleton, but not enforced.
  pub fn new(keyboard_name: &str) -> Self {
    let keyboard = BufReader::new(File::open(keyboard_name).unwrap());
    let conn = TcpStream::connect("127.0.0.1:8000").unwrap();
    let prot = Protocol::new(conn);
    Self { keyboard, prot }
  }

  /// Let the manager take control of the main thread. From this point forward,
  /// all peripheral signals will be sent along over the protocol.
  pub fn run(&mut self) {
    loop {
      let msg = self.prot.read_msg().unwrap();
      use Message::*;
      match msg {
        ReadInput => {
          let mut line = String::new();
          self.keyboard.read_line(&mut line).expect("Failed to read line from keyboard");
          self.prot.send_msg(Line(line)).expect("Message send failed");
        }
        _ => panic!("Unexpected message"),
      };
    }
  }
}
