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
        if !(1..MAX_N).contains(&n) {
            request_new_choice();
            continue;
        }

        break;
    }

    let fib = fibonacci(n);
    // .next() and .next_back() to access the first and last char of a double-ended
    // iterator trait
    let ord_suffix = match n_str {
        s if s.ends_with("11") || s.ends_with("12") || s.ends_with("13") => "th",
        _ => match n_str.chars().next_back() {
            Some('1') => "st",
            Some('2') => "nd",
            Some('3') => "rd",
            _ => "th",
        },
    };
    println!("The {n}{ord_suffix} Fibonacci number is: {fib}");
}
fn request_new_choice() {
    println!("Invalid n. Please enter an integer > 0, < {MAX_N}:");
}

fn fibonacci(k: u64) -> u64 {
    let (mut f, mut fminus1) = (0, 1); // f(i), f(i-1)
    for _ in 1..k {
        let fplus1 = f + fminus1; // f(i+1)
        fminus1 = f;
        f = fplus1;
    }
    f
}
