use public_macro::public_macro;

#[derive(Debug)]
#[public_macro]
struct Example {
    pub first: i32,
    second: String,
}
fn main() {
   let e = Example {
        first: 42,
        second: "Hello, World!".to_string(),
    };
    println!("Example: first = {}, second = {}", e.first, e.second);
}

