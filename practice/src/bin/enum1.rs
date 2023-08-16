enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn which_way(go: Direction) -> &'static str {
    match go {
        Direction::Up => "up",
        Direction::Down => "down",
        Direction::Left => "left",
        Direction::Right => "right",
    }
}

fn main() {
    let dir = Direction::Up;
    println!("{}", which_way(dir));
}
