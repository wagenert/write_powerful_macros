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

    let builder_fields = fields.iter().map(|f| {
        let field_name = &f.ident;
        let field_type = &f.ty;
        quote! {
            #field_name: Option<#field_type>
        }
    });

    let builder_inits = fields.iter().map(|f| {
        let field_name = &f.ident;
        quote! {
            #field_name: None
        }
    });

    let builder_methods = fields.iter().map(|f| {
        let field_name = &f.ident;
        let field_type = &f.ty;
        quote! {
            pub fn #field_name(&mut self, input: #field_type) -> &mut Self {
                self.#field_name = Some(input);
                self
            }
        }
    });

    let set_fields = fields.iter().map(|f| {
        let field_name = &f.ident;
        let field_name_as_str = field_name.as_ref().unwrap().to_string();

        quote! {
            #field_name: self.#field_name.as_ref().expect(
                &format!("Field '{}' is not set", #field_name_as_str)).to_string()
        }
    });

    quote! {
        struct #builder {
            #(#builder_fields,)*
        }

        impl #builder {
            #(#builder_methods)*

            pub fn build(&self) -> #name {
                #name {
                    #(#set_fields,)*
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
