use std::net::{TcpListener, TcpStream};

use protocol::{read_msg, send_msg, Message};

fn handler(s: TcpStream) -> std::io::Result<()> {
  loop {
    match read_msg(&s) {
      Ok(msg) => {
        use Message::*;
        match msg {
          HELLO => {
            println!("Device said hello!");
            send_msg(&s, ACK)?;
          }
          MSG(m) => {
            println!("Got a message: {}", m);
            send_msg(&s, ACK)?;
          }
          GOODBYE => {
            println!("Got goodbye, detatching");
            return Ok(());
          }
          ACK => panic!("Unexpected ACK"),
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
