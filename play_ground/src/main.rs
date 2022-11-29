
fn main() {
    for i in 70..100 {
        println!("fib {} is {}", i, fib(i))
    }
}

fn fib(n: i64) -> i64 {
    if n == 0 {
        return 0;
    }
    else if n == 1 {
        return 1;
    }
    else {
        return fib(n - 1) + fib(n - 2);
    }

    // until ownership: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
}

