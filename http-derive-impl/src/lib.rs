use darling::FromVariant;
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Fields, Ident, parse_macro_input};

#[proc_macro_derive(HttpStatus, attributes(http))]
pub fn http_status_impl(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);

    let syn::Data::Enum(data) = data else {
        return quote! {
            compile_error!("HttpStatus is only supported on enums");
        }
        .into();
    };

    let mut impl_variants = vec![];
    for variant in data.variants {
        let attrs = match VariantArgs::from_variant(&variant) {
            Ok(val) => val,
            Err(err) => return err.write_errors().into(),
        };

        let variant_ident = &variant.ident;

        match (attrs.status, attrs.transparent) {
            (Some(status), None) => {
                match &variant.fields {
                    Fields::Unit => {
                        impl_variants.push(quote! { #ident::#variant_ident => Self::#status });
                    }
                    Fields::Unnamed(_) => {
                        impl_variants.push(quote! { #ident::#variant_ident(..) => Self::#status })
                    }
                    Fields::Named(_) => {
                        impl_variants
                            .push(quote! { #ident::#variant_ident { .. } => Self::#status });
                    }
                };
            }
            (None, Some(_)) => {
                match &variant.fields {
                    Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                        impl_variants.push(quote! { #ident::#variant_ident(field) => field.into() })
                    }
                    _ => {
                        return quote! {
                            compile_error!("#[http(transparent)] is only supported on variants with one unnamed field.");
                        }
                        .into();
                    }
                };
            }
            (Some(_), Some(_)) => {
                return quote! {
                    compile_error!("Either #[http(status = \"...\")] or #[http(transparent)] has to be used.");
                }
                .into();
            }
            (None, None) => {
                return quote! {
                    compile_error!("One or more variants are missing an #[http(...)] attribute.");
                }
                .into();
            }
        }
    }

    #[allow(unused_mut)]
    let mut http_crates = Vec::<proc_macro2::TokenStream>::new();
    #[cfg(feature = "http-02")]
    http_crates.push(quote! { http_derive::http_02 });

    #[cfg(feature = "http-1")]
    http_crates.push(quote! { http_derive::http_1 });

    let mut impls = vec![];
    for http_crate in http_crates {
        impls.push(quote! {
            impl From<&#ident> for #http_crate::status::StatusCode {
                fn from(value: &#ident) -> Self {
                    match value {
                        #( #impl_variants, )*
                    }
                }
            }
        });
    }

    quote! {
        #( #impls )*
    }
    .into()
}

#[derive(Debug, FromVariant)]
#[darling(attributes(http))]
struct VariantArgs {
    status: Option<Ident>,
    transparent: Option<bool>,
}
