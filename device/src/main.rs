use std::{net::TcpStream, io::{stdin, BufRead}};
use protocol::{Message, Protocol};

fn handler(mut p: Protocol) -> std::io::Result<()> {
  loop {
    match p.read_msg() {
      Ok(msg) => {
        use Message::*;
        match msg {
          ReadInput => {
            println!("Server requested a line, send something:");
            let line = stdin().lock().lines().next().unwrap().unwrap();
            p.send_msg(Line(line))?;
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
  let prtcl = Protocol::new(conn);

  handler(prtcl)?;

  Ok(())
}
