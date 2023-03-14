use manager::Manager;

mod manager;
mod numpad;

// Not the actual name
static KEYBOARD: &str = "/dev/usb";

fn main() {
  println!("[main] Creating manager");
  let mut man = Manager::new(KEYBOARD);
  println!("[main] Starting manager");
  man.run();
}
