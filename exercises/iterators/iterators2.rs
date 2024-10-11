// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.


// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
//? The variable 'first' is a 'char'. It needs to be 
//? capitialized and added to the remaining characters in 'c'
//? in order to return the correct 'String'.
//? The remaining characters in 'c' can be viewed as a string slice using the
//? 'as_str' method
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),   // if the input is empty
        Some(first) => {
            first.to_uppercase().collect::<String>() + c.as_str()
        },
    }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // vec![]
    // let mut result: Vec<String> = Vec::new();

    // // iter方法生成一个不可变引用的迭代器
    // // 如果需要一个获取所有权并且返回拥有所有权的迭代器，可以调用into_iter
    // // 而不是iter.类似的，如果希望迭代可变引用，可以调用iter_mut而不是iter
    // let word_iter = words.iter();
    // for word in word_iter {
    //     result.push(capitalize_first(word));
    // }
    // result

    words.iter().map(|x| capitalize_first(x)).collect()
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
//? Collect is very powerful and general
//? Rust just needs to know the desired type
pub fn capitalize_words_string(words: &[&str]) -> String {
    let mut result = String::new();

    let word_iter = words.iter();
    for word in word_iter {
        result.push_str(&capitalize_first(word));
    }
    result

    // words.iter().map(|x| capitalize_first(x)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    // #[ignore = "tmp"]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    // #[ignore = "tmp2"]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
