use std::{fs::File, io::Read};

use protocol::Message;

pub enum Command {
  Send(Message),
  Receive(Message),
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
    let obj_str = line.split_off(1);
    let obj = serde_json::from_str(obj_str.as_str()).expect("Failed to parse");
    let cmd = match line.chars().next() {
      Some('>') => Command::Send(obj),
      Some('<') => Command::Receive(obj),
      x => panic!("Unexpected start of line: {:?}", x),
    };
    cmds.push(cmd);
  }
  cmds
}
