extern crate proc_macro;
use proc_macro::TokenStream;
use typemarker_core::typestate_macro;

#[proc_macro_attribute]
pub fn typestate(attr: TokenStream, item: TokenStream) -> TokenStream {
    typestate_macro(attr.into(), item.into()).into()
}
