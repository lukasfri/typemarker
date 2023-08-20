use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse::Parser, parse2, Attribute, ItemEnum};

#[derive(Debug, Default, Eq, PartialEq, FromMeta)]
struct Args {
    no_value: Option<()>,
    no_trait: Option<()>,
}

pub fn typestate_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = Attribute::parse_outer.parse2(attr).unwrap().pop().unwrap();

    let args = match attr.meta {
        syn::Meta::Path(_) => Args::default(),
        syn::Meta::List(_) => Args::from_meta(&attr.meta).unwrap(),
        syn::Meta::NameValue(_) => unimplemented!(),
    };

    let ItemEnum {
        ident,
        vis,
        variants,
        ..
    } = parse2(item)
        .unwrap_or_else(|_| unimplemented!("Typestate can only be created using enums."));
    // // let bodys = parse2::<Item>(tokens)

    if variants.iter().any(|variant| !variant.fields.is_empty()) {
        panic!("All enum variants must be blank/have no fields.")
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
        args.no_value.is_none(),
        args.no_trait.is_none(),
    )
}

pub fn generate_mod(
    ident: syn::Ident,
    vis: syn::Visibility,
    variant_idents: &[syn::Ident],
    has_value: bool,
    has_trait: bool,
) -> TokenStream {
    let base_enums = quote! {
      #( pub enum #variant_idents {} )*
    };

    let value_tokens = if has_value {
        quote! {
          pub enum Dynamic {
            #(#variant_idents),*

          }
        }
    } else {
        TokenStream::default()
    };

    let trait_tokens = if has_trait {
        let base_trait_tokens = if has_value {
            quote! {
              pub trait Trait: sealed::Sealed {
                fn dynamic() -> Dynamic;
              }
              #( impl Trait for #variant_idents {
                fn dynamic() -> Dynamic {
                    Dynamic::#variant_idents
                }
              } )*
            }
        } else {
            quote! {
              pub trait Trait: sealed::Sealed {}
              #( impl Trait for #variant_idents {} )*
            }
        };

        let sealed_trait_tokens = quote! {
            mod sealed {
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
      #vis mod #ident {
        #base_enums

        #value_tokens

        #trait_tokens
      }
    }
}
