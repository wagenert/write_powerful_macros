use public_macro::delete;
use public_macro::public_macro;

#[public_macro]
#[derive(Debug)]
struct Example {
    pub first: i32,
    second: String,
}

#[delete]
struct Example2 {}

#[public_macro]
#[derive(Debug, PartialEq, Clone)]
struct UnnamedExample(i32, pub &'static str);

#[public_macro]
enum ExampleEnum {
    Variant1(i32),
    Variant2(String),
}

fn main() {
    let e = Example {
        first: 42,
        second: "Hello, World!".to_string(),
    };
    println!("Example: first = {}, second = {}", e.first, e.second);
}
