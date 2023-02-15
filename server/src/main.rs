use std::net::{TcpListener, TcpStream};

use protocol::{read_msg, send_msg, Message};

fn handler(s: TcpStream) -> std::io::Result<()> {
  loop {
    use Message::*;
    send_msg(&s, ReadInput)?;

    match read_msg(&s) {
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
    if let Err(_) = handler(stream) {
      println!("Disconnected.");
    }
  }

  Ok(())
}
