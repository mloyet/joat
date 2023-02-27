use serde::{Deserialize, Serialize};
use std::net::TcpStream;

#[derive(Serialize, Deserialize)]
pub enum Suit {
  HEART,
  DIAMOND,
  SPADE,
  CLUB,
}

#[derive(Serialize, Deserialize)]
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
  KING
}

#[derive(Serialize, Deserialize)]
pub struct Card(Suit, Rank);

#[derive(Serialize, Deserialize)]
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
  ACK
}

// These functions seem kind of silly, but since this is a shared library by both the server and
// the device, if we are ever to change the protocol to send data in a more sophisticated way, we
// will be able to make the change in one place.

pub fn send_msg(conn: &TcpStream, msg: Message) -> Result<(), serde_json::Error> {
  serde_json::to_writer(conn, &msg)
}

pub fn read_msg(conn: &TcpStream) -> Result<Message, serde_json::Error> {
  let mut de = serde_json::Deserializer::from_reader(conn);
  Message::deserialize(&mut de)
}
