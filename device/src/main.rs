use protocol::{Message, Protocol};
use std::{
  io::{stdin, BufRead},
  net::TcpStream,
};

fn handler(mut p: Protocol) -> std::io::Result<()> {
  loop {
    let msg = p.read_msg()?;
    use Message::*;
    match msg {
      ReadInput => {
        println!("Server requested a line, send something:");
        let line = stdin().lock().lines().next().unwrap().unwrap();
        p.send_msg(Line(line))?;
      }
      _ => panic!("Unexpected message"),
    };
  }
}

fn main() -> std::io::Result<()> {
  let conn = TcpStream::connect("127.0.0.1:8000")?;
  let prtcl = Protocol::new(conn);

  if let Err(_) = handler(prtcl) {
    println!("Disconnected");
  }

  Ok(())
}
