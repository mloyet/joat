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

impl LCD {
  /// Public static funciton to fork off worker thread
  pub fn start(filename: &str, receiver: Receiver<LCDCommand>) {
    let file = OpenOptions::new().write(true).open(filename).unwrap();
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
