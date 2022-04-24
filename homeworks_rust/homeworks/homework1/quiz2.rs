// quiz2.rs
// This is a quiz for the following sections:
// - Strings

// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    println!("blue");
    println!("red".to_string());
    println!(String::from("hi"));
    println!("rust is fun!".to_owned());
    println!("nice weather".into());
    println!(format!("Interpolation {}", "Station"));
    println!(&String::from("abc")[0..1]);
    println!("  hello there ".trim());
    println!("Happy Monday!".to_string().replace("Mon", "Tues"));
    println!("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
