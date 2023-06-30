// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.

// By wrapping a value within Some, Rust enforces explicit handling of the presence or absence of a value, 
// promoting safer code and avoiding unexpected null-related errors.

// In summary, this code checks if the `option` variable holds a `Some` value and, 
// if so, adds its inner value to `res`. The final value of `res` is then printed to the console.

fn main() {
    //Declares a mutable variable `res` and initializes it with the value `42`.
    let mut res = 42;

    // Declares a variable `option` and assigns it the value `Some(12)`. 
    // In Rust, `Some` is an enum variant used to represent a value that exists.
    let option = Some(12);

    // Pattern matching 1:
    // If option is None, unwrap will panic, causing the program to terminate with an error.
    let value = option.unwrap();
    println!("{}", value);

    // // Pattern matching 2:
    // Uses the `if let` construct to pattern match the `option` variable. 
    // It checks if `option` contains a `Some` variant and binds its inner value to the variable `x`.
    match option {
        Some(value) => {
            println!("{}", value);
        },
        None => {
            println!("None")
        }
    }

    // // Pattern matching 3:
    // If the pattern match is successful (i.e., `option` is `Some`), it executes the code block inside the if let statement.
    if let Some(x) = option {
        // The code block increments `res` by the value of `x` using the `+=` operator
        res += x
    }
    // prints the value of `res` using `println!` macro
    println!("{}", res);
}

