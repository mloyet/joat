use manager::Manager;

mod manager;
mod numpad;

// Is this always the correct device?
static KEYBOARD: &str = "/dev/input/event0";

fn main() {
  println!("[main] Creating manager");
  let mut man = Manager::new(KEYBOARD);
  println!("[main] Starting manager");
  man.run();
}
