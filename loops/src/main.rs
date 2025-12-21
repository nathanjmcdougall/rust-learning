fn main() {
    /*/
    Like if-blocks, loops are also expressions. The value of a loop default to (),
    unless there is a `break` keyword, giving its value.

    There's also the `return` keyword to exit a function early from within a loop.

    ``break`` and ``return`` constructs conventionally end with semi-colons, but this
    is optional.
    */
    let mut count = 0;
    // outer loop is named 'counting_up
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                // Very cool feature - breaking a named loop.
                // Default is just to break the innermost loop.
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    /*
    While loops are provided for convenience, even though obviously the loop construct
    can be used to achieve the same effect. For example:
    */
    let mut number = 3;
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    /*
    for-in constructs are not just for convenience, they are treated specially by the
    compiler and are safer.

    They are the most commonly used loop construct in Rust.
    */
    for i in (1..4).rev() {
        // .rev() to reverse/ (1..4) to get a range from 1 to 3 (`std::ops::Range`)
        println!("{i}!");
    }
}
