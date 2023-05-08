#![allow(dead_code)]

use std::{collections::HashSet, thread, time::Duration};

use protocol::{Card, Rank};

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
    self.deal();
    loop {
      self.do_turn();
    }
  }

  fn deal(&mut self) {
    for player in self.players.iter_mut() {
      for _ in 0..5 {
        player.send_card(self.deck.draw().unwrap());
      }
    }
  }

  fn do_turn(&mut self) {
    let len = self.players.len();
    let cur = &mut self.players[self.turn];

    // Target selection loop.
    cur.clear();
    cur.send_msg("Who do you want to take from?");
    let target: usize;
    loop {
      let sel = cur.read_input();
      match sel.parse::<usize>() {
        Err(_) => {
          cur.clear();
          cur.send_msg("Put in an integer.");
        }
        Ok(i) => {
          if i < len && i != self.turn {
            target = i;
            break;
          }

          if i != self.turn {
            cur.clear();
            cur.send_msg(&format!("Pick a player between 0 and {}.", len - 1))
          } else {
            cur.clear();
            cur.send_msg("Can't pick yourself. Pick again.")
          }
        }
      }
    }

    // Card selection loop.
    cur.clear();
    cur.send_msg(&format!("What would you like to take from {}?", target));
    let rank: Rank;
    loop {
      let sel = cur.read_input();
      match sel.parse::<usize>() {
        Err(_) => {
          cur.clear();
          cur.send_msg("Put in an integer.");
        }
        Ok(i) => {
          if i > 13 || i == 0 {
            cur.clear();
            cur.send_msg("Pick a card between 1 and 13");
            continue;
          }

          // Make sure the player has the card.
          let r: Rank = i.into();
          let mut found = false;
          for Card(_, rank) in cur.hand.iter() {
            if &r == rank {
              found = true;
              break;
            }
          }

          if !found {
            cur.clear();
            cur.send_msg("Must request a rank which you have.");
            continue;
          }
          rank = r;
          break;
        }
      }
    }

    // Figure out if it is a "hit"
    let target = &mut self.players[target];
    let mut expected = HashSet::new();
    for c @ Card(_, r) in target.hand.iter() {
      if &rank == r {
        expected.insert(c.clone());
      }
    }

    if expected.len() == 0 {
      // Go fish logic.
      let cur = &mut self.players[self.turn];
      cur.clear();
      cur.send_msg("Sorry... Go fish!");
      match self.deck.draw() {
        Some(c) => cur.send_card(c),
        None => cur.send_msg("Oh no, no more cards!"),
      }
      // Advance the turn.
      self.turn = (self.turn + 1) % self.players.len();
    } else {
      // Getting cards loop.
      target.clear();
      target.send_msg(&format!("Give up all of your {}s", rank));
      loop {
        thread::sleep(Duration::new(1,0));
        let cards = target.read_table();
        if cards.len() != expected.len() {
          continue;
        }
        let mut matches = true;
        for card in cards {
          if !expected.contains(&card) {
            matches = false;
          }
        }
        if !matches {
          continue;
        }
        break;
      }
      target.send_msg("Thank you! Please discard the cards now.");

      // Giving cards to requestor
      let cur = &mut self.players[self.turn];
      cur.clear();
      cur.send_msg("You have received cards!");
      for card in expected.drain() {
        cur.send_card(card);
      }
    }
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
