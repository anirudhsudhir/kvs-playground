use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{prelude::*, BufWriter};

#[derive(Debug, Serialize, Deserialize)]
enum Direction {
    Up(String),
    Down(bool),
    Left(char),
    Right,
}

#[derive(Debug, Serialize, Deserialize)]
struct Move {
    direction: Direction,
    positions: u32,
}

fn main() {
    let vals_up = Move {
        direction: Direction::Up(String::from(
            "looooooooooong stringgggggg vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv",
        )),
        positions: 2,
    };

    let mut flex_serial_up = flexbuffers::FlexbufferSerializer::new();
    vals_up.serialize(&mut flex_serial_up).unwrap();

    let vals_down = Move {
        direction: Direction::Down(true),
        positions: 100,
    };

    let mut flex_serial_down = flexbuffers::FlexbufferSerializer::new();
    vals_down.serialize(&mut flex_serial_down).unwrap();
    {
        // if Path::exists(Path::new("data")) {
        //     fs::remove_file("data").unwrap();
        // }
        let file = fs::OpenOptions::new().append(true).open("data").unwrap();
        let mut writer = BufWriter::new(file);

        for _ in 1..3 {
            let temp = flex_serial_up.view();
            println!("len = {}", temp.len());
            writer.write_all(temp).unwrap();
        }
        for _ in 1..3 {
            let temp = flex_serial_up.view();
            println!("len = {}", temp.len());
            writer.write_all(temp).unwrap();
        }
        for _ in 1..3 {
            let temp = flex_serial_up.view();
            println!("len = {}", temp.len());
            writer.write_all(temp).unwrap();
        }
    }

    // let mut vals: Vec<Move> = Vec::new();
    //
    // let mut file = File::open("data").unwrap();
    // let mut buf = Vec::new();
    // file.read_to_end(&mut buf).unwrap();
    // let mut reader = Cursor::new(buf);
    // // let read = buf.as_slice();
    //
    // // reader.
    //
    // while let Ok(move_val) = flexbuffers::Reader::get_root(reader) {
    //     vals.push(Move::deserialize(move_val).unwrap());
    //     reader.seek()
    // }
    //
    // println!("{:?}", vals);
}
