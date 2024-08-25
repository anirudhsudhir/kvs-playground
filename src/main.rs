use rmp_serde::{self, decode, encode};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, Seek, SeekFrom, Write};
use std::path;

#[derive(Debug)]
pub enum KvsError {
    IoError(io::Error),
    SerializationError(encode::Error),
    DeserializationError(decode::Error),
    MapError(String),
    KeyNotFoundError,
    CliError(String),
    OtherError(String),
}

#[derive(Debug, Serialize, Deserialize)]
enum OperationType {
    Set(String, String),
    Rm(String),
}

#[derive(Debug, Serialize, Deserialize)]
struct LogCommand {
    operation: OperationType,
}

fn main() {
    let cmd = LogCommand {
        operation: OperationType::Set(
            String::from("this is a key"),
            String::from("This is a value"),
        ),
    };

    let bson_data = bson::to_vec(&cmd).unwrap();
    let msg_pack_data = rmp_serde::to_vec(&cmd).unwrap();

    let mut msg_pack_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("./msg_pack.db")
        .unwrap();

    let mut bson_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("./bson.db")
        .unwrap();

    msg_pack_file.write_all(&msg_pack_data).unwrap();
    bson_file.write_all(&bson_data).unwrap();
}
