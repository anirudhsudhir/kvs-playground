use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(Debug, Serialize, Deserialize)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Serialize, Deserialize)]
struct Move {
    direction: Direction,
    positions: u32,
}

fn main() {
    let vals = bson::to_vec(&Move {
        direction: Direction::Up,
        positions: 2,
    })
    .unwrap();

    let mut buf = Vec::with_capacity(1000);

    for _ in 1..1000 {
        buf.extend_from_slice(&vals.clone());
    }

    let mut reader = Cursor::new(buf);

    let mut parsed_bson: Vec<bson::Bson> = Vec::with_capacity(1000);

    while let Ok(doc) = bson::Document::from_reader(&mut reader) {
        parsed_bson.push(bson::from_document(doc).unwrap());
    }

    println!("{:?}", parsed_bson);
}
