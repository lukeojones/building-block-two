use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use ron;

fn main() {
    // exercise_one();
    // exercise_two();
    exercise_two_ron();
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
