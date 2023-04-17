//! The camera/detection peripheral implementation

use std::io::Bytes;
use std::io::Read;
use std::io::Write;
use std::iter::Peekable;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;

use std::process::ChildStdin;
use std::process::ChildStdout;
use std::process::Command;
use std::process::Stdio;

use protocol::Card;

/// Commands that can be send to the Camera object by the Manager.
/// Currently, the camera does one snapshot per request, but the API
/// could include different capture modes in the future as required.
pub enum CameraCommand {
  RequestScan,
  // BeginStreamingReads,
  // EndStreamingReads
}

pub type DetectResult = Vec<Card>;

pub struct Camera {
  fname: String,
  script_in: ChildStdin,
  script_out: Peekable<Bytes<ChildStdout>>,
  receiver: Receiver<CameraCommand>,
  sender: Sender<DetectResult>,
}

/// Object that encapsulates the Camera peripheral.
///
/// It is assumed that from the boot scripts the camera is streaming
/// updates to fname. The ML detection script runs as a child process.
/// The Camera object holds onto the input and output pipes in order
/// to request and read results of the classification.
///
/// The Manager object communicates with the camera through two channels,
/// one that sends CameraCommand objects, the other that receives
/// DetectResults.
impl Camera {
  pub fn start(
    filename: &str,
    scriptname: &str,
    receiver: Receiver<CameraCommand>,
    sender: Sender<DetectResult>,
  ) {
    let mut cmd = Command::new(scriptname)
      .stdout(Stdio::piped())
      .stdin(Stdio::piped())
      .spawn()
      .expect("Failed to spawn detection subprocess");

    let script_out = cmd.stdout.take().unwrap().bytes().peekable();
    let mut camera = Self {
      fname: filename.to_string(),
      script_in: cmd.stdin.take().unwrap(),
      script_out,
      receiver,
      sender,
    };

    thread::Builder::new()
      .name("camera".to_string())
      .spawn(move || camera.run())
      .unwrap();
  }

  fn run(&mut self) {
    loop {
      let _ = self.receiver.recv().unwrap();
      self.script_in.write_all(self.fname.as_bytes()).unwrap();
      let cards = self.read_result();
      self.sender.send(cards).unwrap();
    }
  }

  // Mini parser.

  fn expect(&mut self, v: u8) {
    match self.script_out.next().unwrap().unwrap() {
      g if g == v => return,
      x => panic!("Expected {}, got {}", v, x),
    }
  }

  fn read_item(&mut self) -> String {
    todo!();
  }

  fn parse_card(&self, _card: String) -> Card {
    todo!();
  }

  fn read_result(&mut self) -> DetectResult {
    let mut result = Vec::new();
    self.expect(b'[');
    loop {
      // Read in a card.
      let item = self.read_item();
      let card = self.parse_card(item);
      result.push(card);

      // Continue? 
      match self.script_out.peek().unwrap().as_ref().unwrap() {
        b',' => {
          self.script_out.next().unwrap().unwrap();
        }
        b']' => break,
        s => panic!("Expected separator or end, got: {}", s),
      }
    }
    self.expect(b']');
    result
  }
}
