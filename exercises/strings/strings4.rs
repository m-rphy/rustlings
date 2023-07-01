// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string("red".to_string()); // `.to_string()` -> String
    string(String::from("hi")); // `String::??` -> String 
    string("rust is fun!".to_owned()); // `to_owned() -> String
    string("nice weather".into()); // `.into()` -> String
    string(format!("Interpolation {}", "Station")); // `format!()` -> String
    string("Happy Monday!".to_string().replace("Mon", "Tues")); // `replace()`-> String    
    string("mY sHiFt KeY iS sTiCkY".to_lowercase()); // `.to_lowercase()` -> String

    string_slice("blue"); // String Literal -> String slice
    string_slice(&String::from("abc")[0..1]); // `&String` -> String slice
    string_slice("  hello there ".trim()); // `.trim()` -> String Slice
}
// `.into()` is a really cool method:

// let number: u32 = 42;
// let float: f64 = number.into();

// println!("Float: {}", float);  // Output: Float: 42.0