// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.


fn trim_me(input: &str) -> String {
    // Remove whitespace from both ends of a string!

    // NOTE -  the `trim` method returns a borrowed string slice (`&str`), 
    //          not an owned `String` object.
    input.trim().to_string() // the conventional choice

    // This will compile 
    // input.trim().to_owned() // Acheives the same thing, but not common
}

fn compose_me(input: &str) -> String {
    // Add " world!" to the string! There's multiple ways to do this!
    input.to_string() + " world!"
}

fn replace_me(input: &str) -> String {
    // Replace "cars" in the string with "balloons"!

    // NOTE -  the `replace()` method returns a new `String` object directly
    input.replace("cars", "balloons")//.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
