
fn main() {
    
    let x = x_plus(12);

    println!("{x}")
    // until control-flow: https://doc.rust-lang.org/book/ch03-05-control-flow.html
}

fn x_plus(x: i32) -> i32 {
    x + 20
}