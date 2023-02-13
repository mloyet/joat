use serde::{Deserialize, Serialize};
use std::net::TcpStream;

#[derive(Serialize, Deserialize)]
pub enum Message {
    HELLO,
    MSG(String),
    GOODBYE,
}

// These functions seem kind of silly, but since this is a shared library by both the server and
// the device, if we are ever to change the protocol to send data in a more sophisticated way, we
// will be able to make the change in one place.

pub fn send_msg(conn: &TcpStream, msg: Message) -> Result<(), serde_json::Error> {
    serde_json::to_writer(conn, &msg)
}

pub fn read_msg(conn: &TcpStream) -> Result<Message, serde_json::Error> {
    serde_json::from_reader(conn)
}
