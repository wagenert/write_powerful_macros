#[macro_use]
extern crate hello_world_macro;

#[derive(Hello, UpperCaseName)]
struct Example;

#[derive(Hello)]
enum Pet {
    Cat,
}

fn main() {
    let e = Example {};
    e.hello_world();
    e.uppercase();
    Example::testing_testing();
    let p = Pet::Cat;
    p.hello_world();
}
