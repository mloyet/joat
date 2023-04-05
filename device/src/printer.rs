use std::{
  cmp::min,
  fs::{File, OpenOptions},
  io::Write,
  thread,
  time::Duration, sync::mpsc::Receiver,
};

use protocol::{Card, Rank, Suit};

use crate::bitmaps::{CLUBS, DIAMONDS, HEARTS, JACK, KING, QUEEN, SPADES};

pub struct Printer {
  file: File,
  receiver: Receiver<Card>,
}

impl Printer {
  pub fn start(filename: &str, receiver: Receiver<Card>) {
    let file = OpenOptions::new().write(true).open(filename).unwrap();
    let mut printer = Self { file, receiver };
    thread::Builder::new()
      .name("printer".to_string())
      .spawn(move || printer.run())
      .unwrap();
  }

  /// Private worker thread loop
  fn run(&mut self) {
    self.initialize();
    loop {
      let card = self.receiver.recv().unwrap();
      self.write_card(&card);
    }
  }

  fn initialize(&mut self) {
    // wake()
    self.write1(255); // Wake
    self.wait();
    self.write4(27, 56, 0, 0); // Sleep off

    // reset()
    self.write2(27, 64);

    self.write2(27, 68); // Set tab stops...
    self.write4(4, 8, 12, 16); // ...every 4 columns,
    self.write4(20, 24, 28, 0); // 0 marks end-of-list.
    self.wait();
  }

  fn write_card(&mut self, card: &Card) {
    use Suit::*;
    match card {
      Card(SPADE, r) => self.write_suit(SPADES, SPADE, r),
      Card(CLUB, r) => self.write_suit(CLUBS, CLUB, r),
      Card(HEART, r) => self.write_suit(HEARTS, HEART, r),
      Card(DIAMOND, r) => self.write_suit(DIAMONDS, DIAMOND, r),
    };
  }

  fn write_suit(&mut self, suits: &[&[u8]], suit: Suit, rank: &Rank) {
    self.align_left();
    self.write(format!("{}\n", to_char(rank)));
    self.write(format!("{} of {}\n", rank, suit));

    match rank {
        Rank::ACE => {
          self.write("\n\n\n\n".to_string());
          self.print_bitmap(376, suits[0]);
          self.write("\n\n\n\n".to_string());
        },
        Rank::TWO => {
          self.write("\n\n".to_string());
          self.print_bitmap(376, suits[0]);
          self.print_bitmap(376, suits[0]);
          self.write("\n\n".to_string());
        },
        Rank::THREE => {
          self.print_bitmap(376, suits[0]);
          self.print_bitmap(376, suits[0]);
          self.print_bitmap(376, suits[0]);
        },
        Rank::FOUR => {
          self.write("\n".to_string());
          self.print_bitmap(376, suits[1]);
          self.write("\n".to_string());
          self.print_bitmap(376, suits[1]);
          self.write("\n".to_string());
        },
        Rank::FIVE => {
          self.print_bitmap(376, suits[1]);
          self.print_bitmap(376, suits[0]);
          self.print_bitmap(376, suits[1]);
        },
        Rank::SIX => {
          self.print_bitmap(376, suits[1]);
          self.print_bitmap(376, suits[1]);
          self.print_bitmap(376, suits[1]);
        },
        Rank::SEVEN =>{
          self.print_bitmap(376, suits[1]);
          self.print_bitmap(376, suits[2]);
          self.print_bitmap(376, suits[1]);
        },
        Rank::EIGHT => {
          self.print_bitmap(376, suits[2]);
          self.print_bitmap(376, suits[2]);
          self.print_bitmap(376, suits[1]);
        },
        Rank::NINE => {
          self.print_bitmap(376, suits[2]);
          self.print_bitmap(376, suits[2]);
          self.print_bitmap(376, suits[2]);
        },
        Rank::TEN => {
          self.print_bitmap(376, suits[1]);
          self.print_bitmap(376, suits[2]);
          self.print_bitmap(376, suits[1]);
          self.print_bitmap(376, suits[2]);
        },
        Rank::JACK => {
          self.print_bitmap(376, JACK);
          self.print_bitmap(376, suits[0]);
        },
        Rank::QUEEN => {
          self.print_bitmap(376, QUEEN);
          self.print_bitmap(376, suits[0]);
        },
        Rank::KING => {
          self.print_bitmap(376, KING);
          self.print_bitmap(376, suits[0]);
        },
    }

    self.align_right();
    self.write(format!("{} of {}\n", rank, suit));
    self.write(format!("{}\n", to_char(rank)));
  }

  /// Print a bitmap in ~255 byte stripes.
  fn print_bitmap(&mut self, w: usize, bitmap: &[u8]) {
    let stripe_width: u8 = (w >> 3).try_into().unwrap();
    let stripe_height = 255 / stripe_width;

    // Split the image into chunks to send to the printer.
    let offsets = (0..bitmap.len()).step_by(stripe_width as usize * stripe_height as usize);
    for offset in offsets {
      let chunk_height = min(
        stripe_height.into(),
        (bitmap.len() - offset) / stripe_width as usize,
      );
      let chunk_length: usize = stripe_width as usize * chunk_height;

      let start: usize = offset;
      let end: usize = offset + chunk_length;

      self.begin_bitmap(stripe_width, chunk_height.try_into().unwrap());
      self.file.write_all(&bitmap[start..end]).unwrap();

      // Sloppy.
      self.wait();
    }
  }

  /// ASCII_DC2-*-height-width command widget.
  fn begin_bitmap(&mut self, w: u8, h: u8) {
    self.file.write_all(&[18, 42, h, w]).unwrap();
  }

  fn align_left(&mut self) {
    self.write3(27, 97, 0);
  }

  fn align_right(&mut self) {
    self.write3(27, 97, 2);
  }

  fn write1(&mut self, b: u8) {
    self.file.write_all(&[b]).unwrap();
  }

  fn write2(&mut self, b1: u8, b2: u8) {
    self.file.write_all(&[b1, b2]).unwrap();
  }

  fn write3(&mut self, b1: u8, b2: u8, b3: u8) {
    self.file.write_all(&[b1, b2, b3]).unwrap();
  }

  fn write4(&mut self, b1: u8, b2: u8, b3: u8, b4: u8) {
    self.file.write_all(&[b1, b2, b3, b4]).unwrap();
  }

  fn write(&mut self, s: String) {
    self
      .file
      .write_all(s.as_bytes())
      .unwrap();
  }

  fn wait(&self) {
    thread::sleep(Duration::from_millis(100));
  }
}

fn to_char(r: &Rank) -> String {
  match r {
    Rank::ACE => "A",
    Rank::TWO => "2",
    Rank::THREE => "3",
    Rank::FOUR => "4",
    Rank::FIVE => "5",
    Rank::SIX => "6",
    Rank::SEVEN => "7",
    Rank::EIGHT => "8",
    Rank::NINE => "9",
    Rank::TEN => "10",
    Rank::JACK => "J",
    Rank::QUEEN => "Q",
    Rank::KING => "K",
  }
  .to_string()
}