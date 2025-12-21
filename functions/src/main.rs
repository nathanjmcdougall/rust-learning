fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// N.B. the distinction between "parameters" and "arguments". The parameters refer
// to the variables in the interface, i.e. the function defintion, whereas the arguments
// refer to the actual "concrete values" passed to the function when it is called.
// However, like in Python, its common for this distinction to be blurred in casual
// conversation. Actually, in Python the Google Docstring style uses "Args:" as the
// section header for documenting the parameters of a function 👀

// Another important distinction is that of "statement" and "expression". Unlike Python
// where everything is an expression, there are some constructs in Rust that are not
// expressions, i.e. they do not return a value. Function definitions and `let`
// statements do not return a value; they are statements.
// Statements end in semicolons. Expressions do not. If you put a semicolon at the end
// of an expression, you turn it into a statement!

// Lastly, it's worth noting that curly braces denote a so-called "syntactic scope", but
// they are expressions since they evaluate to the value of the last expression in the
// block.
