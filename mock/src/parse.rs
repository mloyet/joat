use std::{fs::File, io::Read};

use protocol::Message;

pub enum Command {
  Send(Message),
  Receive(Message),
  Comment(String),
}

pub fn parse(filename: &str) -> Vec<Command> {
  let mut data = String::new();
  File::open(filename)
    .expect("Parse passed bad filename")
    .read_to_string(&mut data)
    .expect("Failed to read file to string");

  let mut cmds = Vec::new();
  for line in data.split("\n") {
    if line.is_empty() {
      continue;
    }
    let mut line = line.to_string();
    let body = line.split_off(1);
    let cmd = match line.chars().next() {
      Some('>') => Command::Send(serde_json::from_str(body.as_str()).unwrap()),
      Some('<') => Command::Receive(serde_json::from_str(body.as_str()).unwrap()),
      Some('#') => Command::Comment(body),
      x => panic!("Unexpected start of line: {:?}", x),
    };
    cmds.push(cmd);
  }
  cmds
}
