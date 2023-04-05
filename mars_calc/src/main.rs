use std::io;

fn main() {
    println!("Enteryour weight: Kg");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let weight: f32 = input.trim().parse().unwrap();
    println!("Your weight on mars: {}kg", calc_mars_weght(weight));
}

fn calc_mars_weght(weight: f32) ->  f32 {
    return (weight * 3.73) / 9.81
}