use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{prelude::*, BufWriter, Cursor};
use std::path::Path;

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

    {
        if Path::exists(Path::new("data.bson")) {
            fs::remove_file("data.bson").unwrap();
        }
        let file = File::create_new("data.bson").unwrap();
        let mut writer = BufWriter::new(file);

        for _ in 1..1000 {
            writer.write_all(&vals).unwrap();
        }
    }

    let mut file = File::open("data.bson").unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    let mut reader = Cursor::new(buf);

    let mut parsed_bson: Vec<bson::Bson> = Vec::with_capacity(1000);

    while let Ok(doc) = bson::Document::from_reader(&mut reader) {
        parsed_bson.push(bson::from_document(doc).unwrap());
    }

    println!("{:?}", parsed_bson);
}
