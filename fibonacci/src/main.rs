use std::io;

const MAX_N: u64 = 93; // To avoid overflows

fn main() {
    println!("Let's calculate the n-th Fibonacci number!");

    let mut n: u64;
    let mut n_str: &str;
    let mut input;
    loop {
        println!("Please enter n:");
        input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(_) => {
                request_new_choice();
                continue;
            }
        };
        n_str = input.trim();
        n = match n_str.parse() {
            Ok(n) => n,
            Err(_) => {
                request_new_choice();
                continue;
            }
        };
        if (n == 0) || (n >= MAX_N) {
            request_new_choice();
            continue;
        }

        break;
    }

    let fib = fibonacci(n);
    // .next() and .next_back() to access the first and last char of a double-ended
    // iterator trait
    let ord_suffix = match n_str.chars().next_back() {
        Some('1') => "st",
        Some('2') => "nd",
        Some('3') => "rd",
        _ => "th",
    };
    println!("The {n}{ord_suffix} Fibonacci number is: {fib}");
}
fn request_new_choice() {
    println!("Invalid n. Please enter an integer > 0, < {MAX_N}:");
}

fn fibonacci(k: u64) -> u64 {
    let mut f = 0; // f(i)
    let mut f1 = 0; // f(i-1)
    let mut f2 = 0; // f(i-2)
    for _ in 0..k {
        f2 = match f2 {
            _ => f1,
        };
        f1 = match f1 {
            0 => 1,
            _ => f,
        };
        f = f1 + f2;
    }
    f
}
