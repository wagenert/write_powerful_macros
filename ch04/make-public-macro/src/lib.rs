extern crate core;

use proc_macro::TokenStream;
use quote::quote;
use syn::{DataStruct, DeriveInput, FieldsNamed, data::Fields, parse_macro_input, token::Struct};

#[proc_macro_attribute]
pub fn public(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;
    let fields = match ast.data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => unimplemented!("only works for stucts with named fields"),
    };
    
    let public_version = quote! {
        pub struct #name {
            pub first: String,
            pub second: u32,
        }
    };
    public_version.into()
}
