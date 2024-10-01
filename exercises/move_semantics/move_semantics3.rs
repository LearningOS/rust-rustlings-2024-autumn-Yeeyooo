// move_semantics3.rs
//
// Make me compile without adding new lines-- just changing existing lines! (no
// lines with multiple semicolons necessary!)
//
// Execute `rustlings hint move_semantics3` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let vec0 = Vec::new();           //? vec0 is immutable, initially empty

    let mut vec1 = fill_vec(vec0);  //? vec1 is mutable

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    // println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

//? You can, instead of adding that line back, add 'mut' in one place
//? that will change an existing binding to be a mutable binding instead of
//? an immutable one
fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
