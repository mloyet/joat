//! The numpad peripheral implementation.

use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::sync::mpsc::Sender;
use std::thread;

use protocol::Message;

/// Object that encapsulates the numpad peripheral.
///
/// Continously reads from the file and sends messages along the send
/// channel.
pub struct Numpad {
  file: File,
  sender: Sender<Message>,
}

impl Numpad {
  /// Public static function to fork off worker thread
  pub fn start(filename: &str, sender: Sender<Message>) {
    let file = File::open(filename).unwrap();
    let mut numpad = Self { file, sender };
    thread::Builder::new().name("numpad".to_string()).spawn(move || numpad.run()).unwrap();
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
    loop {
      let event = InputEvent::blocking_read_from_file(&mut self.file);
      println!("{}", event);

      // event type 1 is keypress.
      // ignore all others.
      if event.value != 1 {
        continue;
      }

      // codes taken from the linux source code.
      // https://github.com/raspberrypi/linux/blob/rpi-5.15.y/include/uapi/linux/input-event-codes.h.
      match event.code {
        55 => ascii.push('*'),
        71 => ascii.push('7'),
        72 => ascii.push('8'),
        73 => ascii.push('9'),
        74 => ascii.push('-'),
        75 => ascii.push('4'),
        76 => ascii.push('5'),
        77 => ascii.push('6'),
        78 => ascii.push('+'),
        79 => ascii.push('1'),
        80 => ascii.push('2'),
        81 => ascii.push('3'),
        82 => ascii.push('0'),
        98 => ascii.push('/'),
        // equal/enter cuts off the string.
        96 | 117 => return String::from_iter(ascii),
        _ => {}
      }
    }
  }
}

/// What comes out of event device files.
///
/// https://github.com/raspberrypi/linux/blob/rpi-5.15.y/drivers/input/input-compat.h.
struct InputEvent {
  sec: u32,
  usec: u32,
  typ: u16,
  code: u16,
  value: i32,
}

impl InputEvent {
  /// Hopefully this works as intended and is blocking...
  fn blocking_read_from_file(file: &mut File) -> Self {

    let mut raw_struct = [0; 16];
    file.read_exact(&mut raw_struct).unwrap();

    let (sec, rest) = raw_struct.split_at(4);
    let sec = u32::from_ne_bytes(sec.try_into().unwrap());

    let (usec, rest) = rest.split_at(4);
    let usec = u32::from_ne_bytes(usec.try_into().unwrap());

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
      write!(f, "typ: {} code: {} value: {}", self.typ, self.code, self.value)
    }
}
