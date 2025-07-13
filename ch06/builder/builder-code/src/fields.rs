use proc_macro2::TokenStream;
use quote::quote;
use syn::Type;

pub fn original_struct_setters(
    fields: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
) -> impl Iterator<Item = TokenStream> + '_ {
    fields.iter().map(|f| {
        let (field_name, field_type) = get_name_and_type(f);
        let field_name_as_str = field_name.as_ref().unwrap().to_string();

        if matches_type(field_type, "String") {
            quote! {
                #field_name: self.#field_name.as_ref().expect(
                    &format!("Field '{}' is not set", #field_name_as_str)).to_string()
            }
        } else {
            quote! {
                #field_name: self.#field_name.expect(
                    &format!("Field '{}' is not set", #field_name_as_str))
            }
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

fn matches_type(ty: &Type, type_name: &str) -> bool {
    if let Type::Path(p) = ty {
        let first_match = p.path.segments[0].ident.to_string();
        return first_match == *type_name;
    }
    false
}

#[cfg(test)]
mod tests {
    use syn::{Field, Ident, Path, PathSegment, Type, TypePath, punctuated::Punctuated};

    use crate::fields::get_name_and_type;

    #[test]
    fn get_name_and_type_give_back_name() {
        let p = PathSegment {
            ident: Ident::new("String", proc_macro2::Span::call_site()),
            arguments: syn::PathArguments::default(),
        };

        let mut pun = Punctuated::new();
        pun.push(p);
        let ty = Type::Path(TypePath {
            qself: None,
            path: Path {
                leading_colon: None,
                segments: pun,
            },
        });

        let f = Field {
            attrs: vec![],
            vis: syn::Visibility::Inherited,
            mutability: syn::FieldMutability::None,
            ident: Some(Ident::new("example", proc_macro2::Span::call_site())),
            colon_token: None,
            ty,
        };

        let (actual_name, _) = get_name_and_type(&f);
        assert_eq!(actual_name.as_ref().unwrap().to_string(), "example");
    }
}
