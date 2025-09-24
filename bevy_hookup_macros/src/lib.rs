use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Meta, parse_macro_input};

#[proc_macro_derive(Sendable, attributes(sendable))]
pub fn derive_sendable(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let Data::Enum(data) = data else {
        unimplemented!();
    };

    let field_impls = data
        .variants
        .iter()
        .filter(|variant| {
            variant.attrs.iter().any(|attr| {
                if let Meta::Path(path) = &attr.meta
                    && path.is_ident("sendable")
                {
                    true
                } else {
                    false
                }
            })
        })
        .map(|variant| {
            let variant_name = &variant.ident;
            let Some(first_field) = variant.fields.iter().nth(0) else {
                unimplemented!()
            };

            let field_type = &first_field.ty;

            quote! {
                impl From<&#field_type> for #ident {
                    fn from(value: &#field_type) -> Self {
                        Self::#variant_name(value.clone())
                    }
                }

                impl Into<Option<#field_type>> for #ident {
                    fn into(self) -> Option<#field_type> {
                        match self {
                            Self::#variant_name(value) => Some(value),
                            _ => None,
                        }
                    }
                }
            }
        });

    quote! {
        #(#field_impls)*
    }
    .into()
}
