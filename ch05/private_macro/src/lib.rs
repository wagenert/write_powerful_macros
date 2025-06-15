use proc_macro::TokenStream;
use quote::quote;
use syn::__private::{Span, TokenStream2};
use syn::{DataStruct, DeriveInput, Fields::Named, parse_macro_input};
use syn::{FieldsNamed, Ident};

#[proc_macro]
pub fn private(input: TokenStream) -> TokenStream {
    let item_as_stream: proc_macro2::TokenStream = input.clone().into();
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let methods = generated_methods(&ast);

    quote! {
        #item_as_stream
        impl #name {
            #(#methods)*
        }
    }
    .into()
}

fn generated_methods(ast: &DeriveInput) -> Vec<TokenStream2> {
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

            quote! {
                fn #method_name(&self) -> &#field_type {
                    &self.#field_name
                }
            }
        })
        .collect()
}
