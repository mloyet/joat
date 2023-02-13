use std::{net::{TcpListener, TcpStream}, io::{Read, Write}};


fn echoer(mut s: TcpStream) -> std::io::Result<()> {
  let mut buf: [u8;1024] = [0;1024];
  loop {
    let amt = s.read(&mut buf)?;
    s.write(&buf[0..amt])?;
  }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")?;

    for stream in listener.incoming() {
      let stream = stream?;
      println!("Attached to client {:?}", stream);
      if let Err(_) = echoer(stream) {
        println!("Disconnected.");
      }
    }

    Ok(())
}
