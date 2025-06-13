use public_macro::public_macro;
use public_macro::delete;

#[derive(Debug)]
#[public_macro]
struct Example {
    pub first: i32,
    second: String,
}

#[delete]
struct Example2 {}

#[public_macro]
struct UnnamedExample(i32, pub &'static str);

fn main() {
   let e = Example {
        first: 42,
        second: "Hello, World!".to_string(),
    };
    println!("Example: first = {}, second = {}", e.first, e.second);
}

