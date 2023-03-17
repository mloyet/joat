use manager::Manager;

mod manager;
mod numpad;
mod lcd;

static KEYBOARD: &str = "/dev/input/event1";
static LCD: &str = "/dev/lcd";

fn main() {
  println!("[main] Creating manager");
  let mut man = Manager::new(KEYBOARD, LCD);
  println!("[main] Starting manager");
  man.run();
}
