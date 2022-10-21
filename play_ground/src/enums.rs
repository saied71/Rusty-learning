#![allow(dead_code)]


#[derive(Debug)]
enum Direction {
    Up(Point),
    Down(Point),
    Left(Point),
    Right(Point),
}
#[derive(Debug)]
enum Keys {
    UpKey(String),
    DownKey(String),
    LeftKey(String),
    RightKey(String),
}

impl Direction {
    fn match_direction(&self) -> Keys {
        match *self {
            Direction::Up(_) => Keys::UpKey(String::from("press w")),
            Direction::Down(_) => Keys::DownKey(String::from("press s")),
            Direction::Left(_) => Keys::LeftKey(String::from("press a")),
            Direction::Right(_) => Keys::RightKey(String::from("press r")),
        }
    }
}

impl Keys {
    fn destruct(&self) -> &String {
        match *self {
            Keys::UpKey(ref s) => s,
            Keys::DownKey(ref s) => s,
            Keys::LeftKey(ref s) => s,
            Keys::RightKey(ref s) => s,
        }
    }
}
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let u = Direction::Up(Point {x: 0, y: 0});
    let k = u.match_direction();
    let x = k.destruct();

    println!("{}", x);
}
