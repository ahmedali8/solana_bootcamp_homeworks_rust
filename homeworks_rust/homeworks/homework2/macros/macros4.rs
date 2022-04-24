// macros4.rs
// Make me compile! Execute `rustlings hint macros4` for hints :)



#[macro_export]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };

    // It means "for an expression, give it the variable name $val".
    // In macros, variables start with a $. In this macro,
    // if you give it one expression, it will print it.
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
