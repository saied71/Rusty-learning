#[derive(Debug)]
enum Example{
    Float(f64),
    Int(i32),
    Text(String),
}

fn main() {
    let r = vec![
        Example::Int(23),
        Example::Float(3.45),
        Example::Text(String::from("asdasd"))
    ];

    println!("{:?}", r)
}