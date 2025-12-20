// You can install the rust-analyzer extension in VS Code to get linting and formatting
// The rust-analyzer in VS Code will annotate variables with their types in grey

use std::io; // namespace access is with :: and imports are with the `use` keyword

// Curly braces and semicolons everywhere :)
fn main() {
    println!("Guess the number!");

    println!("Please input your guess");

    // We want the string to be mutable so we can modify it with content from stdib
    //
    // Unlike Python, there are no classes in Rust so String is not a type. A method on
    // the type is not therefore called a class method but is rather called an
    // "associated function". So here, ::new() is an associated function on the String
    // type. As for imports/uses, :: gives us namespace access agains the String type.
    //
    // It's also worth touching on how some more advanced topics here. You might wonder
    // why we can't just write `let mut guess = ""` to create a new string. Rust has two
    // different types for strings: String and &str. The former is a heap-allocated,
    // growable string type while the latter is an immutable reference to a string
    // slice. Here, we need a String because we want to modify it later. The
    // `String::new()` function creates a new, empty String instance.
    //
    // Coming from a Python background, you might wonder what it means for something to
    // be heap-allocated. In Rust, values can be stored either on the stack or on the
    // heap. Stack allocation is faster and more efficient, but it has a limited size
    // and scope. Heap allocation, on the other hand, allows for dynamic memory
    // management and can handle larger data structures. Since strings can grow in size
    // and we want to read user input into it, we use a heap-allocated String here.
    //
    // You can read a bit more about the difference between heap- and stack- allocated
    // data here:
    // https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/the-stack-and-the-heap.html
    // This is an older version of the Rust book than the one I am using here:
    // https://rust-book.cs.brown.edu/ch02-00-guessing-game-tutorial.html
    let mut guess = String::new();

    io::stdin() // stdin() returns type Stdin which is a handle
        // Note the indentation style here
        // Also note we access the methods on the handle with . rather than ::
        // The distinction is similar to that of instance and class methods in Python.
        // &mut means a mutable reference - this will be explained later
        .read_line(&mut guess)
        // read_line returns a Result enum - Ok and Err. This .expect is a special
        // method on Result that will cause the program to crash with the given error
        // message if the Result is an Err variant.
        // The best way to deal with this is actually to write error-handling code,
        // not to simply crash, but we're keeping it simple for now.
        .expect("Failed to read line");

    println!("You guessed: {guess}") // N.B. string literals are like Python f-strings by default
    // you can also do this with positional arguments like so:
    // println!("You guessed: {}", guess);
}
