// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.

// #[macro_export]标注表示，只要将定义了宏的crate引入作用域
// 宏就应当是可用的。如果没有该标注，这个宏就不能被引入作用域
mod macros {
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
