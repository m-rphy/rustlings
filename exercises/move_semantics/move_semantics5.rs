// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100; // you need to use the borrowed reference, then move on to the next barrowed reference
    let z = &mut x; // One cannot borrow `x` as mutable more than once at a time
    *z += 1000;
    assert_eq!(x, 1200);
}

// no good
// fn main() {
//     let mut x = 100;
//     let y = &mut x;
//     let z = &mut x; // One cannot borrow `x` as mutable more than once at a time
//     *y += 100;
//     *z += 1000;
//     assert_eq!(x, 1200);
// }
