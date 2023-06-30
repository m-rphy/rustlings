// intro2.rs
//
// Make the code print a greeting to the world.
//
// Execute `rustlings hint intro2` or use the `hint` watch subcommand for a
// hint.

// `&str` is a string that can be refrenced
fn main() {
    let world: &str = "World";
    println!("Hello {}!", world);
}
