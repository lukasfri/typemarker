#![doc = include_str!("../README.md")]

extern crate proc_macro;
use proc_macro::TokenStream;
use typemarker_core::typemarker_macro;

#[proc_macro_attribute]
pub fn typemarker(attr: TokenStream, item: TokenStream) -> TokenStream {
    typemarker_macro(attr.into(), item.into()).into()
}
