#![allow(dead_code)]

use std::collections::HashSet;

use protocol::{Card, Message, Protocol};

pub struct Player {
  prot: Protocol,
  hand: HashSet<Card>,
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
}
