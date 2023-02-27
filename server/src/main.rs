use std::net::TcpListener;
use protocol::{Protocol, Message};

fn handler(mut p: Protocol) -> std::io::Result<()> {
  loop {
    use Message::*;
    p.send_msg(ReadInput)?;

    match p.read_msg() {
      Ok(msg) => {
        match msg {
          Line(str) => {
            println!("Got a message: {}", str);
          }
          _ => panic!("Unexpected message")
        };
      }
      Err(e) => println!("Failed to read message: {:?}", e),
    }
  }
}

fn main() -> std::io::Result<()> {
  let listener = TcpListener::bind("127.0.0.1:8000")?;

  for stream in listener.incoming() {
    let stream = stream?;
    println!("Attached to client {:?}", stream);
    let prtcl = protocol::Protocol::new(stream);
    if let Err(_) = handler(prtcl) {
      println!("Disconnected.");
    }
  }

  Ok(())
}
