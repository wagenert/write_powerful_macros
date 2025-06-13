extern crate core;
use quote::quote;
use syn::{parse_macro_input, Data::{Struct, Enum}, DataStruct, DeriveInput, Fields::{Named, Unnamed}, FieldsNamed, FieldsUnnamed, DataEnum};

#[proc_macro_attribute]
pub fn public_macro(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;
    let output = match ast.data {
        Struct(DataStruct{ fields: Named(FieldsNamed {ref named, ..}), ..}) => {
            let builder_fields = named.iter().map(|f| {
                let field_name = &f.ident;
                let field_type = &f.ty;
                quote! {
                    pub #field_name: #field_type
                }
                
            });
            quote! {
                pub struct #name {
                    #(#builder_fields,)*
                }
            }
        },
        Struct(DataStruct { fields: Unnamed(FieldsUnnamed { ref unnamed, .. }), .. }) => {
            let builder_fields = unnamed.iter().map(|f| {
                let field_type = &f.ty;
                quote! {
                    pub #field_type
                }
                
            });
            quote! {
                pub struct #name(
                    #(#builder_fields,)*
                );
            }
        },
        Enum(DataEnum { variants, ..}) => {
            let variant_names = variants.iter().map(|v| {
                quote! {
                    #v
                }
            });
            quote! {
                pub enum #name {
                    #(#variant_names),*
                }
            }
        },
        _ => unimplemented!("Only named and unnamed structs are supported"),
    };
    output.into()
}

#[proc_macro_attribute]
pub fn delete(_attr: proc_macro::TokenStream, _item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // This macro is intentionally left empty to demonstrate how to create a macro
    // that does nothing. It can be used to mark items for deletion or ignore them.
    let public_version = quote! {
        // This macro does nothing
    };
    public_version.into()
}