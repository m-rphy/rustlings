// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.

use std::mem;

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        // Since my_option is None, calling unwrap() will panic. 
        // It is unnecessary in this case, so the code can be modified as follows:
        // my_option.unwrap();
        println!("None")
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6,
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // The resize() method does not return a new vector. 
    // Instead, it modifies the existing vector in-place. 
    // In the code, resize() is used to attempt resizing the vector to an empty size. 
    // However, the return value is assigned to my_empty_vec, which results in an error. 
    // To create an empty vector, you can use the Vec::new() constructor

    // let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5);
    let my_empty_vec: Vec<i32> = Vec::new(); 
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    // Let's swap these two!
    let mut value_a = 45;
    let mut value_b = 66;
    // One way...
    ////////////////////
    // let temp = value_a
    // Let's swap these two!
    // value_a = value_b;
    // value_b = temp;
    ////////////////////
    // OR
    mem::swap(&mut value_a, &mut value_b);


    println!("value a: {}; value b: {}", value_a, value_b);
}
