use adsb_deku::Frame;
use deku::DekuContainerRead;

use std::io::{BufRead, BufReader};
use std::net::TcpStream;

fn main() {
    let stream = TcpStream::connect(("127.0.0.1", 30002)).unwrap();
    let mut reader = BufReader::new(stream);
    let mut input = String::new();
    loop {
        let len = reader.read_line(&mut input).unwrap();
        let hex = &input.to_string()[1..len - 2];
        println!("{}", hex);
        let bytes = hex::decode(&hex).unwrap();
        println!("{:#?}", Frame::from_bytes((&bytes, 0)).unwrap().1);
        input.clear();
    }
}
