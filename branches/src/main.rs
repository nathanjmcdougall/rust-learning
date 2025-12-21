fn main() {
    let number = 7;

    /* */
    The different branches of the if-statement, just like the match-statement, are
    called "arms".
    N.B. in Rust, too many arms in an if-statement is considered a code smell;
    refactoring to use a match-statement is likely better.
    Unlike Python, there's not "truthy" or "falsy" evaluation - conditionals must
    always be boolean.
    */
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        // N.B> else if not elif or elseif.
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    /*
    Also note; if-statements are expressions. They evaluate to the value of the last
    expression in the executed branch. This can be used for ternary-expression-like
    behaviour. But values must be the same type!

    Lastly, note that let-statements don't need to include an assignment.
    You can do this:
    */
    // let x;
    // if cond {
    // x = 1;
    // } else {
    // x = 2;
    // }
    /*
    And Rust will 
    */
}
