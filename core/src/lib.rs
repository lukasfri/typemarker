//! Core module of typemarker that includes all the code for the macro and tests.
//!
//! For documentation on how to use typemarker, see the typemarker crate.

use darling::{export::NestedMeta, FromMeta};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned};
use syn::{parse2, spanned::Spanned, ItemEnum};

#[derive(Debug, Default, Eq, PartialEq, FromMeta)]
pub struct Args {
    pub no_value: Option<()>,
    pub value_name: Option<syn::Ident>,

    pub no_trait: Option<()>,
    pub trait_name: Option<syn::Ident>,
}

impl Args {
    pub fn parse(attr: TokenStream) -> Result<Self, TokenStream> {
        let attr_args = NestedMeta::parse_meta_list(attr).map_err(|e| quote_spanned! {
            e.span() =>
                compile_error!(format!("Meta from attribute could not be parsed: {}", e).as_str());
        })?;

        Args::from_list(&attr_args).map_err(|e| {
            quote_spanned! {
                e.span() =>
                compile_error!("Args could not be parsed from meta: {}", e);
            }
        })
    }
}

pub fn typemarker_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = match Args::parse(attr) {
        Ok(val) => val,
        Err(err) => return err,
    };

    let ItemEnum {
        ident,
        vis,
        variants,
        ..
    } = parse2(item)
        .unwrap_or_else(|_| unimplemented!("Typestate can only be created using enums."));
    // // let bodys = parse2::<Item>(tokens)

    if let Some(variant) = variants.iter().find(|variant| !variant.fields.is_empty()) {
        return quote_spanned! {
            variant.fields.span() =>
            compile_error!("All enum variants must be blank/have no fields.")
        };
    }

    let variant_idents: Vec<_> = variants
        .iter()
        .map(|variant| &variant.ident)
        .cloned()
        .collect();

    generate_mod(
        ident,
        vis,
        &variant_idents,
        if args.no_value.is_none() {
            args.value_name.or(Ident::from_string("Dynamic").ok())
        } else {
            None
        },
        if args.no_trait.is_none() {
            args.trait_name.or(Ident::from_string("Trait").ok())
        } else {
            None
        },
    )
}

pub fn generate_mod(
    ident: syn::Ident,
    vis: syn::Visibility,
    variant_idents: &[syn::Ident],
    value: Option<Ident>,
    enum_trait: Option<Ident>,
) -> TokenStream {
    let base_enums = quote! {
      #( pub enum #variant_idents {} )*
    };

    let value_tokens = if let Some(value) = &value {
        quote! {
            #[derive(::core::cmp::Eq, ::core::cmp::PartialEq)]
          pub enum #value {
            #(#variant_idents),*

          }
        }
    } else {
        TokenStream::default()
    };

    let trait_tokens = if let Some(enum_trait) = enum_trait {
        let base_trait_tokens = if let Some(value) = value {
            quote! {
              pub trait #enum_trait: __sealed::Sealed {
                fn dynamic() -> #value;
              }
              #( impl #enum_trait for #variant_idents {
                fn dynamic() -> #value {
                    #value::#variant_idents
                }
              } )*
            }
        } else {
            quote! {
              pub trait #enum_trait: __sealed::Sealed {}
              #( impl #enum_trait for #variant_idents {} )*
            }
        };

        let sealed_trait_tokens = quote! {
            mod __sealed {
                pub trait Sealed {}
                #( impl Sealed for super::#variant_idents {} )*
            }
        };
        quote! {
            #base_trait_tokens

            #sealed_trait_tokens
        }
    } else {
        TokenStream::default()
    };

    quote! {
      #[allow(non_snake_case)]
      #vis mod #ident {
        #base_enums

        #value_tokens

        #trait_tokens
      }
    }
}
