use std::{io, net::TcpStream};

use protocol::{Card, Message, Protocol, Rank, Suit};

static SERVER: &str = "192.168.0.27:8000";

fn main() {
  let mut prot = Protocol::new(TcpStream::connect(SERVER).unwrap());

  loop {
    match prot.read_msg().unwrap() {
      Message::Print(s) => println!("{}", s),
      Message::Clear => println!(),
      Message::PrintCard(c) => println!("You were dealt a {}", c),
      Message::ReadInput => {
        print!("> ");
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        prot.send_msg(Message::Line(buf)).unwrap();
      }
      Message::RequestScan => {
        println!("Type the cards (suit then rank)");
        print!("> ");
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        let mut words = buf.split(" ");
        let mut cards = Vec::new();

        loop {
          match (words.next(), words.next()) {
            (Some(s), Some(r)) => {
              println!("({} {})", s, r);
              use Rank::*;
              use Suit::*;
              let rank = match r {
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
              let suit = match s {
                "H" => HEART,
                "D" => DIAMOND,
                "S" => SPADE,
                "C" => CLUB,
                _ => {
                  println!("Enter valid suit: HDSC.");
                  continue;
                }
              };
              cards.push(Card(suit, rank));
            }
            _ => {
              prot.send_msg(Message::DetectedCards(cards)).unwrap();
              break;
            }
          }
        }
      }
      Message::DEBUG(_) => {}
      Message::ERR(_) => {}
      Message::ACK => {}
      Message::Line(_) => panic!(),
      Message::DetectedCards(_) => panic!(),
    }
  }
}
