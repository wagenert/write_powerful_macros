mod fields;

use crate::fields::{
    builder_field_definitions, builder_init_values, builder_methods, original_struct_setters,
};

use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{Data::Struct, DataStruct, DeriveInput, Fields::Named, FieldsNamed};

pub fn create_builder(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse2(item).unwrap();

    let name = &ast.ident;
    let builder = format_ident!("{}Builder", name);

    let fields = match ast.data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => panic!("Only structs with named fields are supported"),
    };

    let builder_fields = builder_field_definitions(fields);

    let builder_inits = builder_init_values(fields);

    let builder_methods = builder_methods(fields);

    let original_struct_set_fields = original_struct_setters(fields);

    quote! {
        struct #builder {
            #(#builder_fields,)*
        }

        impl #builder {
            #(#builder_methods)*

            pub fn build(&self) -> #name {
                #name {
                    #(#original_struct_set_fields,)*
                }
            }
        }

        impl #name {
            pub fn builder() -> #builder {
                #builder {
                    #(#builder_inits,)*
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_strcut_name_should_be_present_in_output() {
        let input = quote! {
            struct StructWithNoFields {}
        };

        let actual = create_builder(input);

        assert!(actual.to_string().contains("StructWithNoFieldsBuilder"));
    }

    #[test]
    fn builder_struct_with_expected_methods_should_be_present_in_output() {
        let input = quote! {
            struct StructWithNoFields {}
        };

        let expected = quote! {
            struct StructWithNoFieldsBuilder {}

            impl StructWithNoFieldsBuilder {
                pub fn build(&self) -> StructWithNoFields {
                    StructWithNoFields {}
                }
            }

            impl StructWithNoFields {
                pub fn builder() -> StructWithNoFieldsBuilder {
                    StructWithNoFieldsBuilder {}
                }
            }
        };

        let actual = create_builder(input);

        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    #[ignore]
    fn assert_with_parsing() {
        let input = quote! {
            struct StructWithNoFields {}
        };

        let actual = create_builder(input);

        let derived: DeriveInput = syn::parse2(actual).unwrap();

        let name = derived.ident;

        assert_eq!(name.to_string(), "StructWithNoFieldsBuilder");
    }
}
