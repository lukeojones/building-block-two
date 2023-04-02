use std::error::Error;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use ron;
use bson;
use bson::Document;

fn main() {
    // exercise_one();
    // exercise_two();
    // exercise_two_ron();
    // exercise_three_single_move();
    exercise_three_multi_move();
}


fn exercise_three_multi_move() {
    println!();
    println!("<<< Exercise Three >>>");
    let moves: Vec<Move> = (1..=1000)
        .map(|n| Move { direction: Direction::Up, distance: n})
        .collect();

    let path = "output_three.bson";
    let file = File::create(&path).expect(&*format!("Unable to create file: {}", path));

    for m in moves {
        let bson = bson::to_bson(&m).unwrap();
        let document = bson.as_document().unwrap();
        println!("{:?}", &document);
        document.to_writer(&file).expect("Unable to write to file");
    }

    let mut file = File::open(path).expect("Unable to open file");
    let mut deserialized_moves = Vec::new();
    loop {
        match bson::Document::from_reader(&mut file) {
            Ok(bson_document) => {
                let deserialized_move: Move = bson::from_document(bson_document).unwrap();
                deserialized_moves.push(deserialized_move);
            }
            Err(e) => {
                if let Some(io_error) = e.source().and_then(|source| source.downcast_ref::<std::io::Error>()) {
                    if let ErrorKind::UnexpectedEof = io_error.kind() {
                        break; // End of file, exit the loop
                    }
                } else {
                    panic!("Issue deserializing {}", e);
                }
            }
        }
    }

    for (index, m) in deserialized_moves.iter().enumerate() {
        println!("Move {} is {:?}", index + 1, m);
    }
}

fn exercise_three_single_move() {
    println!();
    println!("<<< Exercise Three (Single Move) >>>");

    let path = "output_three.bson";
    let file = File::create(&path).expect(&*format!("Unable to create file: {}", path));

    println!("Writing BSON to file");
    let m = Move { direction: Direction::Up, distance: 0};
    let bson = bson::to_bson(&m).unwrap();
    let document = bson.as_document().unwrap();
    println!("{:?}", &document);
    document.to_writer(&file).expect("Unable to write to file");

    println!("Reading BSON from file");
    let mut file = File::open(path).expect("Unable to open file");
    let document = Document::from_reader(file).unwrap();
    let obj: Move = bson::from_document::<Move>(document).unwrap();
    println!("OLD: {:?}", &m);
    println!("NEW: {:?}", &obj);
}

fn exercise_one() {
    println!();
    println!("<<< Exercise One >>>");
    let path = "output.json";
    let a = Move { direction: Direction::Up, distance: 0 };
    println!("Serializing {:?}...", a);

    let a_json = serde_json::to_string(&a).unwrap();
    fs::write(path, a_json).expect("Unable to write to file");

    println!("Deserializing from {}...", path);
    let b_json = fs::read_to_string(path).expect("Unable to read from file");
    let b: Move = serde_json::from_str(&b_json).unwrap();

    println!("Result of deserialization is {:?}...", b);
}

fn exercise_two() {
    println!();
    println!("<<< Exercise Two >>>");
    let a = Move { direction: Direction::Up, distance: 0 };

    println!("Serializing to buffer {:?}...", a);
    let buffer = serde_json::to_vec(&a).unwrap();

    println!("Deserializing from buffer");
    let b: Move = serde_json::from_slice(&buffer).unwrap();

    println!("Result of deserialization is {:?}...", b);
}

fn exercise_two_ron() {
    println!();
    println!("<<< Exercise Two - RON >>>");
    let path = "output_two.json";
    let a = Move { direction: Direction::Up, distance: 0 };
    println!("Serializing (RON) {:?}...", a);

    let a_ron = ron::ser::to_string(&a).unwrap();
    fs::write(path, a_ron).expect("Unable to write to file");

    println!("Deserializing from {}...", path);
    let b_ron = fs::read_to_string(path).expect("Unable to read from file");
    let b: Move = ron::de::from_str(&b_ron).unwrap();

    println!("Result of deserialization (RON) is {:?}...", b);
}

#[derive(Debug, Serialize, Deserialize)]
struct Move {
    direction: Direction,
    distance: u32,
}

#[derive(Debug, Serialize, Deserialize)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
