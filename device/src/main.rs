use std::{net::TcpStream, io::{stdin, BufRead}};
use protocol::{Message, send_msg, read_msg};

fn handler(s: TcpStream) -> std::io::Result<()> {
  loop {
    match read_msg(&s) {
      Ok(msg) => {
        use Message::*;
        match msg {
          ReadInput => {
            println!("Server requested a line, send something:");
            let line = stdin().lock().lines().next().unwrap().unwrap();
            send_msg(&s, Line(line))?;
          }
          _ => panic!("Unexpected message")
        };
      }
      Err(e) => println!("Failed to read message: {:?}", e),
    }
  }
}

fn main() -> std::io::Result<()> {
  let conn = TcpStream::connect("127.0.0.1:8000")?;

  handler(conn)?;

  Ok(())
}
