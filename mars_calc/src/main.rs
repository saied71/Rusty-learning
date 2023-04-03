fn main() {
    println!("Your weight on mars: {}kg", calc_mars_weght(79.0));
}

fn calc_mars_weght(weight: f32) ->  f32 {
    return (weight * 3.73) / 9.81
}