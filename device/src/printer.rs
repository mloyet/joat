use std::{
  cmp::min,
  fs::{File, OpenOptions},
  io::Write,
  thread,
  time::Duration,
};

use crate::bitmaps::{KING, SPADE};

pub struct Printer {
  file: File,
}

impl Printer {
  pub fn start(filename: &str) {
    let file = OpenOptions::new().write(true).open(filename).unwrap();
    let mut printer = Self { file };
    thread::Builder::new()
      .name("printer".to_string())
      .spawn(move || printer.run())
      .unwrap();
  }

  /// Private worker thread loop
  fn run(&mut self) {
    self.initialize();
    self.print_bitmap(376, SPADE);
    self.print_bitmap(376, KING);
  }

  fn write1(&mut self, b: u8) {
    self.file.write_all(&[b]).unwrap();
  }

  fn write2(&mut self, b1: u8, b2: u8) {
    self.file.write_all(&[b1, b2]).unwrap();
  }

  fn write4(&mut self, b1: u8, b2: u8, b3: u8, b4: u8) {
    self.file.write_all(&[b1, b2, b3, b4]).unwrap();
  }

  fn wait(&self) {
    thread::sleep(Duration::from_millis(100));
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
    self
      .file
      .write_all("Initialization Complete.\n".as_bytes())
      .unwrap();
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
}
