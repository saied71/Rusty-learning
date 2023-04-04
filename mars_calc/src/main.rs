use stds::io;

fn main() {

    let mut input = String::new();
    io::stdin().readline(&mut input);
    input = input.parse::<f32>;
    println!("Your weight on mars: {}kg", calc_mars_weght(input));
}

fn calc_mars_weght(weight: f32) ->  f32 {
    return (weight * 3.73) / 9.81
}