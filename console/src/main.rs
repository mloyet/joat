//! The console is a means to send arbitrary messages to a device
//!
//! Currently doesn't really make an effort to receive messages.

use std::{
  io::{self, BufRead, Write},
  net::TcpListener,
};

use protocol::{Card, Message, Protocol, Rank, Suit};

const HELP: &str = "Commands:
    clear                       Clear the device screen.
    scan                        Scan the camera.
    read                        Read line from the keyboard.
    print_card <rank> <suit>    Print out a card on the receipt printer.
    print <msg>                 Print a message to the LCD screen.";

fn main() -> io::Result<()> {
  let listener = TcpListener::bind("0.0.0.0:8000")?;

  for stream in listener.incoming() {
    let stream = stream?;
    println!("Attached to client {:?}", stream);

    let mut prot = Protocol::new(stream);

    let mut line = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    println!("{}", HELP);

    loop {
      print!("> ");
      io::stdout().flush().unwrap();
      line.clear();

      use Message::*;
      handle.read_line(&mut line)?;
      line.pop(); // remove newline.

      // check for the single word commands.
      if line == "clear" {
        prot.send_msg(Clear).unwrap();
        continue;
      }
      if line == "scan" {
        prot.send_msg(RequestScan).unwrap();
        continue;
      }
      if line == "read" {
        prot.send_msg(ReadInput).unwrap();
        continue;
      }
      if line == "help" {
        println!("{}", HELP);
      }

      // check for commands with arguments.
      if let Some((cmd, arg)) = line.split_once(" ") {
        if cmd == "print_card" {
          if let Some((card, suit)) = arg.split_once(" ") {
            use Rank::*;
            use Suit::*;
            let rank = match card {
              "A" => ACE,
              "2" => TWO,
              "3" => THREE,
              "4" => FOUR,
              "5" => FIVE,
              "6" => SIX,
              "7" => SEVEN,
              "8" => EIGHT,
              "9" => NINE,
              "10" => TEN,
              "J" => JACK,
              "Q" => QUEEN,
              "K" => KING,
              _ => {
                println!("Enter valid card: 1-10JQKA.");
                continue;
              }
            };
            let suit = match suit {
              "H" => HEART,
              "D" => DIAMOND,
              "S" => SPADE,
              "C" => CLUB,
              _ => {
                println!("Enter valid suit: HDSC.");
                continue;
              }
            };
            prot.send_msg(PrintCard(Card(suit, rank))).unwrap();
          }
        } else if cmd == "print" {
          prot.send_msg(Print(arg.to_string())).unwrap();
          continue;
        }
      }

      println!("Unrecognized command: {}", line);
      println!("{}", HELP);
    }
  }
  Ok(())
}
