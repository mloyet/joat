#![allow(dead_code)]

use crate::{deck::Deck, player::Player};

pub struct GoFishGame {
  deck: Deck,
  players: Vec<Player>,
  turn: usize,
}

impl GoFishGame {
  pub fn new() -> Self {
    let mut deck = Deck::new();
    deck.shuffle();
    Self {
      deck,
      players: Vec::new(),
      turn: 0,
    }
  }

  pub fn add_player(&mut self, mut player: Player) {
    player.send_msg(&format!(
      "Welcome to Go Fish! You are player {}",
      self.players.len()
    ));
    self.players.push(player);
  }

  pub fn begin(&mut self) {
    self.clear_all();
    self.broadcast("The game is starting.");
    loop { }
  }

  fn clear_all(&mut self) {
    for player in self.players.iter_mut() {
      player.clear();
    }
  }

  fn broadcast(&mut self, msg: &str) {
    for player in self.players.iter_mut() {
      player.send_msg(msg);
    }
  }
}
