// quiz2.rs
//
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times

// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
//
// No hints this time!



// NOTES 
//  - The `pub` keyword indicates that the function has public visibility, 
//    meaning it can be accessed from outside the module where it is defined. 
//    Other modules or crates can use this function by importing it.



// we have an `use` statement that imports the `transformer` function from the `my_module` module
use crate::my_module::transformer;

// an enum `Command` with three variants: `Uppercase`, `Trim`, and `Append(usize)`.
pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

//  a module called `my_module` using the `mod` keyword. 

mod my_module_a {
    // Inside the my_module we bring the `Command` enum into scope using 
    // the `use super::Command` statement, which allows us to use the `Command`
    // enum within the module.
    use super::Command;

    // 
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {

        // an empty vector `output` is initialized to store the processed strings
        let mut output: Vec<String> = vec![]; // could also say vec!::new();

        //  iterating over the `input` vector using a `for` loop, deconstructing each tuple into `string` and `command`.
        for (string, command) in input.iter() {
            // Within the `match` expression, different actions are performed based on the `command` variant:
            match command {
                Command::Uppercase => {
                    output.push(string.to_uppercase());
                }
                Command::Trim => {
                    output.push(string.trim().to_string());
                }
                Command::Append(count) => {
                    let append_string: String = format!("{}{}", string, "bar".repeat(*count));
                    output.push((*append_string).to_string());
                }
            }
        }
        output
    }
}


// This is code written in the style of John Carmack. 
// It may have more more code, but since it seperate the concerns
// using helper function it is more readable, and modular!

// In addition to the readability, it is also more performant:
// 1) By taking advantage of preallocation! 
// -> with the use of `Vec::with_capacity(input.len());`

// 2) The `uppercase_string` and `trim_string` helper functions 
// receive string references (&str) as arguments to avoid unnecessary string cloning.
// Not entirely avoid unnecessary string cloning, they just don't need to passed
// owned variables 

// The `append_bar_to_string` function constructs the transformed string 
// by directly appending the necessary number of "bar" substrings 
// to the original string. This avoids intermediate allocations and reduces 
// unnecessary memory operations.

// focus on efficiency, clarity, and avoiding unnecessary overhead!

mod my_module {

    use super::Command;

    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut output = Vec::with_capacity(input.len());
    
        for (string, command) in input {
            let transformed_string = match command {
                Command::Uppercase => uppercase_string(&string),
                Command::Trim => trim_string(&string),
                Command::Append(count) => append_bar_to_string(&string, count),
            };
    
            output.push(transformed_string);
        }
    
        output
    }
    
    fn uppercase_string(string: &str) -> String {
        string.to_uppercase()
    }
    
    fn trim_string(string: &str) -> String {
        string.trim().to_string()
    }
    
    fn append_bar_to_string(string: &str, count: usize) -> String {

        // NOTE - It's important to note that String::with_capacity does not actually 
        //        allocate the full memory immediately. It only provides an initial 
        //        capacity hint, and the String object dynamically grows its allocated 
        //        memory as needed during appending.

        // `3 * count` represents the total number of characters needed to append "bar" 
        let mut appended_string = String::with_capacity(string.len() + 3 * count);
    
        appended_string.push_str(string);
    
        for _ in 0..count {
            appended_string.push_str("bar");
        }
    
        appended_string
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use super::*;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
