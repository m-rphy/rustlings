// strings2.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    // `String` is a mutable, owned, growable heap-allocated string type in Rust
    // `String` -> String Object
    let word = String::from("green"); // Try not changing this line :)
    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}
// `&str` is a borrowed, immutable data type that is often pasted into 
//      function arguements
// `&str` -> string literal (can be converted to a string object with `.to_string()`)
fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}
