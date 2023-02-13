use std::{io::{Write, Read}, net::TcpStream, thread, time::Duration};

fn main() -> std::io::Result<()> {
    let mut conn = TcpStream::connect("127.0.0.1:8000")?;
    let mut buf : [u8;1024] = [0;1024];

    loop {
        println!("Sending \"Hello\".");
        conn.write(b"Hello")?;
        println!("Sent. Waiting...");
        thread::sleep(Duration::from_millis(100));
        println!("Attempting to read");
        let amt = conn.read(&mut buf)?;
        println!("Read {} bytes: {}", amt, std::str::from_utf8(&buf[0..amt]).unwrap());
    }
}
