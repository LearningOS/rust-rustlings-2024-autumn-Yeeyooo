// vecs2.rs
//
// A Vec of even numbers is given. Your task is to complete the loop so that
// each number in the Vec is multiplied by 2.
//
// Make me pass the test!
//
// Execute `rustlings hint vecs2` or use the `hint` watch subcommand for a hint.

// The variable 'element' represents an item from the Vec as it is
// being iterated.
// In this function, there's a way to directly access the numbers stores in the Vec.
// using the * dereference operator. You can both access and write to the number that way
//? mut in function signature means v is mutable, so we can modify it in the function
fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for element in v.iter_mut() {
        // TODO: Fill this up so that each element in the Vec `v` is
        // multiplied by 2.
        *element *= 2;
    }

    // At this point, `v` should be equal to [4, 8, 12, 16, 20].
    //? return the processed vector v
    v
}

fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    //? iter function returns an immutable iterator over the slice, so we cannot modify it
    //? map() takes a closure and creates an iterator which calls that closure on each element
    //? map() transforms one iterator into another, by means of its argument: something that implements FnMut
    //? Advanced topic: closure
    v.iter().map(|element| {
        // TODO: Do the same thing as above - but instead of mutating the
        // Vec, you can just return the new number!
        element * 2
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        //? (1..) means all integers starting from 1
        //? filter(|x| x % 2 == 0) is a filter, which filters all even numbers
        //? take(5) restricts the filtered result, taking the first 5 elements
        //? collect()将过滤后的迭代器结果收集到一个Vec<i32>向量中，向量的类型是Vec<i32> 
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn test_vec_map() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_map(&v);

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
}
