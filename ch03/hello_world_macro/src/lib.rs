use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Hello)]
pub fn hello(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;

    let add_hello_world = quote! {
        impl #name {
            fn hello_world(&self) {
                println!("Hello, World");
            }
        }
    };
    add_hello_world.into()
}

#[proc_macro_derive(UpperCaseName)]
pub fn uppercase(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;
    let uppercase_name = name.to_string().to_uppercase();

    let add_uppercase = quote! {
        impl #name {
            pub fn testing_testing() {
                println!("One two three")
            }

            fn uppercase(&self) {
                println!("{}", #uppercase_name);
            }
        }

    };
    add_uppercase.into()
}
