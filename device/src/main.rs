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

static CAMERA: &str = "/home/pi/img/board.jpg";
static SCRIPT: &str = "/home/pi/joat/model/detection.py";

static SERVER: &str = "192.168.0.27:8000";

fn main() {
  println!("[main] Creating manager");
  let mut man = Manager::new(KEYBOARD, LCD, PRINTER, CAMERA, SCRIPT, SERVER);
  println!("[main] Starting manager");
  man.run();
}
