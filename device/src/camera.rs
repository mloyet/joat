//! The camera/detection peripheral implementation

use std::io;
use std::io::Bytes;
use std::io::Error;
use std::io::ErrorKind;
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
use protocol::Rank;
use protocol::Suit;

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
    let mut cmd = Command::new("python3")
      .args(&[scriptname])
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
      self.script_in.write_all(&[b'\n']).unwrap();
      println!("[camera] Running detection on {}", self.fname);
      let cards = self.read_result().expect("Failed to parse script output");
      println!("[camera] Detected {} cards", cards.len());
      self.sender.send(cards).unwrap();
    }
  }

  // Mini parser.

  fn word(&mut self) -> io::Result<String> {
    let mut v = Vec::new();
    loop {
      match self.script_out.next().unwrap().unwrap() {
        b' ' | b'\n' => break,
        c => v.push(c),
      }
    }
    String::from_utf8(v).map_err(|e| Error::new(ErrorKind::Other, e))
  }

  fn rank(&mut self) -> io::Result<Rank> {
    let r = self.word()?;
    Ok(if r == "A" {
      1.into()
    } else if r == "J" {
      11.into()
    } else if r == "Q" {
      12.into()
    } else if r == "K" {
      13.into()
    } else {
      r.parse::<usize>().unwrap().into()
    })
  }

  fn suit(&mut self) -> io::Result<Suit> {
    let suitname = self.word()?;
    if suitname == "Spades" {
      Ok(Suit::SPADE)
    } else if suitname == "Clubs" {
      Ok(Suit::CLUB)
    } else if suitname == "Hearts" {
      Ok(Suit::HEART)
    } else if suitname == "Diamonds" {
      Ok(Suit::DIAMOND)
    } else {
      panic!("unexpected suitname: {}", suitname);
    }
  }

  fn read_result(&mut self) -> io::Result<DetectResult> {
    let mut result = Vec::new();
    loop {
      // Continue?
      match self.script_out.peek().unwrap().as_ref().unwrap() {
        b'.' => {
          self.script_out.next(); // .
          self.script_out.next(); // newline
          println!("[camera] reached end of detection");
          break;
        },
        x => println!("{}", x),
      }

      // Read in a card.
      let number = self.rank()?;
      let suit = self.suit()?;
      println!("[camera] {} {}", number, suit);
      result.push(Card(suit, number));
    }
    Ok(result)
  }
}
