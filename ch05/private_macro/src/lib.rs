use proc_macro::TokenStream;
use quote::quote;
use syn::__private::{Span, TokenStream2};
use syn::token::Token;
use syn::{DataStruct, DeriveInput, Fields::Named, parse_macro_input};
use syn::{FieldsNamed, Ident};

#[proc_macro]
pub fn private(input: TokenStream) -> TokenStream {
    //let item_as_stream: proc_macro2::TokenStream = input.clone().into();
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let (fields, methods) = generated_methods(&ast);

    quote! {
        struct #name {
            #(#fields),*
        }

        impl #name {
            #(#methods)*
        }
    }
    .into()
}

fn generated_methods(ast: &DeriveInput) -> (Vec<TokenStream2>, Vec<TokenStream2>) {
    let named_fields = match &ast.data {
        syn::Data::Struct(DataStruct {
            fields: Named(FieldsNamed { named, .. }),
            ..
        }) => named,

        _ => unimplemented!("Only named field structs are supported"),
    };

    named_fields
        .iter()
        .map(|field| {
            let field_name = &field.ident.as_ref().take().unwrap();
            let field_type = &field.ty;

            let method_name = Ident::new(format!("get_{}", field_name).as_str(), Span::call_site());
            (
                quote! {
                    #field_name: #field_type
                },
                quote! {
                    pub fn #method_name(&self) -> &#field_type {
                        &self.#field_name
                    }
                },
            )
        })
        .collect()
}

#[proc_macro]
pub fn hello_world(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as Ident);
    quote! {

        impl #ast {
            fn hello_world(&self) {
                println!("Hello, world!‚");
            }
        }
    }
    .into()
}
