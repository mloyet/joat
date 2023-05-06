//! The device manager.

use std::net::TcpStream;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use protocol::Card;
use protocol::{Message, Protocol};

use crate::camera::Camera;
use crate::camera::CameraCommand;
use crate::camera::DetectResult;
use crate::lcd::LCDCommand;
use crate::lcd::LCD;
use crate::numpad::Numpad;
use crate::printer::Printer;

/// Wrapper around the responsibilities of the device.
///
/// Includes the protocol, the numpad device, and the lcd device.
pub struct Manager {
  prot: Protocol,
  numpad_receiver: Receiver<Message>,
  lcd_sender: Sender<LCDCommand>,
  printer_sender: Sender<Card>,
  camera_sender: Sender<CameraCommand>,
  detect_receiver: Receiver<DetectResult>,
}

impl Manager {
  /// Create a new manager. Should be a singleton, but not enforced.
  pub fn new(
    numpad_name: &str,
    lcd_name: &str,
    printer_name: &str,
    camera_name: &str,
    script_name: &str,
  ) -> Self {
    let (numpad_sender, numpad_receiver) = channel();
    let (lcd_sender, lcd_receiver) = channel();
    let (printer_sender, printer_receiver) = channel();
    let (camera_sender, camera_receiver) = channel();
    let (detect_sender, detect_receiver) = channel();

    println!("[manager] Creating protocol");
    let prot = Protocol::new(TcpStream::connect("127.0.0.1:8000").unwrap());
    println!("[manager] done.");

    println!("[manager] Creating Numpad");
    Numpad::start(numpad_name, numpad_sender, lcd_sender.clone());
    println!("[manager] done.");

    println!("[manager] Creating LCD");
    LCD::start(lcd_name, lcd_receiver);
    println!("[manager] done.");

    println!("[manager] Creating Printer");
    Printer::start(printer_name, printer_receiver);
    println!("[manager] done.");

    println!("[manager] Creating Camera");
    Camera::start(camera_name, script_name, camera_receiver, detect_sender);
    println!("[manager] done.");

    Self {
      prot,
      numpad_receiver,
      lcd_sender,
      printer_sender,
      camera_sender,
      detect_receiver,
    }
  }

  /// Let the manager take control of the main thread.
  ///
  /// All peripheral signals will be sent along over the protocol.
  ///
  /// Some rearchitecting should be done here...
  ///
  /// Protocol should be made asymmetric...
  pub fn run(&mut self) {
    loop {
      let msg = self.prot.read_msg().unwrap();
      use Message::*;
      match msg {
        Print(s) => self.lcd_sender.send(LCDCommand::Write(s)).unwrap(),
        Clear => self.lcd_sender.send(LCDCommand::Clear).unwrap(),
        ReadInput => {
          self
            .lcd_sender
            .send(LCDCommand::Write("Give Input: ".to_string()))
            .unwrap();
          let msg = self.numpad_receiver.recv().unwrap();
          self.prot.send_msg(msg).expect("Message send failed");
        }
        PrintCard(card) => self.printer_sender.send(card).unwrap(),
        RequestScan => {
          self.camera_sender.send(CameraCommand::RequestScan).unwrap();
          let cards = self.detect_receiver.recv().unwrap();
          self.prot.send_msg(DetectedCards(cards)).unwrap();
        }
        _ => panic!("Server shouldn't send these messages... hm..."),
      };
    }
  }
}
