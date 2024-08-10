use serde::{Deserialize, Serialize};
// use std::fs::{self, File};
// use std::io::prelude::*;

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

    let mut moves_ron = Vec::new();
    ron::ser::to_writer(&mut moves_ron, &moves).unwrap();
    let moves_read: Vec<Move> = ron::de::from_bytes(&moves_ron).unwrap();
    println!(
        "Deserialized: {:?}, Ron: {:?}",
        moves_read,
        std::str::from_utf8(&moves_ron).unwrap()
    );
}
