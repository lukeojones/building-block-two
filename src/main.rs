fn main() {
    println!("Hello, world!");
    let a = Move { direction: Direction::Up, distance: 0 };
    println!("The direction is {:?}", a);
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    distance: u32,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
