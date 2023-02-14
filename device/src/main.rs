use std::net::TcpStream;
use protocol::{Message, send_msg};

fn main() -> std::io::Result<()> {
  let conn = TcpStream::connect("127.0.0.1:8000")?;

  use Message::*;
  send_msg(&conn, HELLO)?;
  println!("Said hello");
  // read_msg(&conn)?; // ACK
  // println!("got ACK");
  send_msg(&conn, MSG("TEST".to_string()))?;
  println!("sent TEST");
  // read_msg(&conn)?; // ACK
  // println!("got ACK");
  send_msg(&conn, GOODBYE)?;
  println!("sent goodbye");

  Ok(())
}
