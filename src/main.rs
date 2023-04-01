use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

fn main() {
    println!("Hello, world!");
    let a = Move { direction: Direction::Up, distance: 0 };
    let a_json = serde_json::to_string(&a).unwrap();
    fs::write("output.json", a_json).expect("Unable to write to file");
    println!("The direction is {:?}", a);
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
