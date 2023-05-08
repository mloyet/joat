mod deck;
mod player;
mod gofish;

use gofish::GoFishGame;
use protocol::Protocol;
use std::{fs::File, net::TcpListener};

use crate::player::Player;

fn main() -> std::io::Result<()> {
  let listener = TcpListener::bind("0.0.0.0:8000")?;

  let mut game = GoFishGame::new();
  let mut count = 0;

  for stream in listener.incoming() {
    let stream = stream?;
    println!("Attached to client {:?}", stream);

    let mut prot = Protocol::new(stream);
    let logfile = File::create(format!("/home/pi/logs/server{}.log", count))?;
    prot.attach_logfile(logfile);

    let mut player = Player::new(prot);
    player.clear();

    game.add_player(player);
    count += 1;

    // Not the best criteria in general, but it'll do for our system
    if count == 2 {
      game.begin();
      game = GoFishGame::new();
      count = 0;
    }
  }

  Ok(())
}
