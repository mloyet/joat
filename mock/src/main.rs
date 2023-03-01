use parse::Command;
use protocol::Protocol;
use std::{env, net::TcpListener};

use crate::parse::parse;

mod parse;

fn expect(mut prtcl: Protocol, commands: Vec<Command>) {
  for command in commands {
    match command {
      Command::Send(msg) => prtcl.send_msg(msg).expect("Failed to send"),
      Command::Receive(msg) => {
        let recv = prtcl.read_msg().expect("Failed to receive");
        if msg != recv {
          panic!("Message received did not match");
        }
      },
    }
  }
}

fn main() -> std::io::Result<()> {
  let args = env::args();
  if args.len() != 2 {
    panic!("Pass test file as argument");
  }
  let file = args.last().unwrap();
  let commands = parse(&file);

  let listener = TcpListener::bind("127.0.0.1:8000")?;
  let conn = listener.incoming().next().unwrap();
  let prtcl = Protocol::new(conn?);

  expect(prtcl, commands);

  println!("Test passed!");

  Ok(())
}
