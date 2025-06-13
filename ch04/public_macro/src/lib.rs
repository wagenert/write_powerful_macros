extern crate core;
use quote::quote;
use syn::{parse_macro_input, Data::Struct, DataStruct, DeriveInput, Fields::Named, FieldsNamed};

#[proc_macro_attribute]
pub fn public_macro(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;
    let fields = match ast.data {
        Struct(DataStruct{ fields: Named(FieldsNamed {ref named, ..}), ..}) => named,
        _ => unimplemented!("Only named structs are supported"),
    };
    let builder_fields = fields.iter().map(|f| {
        let field_name = &f.ident;
        let field_type = &f.ty;
        quote! {
            pub #field_name: #field_type
        }
    }); 
    let output = quote! {
        pub struct #name {
            #(#builder_fields,)*
        }
    };
    output.into()
}