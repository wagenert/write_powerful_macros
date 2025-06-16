use function_like_compose_macro::compose;
use private_macro::{hello_world, private};

private! {
 pub struct Example {
     pub string_value: String,
     number_value: i32,
 }
}

fn add_one(n: i32) -> i32 {
    n + 1
}

fn stringify(n: i32) -> String {
    n.to_string()
}

fn main() {
    let e = Example {
        string_value: "Hello, world!".to_string(),
        number_value: 42,
    };

    e.get_string_value();
    e.get_number_value();

    let composed = compose!(add_one >> add_one >> stringify);
    println!("{:?}", composed(5));

    hello_world!(Example);
    e.hello_world();
}
