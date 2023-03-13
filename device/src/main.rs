use manager::Manager;

mod manager;

// Not the actual name
static KEYBOARD: &str = "/dev/usb";

fn main() {
  let mut man = Manager::new(KEYBOARD);
  man.run();
}
