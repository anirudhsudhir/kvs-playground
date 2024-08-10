use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::prelude::*;

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
    let moves = vec![
        Move {
            direction: Direction::Up,
            positions: 2,
        },
        Move {
            direction: Direction::Right,
            positions: 4,
        },
    ];

    let moves_json = serde_json::to_string(&moves).unwrap();
    let mut file_write = File::create("data.json").unwrap();
    file_write.write_all(moves_json.as_bytes()).unwrap();

    let sen = fs::read_to_string("data.json").unwrap();
    let moves_read: Vec<Move> = serde_json::from_str(&sen).unwrap();
    println!("{:?}", moves_read);
}
