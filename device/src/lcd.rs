//! LCD peripheral implementation

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::mpsc::Receiver;
use std::thread;

/// A command that can be sent to the LCD screen
pub enum LCDCommand {
  Write(String),
  Clear,
}

/// Object that encapsulates the LCD peripheral.
///
/// Send messages through a channel to the commands receiver for them
/// to be executed asynchronously.
pub struct LCD {
  file: File,
  receiver: Receiver<LCDCommand>,
}

pub const HEART: char = '\x00';
pub const DIAMOND: char = '\x01';
pub const SPADE: char = '\x02';
pub const CLUB: char = '\x03';

const SUITS_INIT: &str = "\x0cLG00000091f0e040000;\
                          \x0cLG10000040e1f0e0400;\
                          \x0cLG200040e0e1f040e00;\
                          \x0cLG3000e0e1f1f040e00;";

impl LCD {
  /// Public static funciton to fork off worker thread
  pub fn start(filename: &str, receiver: Receiver<LCDCommand>) {
    let mut file = OpenOptions::new().write(true).open(filename).unwrap();
    file
      .write(SUITS_INIT.as_bytes())
      .expect("Failed to init suits");
    let mut lcd = Self { file, receiver };
    thread::Builder::new()
      .name("lcd".to_string())
      .spawn(move || lcd.run())
      .unwrap();
  }

  /// Private worker thread loop
  fn run(&mut self) {
    loop {
      let cmd = self.receiver.recv().unwrap();
      use LCDCommand::*;
      match cmd {
        Write(s) => self.write(&s),
        Clear => self.clear(),
      }
    }
  }

  fn write(&mut self, s: &str) {
    self.file.write_all(s.as_bytes()).unwrap();
  }

  fn clear(&mut self) {
    self.file.write_all(&[12]).unwrap(); // form feed
  }
}
