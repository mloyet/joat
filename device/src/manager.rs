//! The device manager.

use std::net::TcpStream;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use protocol::{Message, Protocol};

use crate::lcd::LCD;
use crate::lcd::LCDCommand;
use crate::numpad::Numpad;

/// Wrapper around the responsibilities of the device.
///
/// Includes the protocol, the numpad device, and the lcd device.
pub struct Manager {
  prot: Protocol,
  numpad_receiver: Receiver<Message>,
  lcd_sender: Sender<LCDCommand>,
}

impl Manager {
  /// Create a new manager. Should be a singleton, but not enforced.
  pub fn new(numpad_name: &str, lcd_name: &str) -> Self {
    let (numpad_sender, numpad_receiver) = channel();
    let (lcd_sender, lcd_receiver) = channel();

    println!("[manager] Creating protocol");
    let prot = Protocol::new(TcpStream::connect("127.0.0.1:8000").unwrap());
    println!("[manager] done.");

    println!("[manager] Creating Numpad");
    Numpad::start(numpad_name, numpad_sender);
    println!("[manager] done.");

    println!("[manager] Creating LCD");
    LCD::start(lcd_name, lcd_receiver);
    println!("[manager] done.");

    Self {
      prot,
      numpad_receiver,
      lcd_sender
    }
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
        Print(s) => self.lcd_sender.send(LCDCommand::Write(s)).unwrap(),
        Clear => self.lcd_sender.send(LCDCommand::Clear).unwrap(),
        ReadInput => {
          let msg = self.numpad_receiver.recv().unwrap();
          self.prot.send_msg(msg).expect("Message send failed");
        }
        _ => panic!("Unexpected message"),
      };
    }
  }
}
