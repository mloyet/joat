#![allow(dead_code)]

use std::{collections::HashSet, time::Duration, thread};

use protocol::{Card, Message, Protocol};

pub struct Player {
  prot: Protocol,
  pub hand: HashSet<Card>,
}

impl Player {
  pub fn new(prot: Protocol) -> Self {
    Self {
      prot,
      hand: HashSet::new(),
    }
  }

  pub fn send_msg(&mut self, msg: &str) {
    self.prot.send_msg(Message::Print(msg.to_string() + "\n")).unwrap();
  }

  pub fn clear(&mut self) {
    self.prot.send_msg(Message::Clear).unwrap();
  }

  pub fn send_card(&mut self, card: Card) {
    self.hand.insert(card.clone());
    self.prot.send_msg(Message::PrintCard(card)).unwrap();
    thread::sleep(Duration::new(10, 0));
  }

  pub fn read_input(&mut self) -> String {
    self.prot.send_msg(Message::ReadInput).unwrap();
    match self.prot.read_msg().unwrap() {
      Message::Line(s) => s,
      _ => panic!(),
    }
  }

  pub fn read_table(&mut self) -> Vec<Card> {
    self.prot.send_msg(Message::RequestScan).unwrap();
    match self.prot.read_msg().unwrap() {
      Message::DetectedCards(cs) => cs,
      _ => panic!(),
    }
  }
}
