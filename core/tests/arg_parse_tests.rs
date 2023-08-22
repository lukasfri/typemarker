use quote::quote;
use typemarker_core::Args;

#[test]
fn no_value_arg_parse() {
    let expected = Args {
        no_trait: None,
        no_value: Some(()),
        trait_name: None,
        value_name: None,
    };

    let input = quote! {
      no_value
    };

    let result = Args::parse(input);

    assert_eq!(expected, result);
}

#[test]
fn no_value_no_trait_arg_parse() {
    let expected = Args {
        no_trait: Some(()),
        no_value: Some(()),
        trait_name: None,
        value_name: None,
    };

    let input = quote! {
      no_value, no_trait
    };

    let result = Args::parse(input);

    assert_eq!(expected, result);
}
