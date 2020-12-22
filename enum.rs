enum Direction {
    Up, 
    Down, 
    Left, 
    Right
}

impl Direction {
    fn as_str(&self) -> &'static str {
        match *self {
            Direction::Up => "up",
            Direction::Down => "down",
            Direction::Left => "left",
            Direction::Right => "right"
        }
    }
}

fn main() {
    println!("{}", Direction::Up.as_str());
}