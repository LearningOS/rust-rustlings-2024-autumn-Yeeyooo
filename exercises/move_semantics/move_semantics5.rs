// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let mut x = 100;      // x is mutable
    let y = &mut x;  // y is mutable reference of x
    *y += 100;                 // x == y == 200
    let z = &mut x;  // z is mutable reference of x
    *z += 1000;                // x == y == z == 1000
    assert_eq!(x, 1200);
}
