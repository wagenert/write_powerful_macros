extern crate core;
use quote::{ToTokens, quote};
use syn::{
    Data::{Enum, Struct},
    DataEnum, DataStruct, DeriveInput,
    Fields::{Named, Unnamed},
    FieldsNamed, FieldsUnnamed, Ident, Visibility,
    parse::Parse,
    parse_macro_input,
    punctuated::Punctuated,
    token::Colon,
};

struct StructField {
    name: Ident,
    ty: Ident,
}

impl ToTokens for StructField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let ty = &self.ty;
        quote! {
            pub #name: #ty
        }
        .to_tokens(tokens);
    }
}

impl Parse for StructField {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _vis: Result<Visibility, _> = input.parse();
        let list = Punctuated::<Ident, Colon>::parse_terminated(input).unwrap();
        Ok(StructField {
            name: list.first().unwrap().clone(),
            ty: list.last().unwrap().clone(),
        })
    }
}

#[proc_macro_attribute]
pub fn public_macro(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;
    let attributes = ast.attrs.iter().map(|attr| {
        let meta = attr.meta.clone();
        quote! {
            #[#meta]
        }
    });
    let output = match ast.data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => {
            // Original functional solution
            let builder_fields = named
                .iter()
                .map(|f| syn::parse2::<StructField>(f.to_token_stream()).unwrap());
            quote! {
                #(#attributes)*
                pub struct #name {
                    #(#builder_fields,)*
                }
            }
        }
        Struct(DataStruct {
            fields: Unnamed(FieldsUnnamed { ref unnamed, .. }),
            ..
        }) => {
            let builder_fields = unnamed.iter().map(|f| {
                let field_type = &f.ty;
                quote! {
                    pub #field_type
                }
            });
            quote! {
                #(#attributes)*
                pub struct #name(
                    #(#builder_fields,)*
                );
            }
        }
        Enum(DataEnum { variants, .. }) => {
            let variant_names = variants.iter().map(|v| {
                quote! {
                    #v
                }
            });
            quote! {
                #(#attributes)*
                pub enum #name {
                    #(#variant_names),*
                }
            }
        }
        _ => unimplemented!("Only named and unnamed structs are supported"),
    };
    output.into()
}

#[proc_macro_attribute]
pub fn delete(
    _attr: proc_macro::TokenStream,
    _item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // This macro is intentionally left empty to demonstrate how to create a macro
    // that does nothing. It can be used to mark items for deletion or ignore them.
    let public_version = quote! {
        // This macro does nothing
    };
    public_version.into()
}
