use protocol::{Message, Protocol};
use std::{fs::File, net::TcpListener};

fn handler(mut p: Protocol) -> std::io::Result<()> {
  loop {
    use Message::*;
    p.send_msg(ReadInput)?;

    let msg = p.read_msg()?;
    match msg {
      Line(str) => {
        println!("Got a message: {}", str);
        p.send_msg(Print(str)).unwrap();
      }
      _ => panic!("Unexpected message"),
    };
  }
}

fn main() -> std::io::Result<()> {
  let listener = TcpListener::bind("0.0.0.0:8000")?;

  for stream in listener.incoming() {
    let stream = stream?;
    println!("Attached to client {:?}", stream);

    let mut prtcl = protocol::Protocol::new(stream);
    let logfile = File::create("server.log")?;
    prtcl.attach_logfile(logfile);

    if let Err(_) = handler(prtcl) {
      println!("Disconnected.");
    }
  }

  Ok(())
}
