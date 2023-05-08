//! The numpad peripheral implementation.

use std::fmt::Display;
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::sync::mpsc::Sender;
use std::thread;

use protocol::Message;

use crate::lcd::{LCDCommand, CLUB, DIAMOND, HEART, SPADE};

/// Object that encapsulates the numpad peripheral.
///
/// Continously reads from the file and sends messages along the send
/// channel.
pub struct Numpad {
  file: File,
  sender: Sender<Message>,
  to_lcd: Sender<LCDCommand>,
}

impl Numpad {
  /// Public static function to fork off worker thread
  pub fn start(filename: &str, sender: Sender<Message>, to_lcd: Sender<LCDCommand>) {
    let file = OpenOptions::new().read(true).open(filename).unwrap();
    let mut numpad = Self {
      file,
      sender,
      to_lcd,
    };
    thread::Builder::new()
      .name("numpad".to_string())
      .spawn(move || numpad.run())
      .unwrap();
  }

  /// Private worker thread loop
  fn run(&mut self) {
    loop {
      let line: String = self.readline();
      println!("[numpad] Received line: {}", line);

      self.sender.send(Message::Line(line)).unwrap();
    }
  }

  /// Manage reading and filtering the scancodes into a String.
  fn readline(&mut self) -> String {
    let mut ascii = Vec::new();

    // helper for matching logic
    macro_rules! push_and_send {
      ($x:expr) => {{
        ascii.push($x);
        self.to_lcd.send(LCDCommand::Write($x.into())).unwrap();
      }};
    }

    loop {
      let event = InputEvent::blocking_read_from_file(&mut self.file);

      // event type 1 is keypress.
      // ignore all others.
      if event.value != 1 {
        continue;
      }

      // event codes taken from the linux source code.
      // https://github.com/raspberrypi/linux/blob/rpi-5.15.y/include/uapi/linux/input-event-codes.h.
      match event.code {
        71 => push_and_send!('7'),
        72 => push_and_send!('8'),
        73 => push_and_send!('9'),
        75 => push_and_send!('4'),
        76 => push_and_send!('5'),
        77 => push_and_send!('6'),
        79 => push_and_send!('1'),
        80 => push_and_send!('2'),
        81 => push_and_send!('3'),
        82 => push_and_send!('0'),
        55 => push_and_send!(HEART),
        74 => push_and_send!(DIAMOND),
        78 => push_and_send!(SPADE),
        98 => push_and_send!(CLUB),
        // backspace
        14 => {
          ascii.pop();
          self.to_lcd.send(LCDCommand::Write("\x08".into())).unwrap();
        }
        // equal/enter cuts off the string.
        96 | 117 => {
          self.to_lcd.send(LCDCommand::Write("\n".into())).unwrap();
          return String::from_iter(ascii);
        }
        _ => {}
      }
    }
  }
}

/// What comes out of event device files.
///
/// https://github.com/raspberrypi/linux/blob/rpi-5.15.y/drivers/input/input-compat.h.
struct InputEvent {
  sec: u64,
  usec: u64,
  typ: u16,
  code: u16,
  value: i32,
}

impl InputEvent {
  /// Hopefully this works as intended and is blocking...
  fn blocking_read_from_file(file: &mut File) -> Self {
    let mut raw_struct = [0; 24];
    file.read_exact(&mut raw_struct).unwrap();

    let (sec, rest) = raw_struct.split_at(8);
    let sec = u64::from_ne_bytes(sec.try_into().unwrap());

    let (usec, rest) = rest.split_at(8);
    let usec = u64::from_ne_bytes(usec.try_into().unwrap());

    let (typ, rest) = rest.split_at(2);
    let typ = u16::from_ne_bytes(typ.try_into().unwrap());

    let (code, value) = rest.split_at(2);
    let code = u16::from_ne_bytes(code.try_into().unwrap());
    let value = i32::from_ne_bytes(value.try_into().unwrap());

    Self {
      sec,
      usec,
      typ,
      code,
      value,
    }
  }
}

impl Display for InputEvent {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}.{:0>6}) ", self.sec, self.usec)?;
    write!(
      f,
      "typ: {} code: {} value: {}",
      self.typ, self.code, self.value
    )
  }
}
