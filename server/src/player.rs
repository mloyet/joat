#![allow(dead_code)]

use std::collections::HashSet;

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
    self.prot.send_msg(Message::Print(msg.to_string())).unwrap();
  }

  pub fn clear(&mut self) {
    self.prot.send_msg(Message::Clear).unwrap();
  }

  pub fn send_card(&mut self, card: Card) {
    self.hand.insert(card.clone());
    self.prot.send_msg(Message::PrintCard(card)).unwrap();
  }

  pub fn read_input(&mut self) -> String {
    match self.prot.read_msg().unwrap() {
      Message::Line(s) => s,
      _ => panic!(),
    }
  }

  pub fn read_table(&mut self) -> Vec<Card> {
    match self.prot.read_msg().unwrap() {
      Message::DetectedCards(cs) => cs,
      _ => panic!(),
    }
  }
}
