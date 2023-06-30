// functions3.rs
//
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a
// hint.



fn main() {
    // if an parameter is in the funciton definition, an arguement needs to be passed in, even if it is `None`
    call_me(3);
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
