use manager::Manager;

mod bitmaps;
mod lcd;
mod manager;
mod numpad;
mod printer;

static KEYBOARD: &str = "/dev/input/event0";
static LCD: &str = "/dev/lcd";
static PRINTER: &str = "/dev/ttyAMA0";

fn main() {
  println!("[main] Creating manager");
  let mut man = Manager::new(KEYBOARD, LCD, PRINTER);
  println!("[main] Starting manager");
  man.run();
}
