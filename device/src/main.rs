use manager::Manager;

mod bitmaps;
mod camera;
mod lcd;
mod manager;
mod numpad;
mod printer;

static KEYBOARD: &str = "/dev/input/event0";
static LCD: &str = "/dev/lcd";
static PRINTER: &str = "/dev/ttyAMA0";
static CAMERA: &str = "/dev/ttyAMA0";

static SCRIPT: &str = "~/joat/model/detection.py";

fn main() {
  println!("[main] Creating manager");
  let mut man = Manager::new(KEYBOARD, LCD, PRINTER, CAMERA, SCRIPT);
  println!("[main] Starting manager");
  man.run();
}
