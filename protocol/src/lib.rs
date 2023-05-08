use serde::{Deserialize, Serialize};
use std::{fmt::Display, fs::File, io::Write, net::TcpStream};

#[derive(Serialize, Deserialize, PartialEq, Hash, Clone, Eq)]
pub enum Suit {
  HEART,
  DIAMOND,
  SPADE,
  CLUB,
}

#[derive(Serialize, Deserialize, PartialEq, Hash, Clone, Eq)]
pub enum Rank {
  ACE,
  TWO,
  THREE,
  FOUR,
  FIVE,
  SIX,
  SEVEN,
  EIGHT,
  NINE,
  TEN,
  JACK,
  QUEEN,
  KING,
}

#[derive(Serialize, Deserialize, PartialEq, Hash, Clone, Eq)]
pub struct Card(pub Suit, pub Rank);

#[derive(Serialize, Deserialize, PartialEq)]
pub enum Message {
  // Server -> Device
  Print(String),
  Clear,
  ReadInput,
  PrintCard(Card),
  RequestScan,

  // Device -> Server
  Line(String),
  DetectedCards(Vec<Card>),

  // Misc
  DEBUG(String),
  ERR(String),
  ACK,
}

// These functions seem kind of silly, but since this is a shared library by both the server and
// the device, if we are ever to change the protocol to send data in a more sophisticated way, we
// will be able to make the change in one place.

pub struct Protocol {
  conn: TcpStream,
  logfile: Option<File>,
}

impl Protocol {
  pub fn new(conn: TcpStream) -> Self {
    Self {
      conn,
      logfile: None,
    }
  }

  //
  // Logging files
  //

  pub fn attach_logfile(&mut self, f: File) {
    self.logfile = Some(f);
  }

  fn log_send(&mut self, msg: &Message) {
    if let Some(f) = &mut self.logfile {
      f.write(b">").expect("Couldn't write to logile");
      serde_json::to_writer(&*f, &msg).expect("Couldn't dump message to logfile");
      f.write(b"\n").expect("Couldn't write to logile");
    }
  }

  fn log_receive(&mut self, msg: &Message) {
    if let Some(f) = &mut self.logfile {
      f.write(b"<").expect("Couldn't write to logile");
      serde_json::to_writer(&*f, &msg).expect("Couldn't dump message to logfile");
      f.write(b"\n").expect("Couldn't write to logile");
    }
  }

  //
  // Message sending/receiving logic
  //

  pub fn send_msg(&mut self, msg: Message) -> Result<(), serde_json::Error> {
    self.log_send(&msg);
    serde_json::to_writer(&self.conn, &msg)
  }

  pub fn read_msg(&mut self) -> Result<Message, serde_json::Error> {
    let mut de = serde_json::Deserializer::from_reader(&self.conn);
    let res = Message::deserialize(&mut de);
    if let Ok(msg) = &res {
      self.log_receive(msg);
    }
    res
  }
}

impl From<usize> for Rank {
  fn from(value: usize) -> Self {
    use Rank::*;
    match value {
      1 => ACE,
      2 => TWO,
      3 => THREE,
      4 => FOUR,
      5 => FIVE,
      6 => SIX,
      7 => SEVEN,
      8 => EIGHT,
      9 => NINE,
      10 => TEN,
      11 => JACK,
      12 => QUEEN,
      13 => KING,
      _ => panic!(),
    }
  }
}

impl From<usize> for Suit {
  fn from(value: usize) -> Self {
    use Suit::*;
    match value {
      0 => HEART,
      1 => DIAMOND,
      2 => CLUB,
      3 => SPADE,
      _ => panic!(),
    }
  }
}

impl Display for Suit {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Suit::HEART => write!(f, "Hearts"),
      Suit::DIAMOND => write!(f, "Diamonds"),
      Suit::SPADE => write!(f, "Spades"),
      Suit::CLUB => write!(f, "Clubs"),
    }
  }
}

impl Display for Rank {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Rank::ACE => write!(f, "Ace"),
      Rank::TWO => write!(f, "Two"),
      Rank::THREE => write!(f, "Three"),
      Rank::FOUR => write!(f, "Four"),
      Rank::FIVE => write!(f, "Five"),
      Rank::SIX => write!(f, "Six"),
      Rank::SEVEN => write!(f, "Seven"),
      Rank::EIGHT => write!(f, "Eight"),
      Rank::NINE => write!(f, "Nine"),
      Rank::TEN => write!(f, "Ten"),
      Rank::JACK => write!(f, "Jack"),
      Rank::QUEEN => write!(f, "Queen"),
      Rank::KING => write!(f, "King"),
    }
  }
}

impl Display for Card {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let Card(s, r) = self;
    write!(f, "{} {}", s, r)
  }
}
