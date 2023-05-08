#![allow(dead_code)]

use protocol::Card;
use rand::Rng;

pub struct Deck {
  order: [usize; 52],
  pos: usize,
}

impl Deck {
  pub fn new() -> Self {
    let mut res = Deck {
      order: [0; 52],
      pos: 0,
    };
    for i in 0..52 {
      res.order[i] = 0;
    }
    res
  }

  pub fn shuffle(&mut self) {
    self.pos = 0;
    for i in 0..51 {
      let j = rand::thread_rng().gen_range((i+1)..52);
      let t = self.order[i];
      self.order[i] = self.order[j];
      self.order[j] = t;
    }
  }

  pub fn draw(&mut self) -> Option<Card> {
    if self.pos >= 52 {
      return None
    }

    let n = self.order[self.pos];
    self.pos += 1;
    let suit = (n / 13).into();
    let rank = ((n % 13) + 1).into();

    Some(Card(suit, rank))
  }
}
