use proc_macro2::TokenStream;
use quote::quote;

pub fn original_struct_setters(
    fields: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
) -> impl Iterator<Item = TokenStream> + '_ {
    fields.iter().map(|f| {
        let field_name = &f.ident;
        let field_name_as_str = field_name.as_ref().unwrap().to_string();

        quote! {
            #field_name: self.#field_name.as_ref().expect(
                &format!("Field '{}' is not set", #field_name_as_str)).to_string()
        }
    })
}

pub fn builder_methods(
    fields: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
) -> impl Iterator<Item = TokenStream> + '_ {
    fields.iter().map(|f| {
        let (field_name, field_type) = get_name_and_type(f);
        quote! {
            pub fn #field_name(&mut self, input: #field_type) -> &mut Self {
                self.#field_name = Some(input);
                self
            }
        }
    })
}

pub fn builder_init_values(
    fields: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
) -> impl Iterator<Item = TokenStream> + '_ {
    fields.iter().map(|f| {
        let field_name = &f.ident;
        quote! {
            #field_name: None
        }
    })
}

pub fn builder_field_definitions(
    fields: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
) -> impl Iterator<Item = TokenStream> + '_ {
    fields.iter().map(|f| {
        let (name, f_type) = get_name_and_type(f);
        quote! {
            pub #name: Option<#f_type>
        }
    })
}

fn get_name_and_type(f: &syn::Field) -> (&Option<syn::Ident>, &'_ syn::Type) {
    let field_name = &f.ident;
    let field_type = &f.ty;
    (field_name, field_type)
}
