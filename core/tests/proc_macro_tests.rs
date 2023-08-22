use quote::quote;
use syn::{parse2, Item};
use typemarker_core::typemarker_macro;

#[test]
fn traffic_light_full() {
    let attribute_body = quote! {};

    let enum_tokens = quote! {
        enum TrafficLight {
            Red,
            Yellow,
            Green,
        }
    };

    let expected = quote! {
        #[allow(non_snake_case)]
        mod TrafficLight {
            pub enum Red {}
            pub enum Yellow {}
            pub enum Green {}

            #[derive(::core::cmp::Eq, ::core::cmp::PartialEq)]
            pub enum Dynamic {
                Red,
                Yellow,
                Green
            }

            pub trait Trait: __sealed::Sealed {
                fn dynamic() -> Dynamic;
            }
            impl Trait for Red {
                fn dynamic() -> Dynamic {
                    Dynamic::Red
                }
            }
            impl Trait for Yellow {
                fn dynamic() -> Dynamic {
                    Dynamic::Yellow
                }
            }
            impl Trait for Green {
                fn dynamic() -> Dynamic {
                    Dynamic::Green
                }
            }

            mod __sealed {
                pub trait Sealed {}
                impl Sealed for super::Red {}
                impl Sealed for super::Yellow {}
                impl Sealed for super::Green {}
            }
        }
    };

    let result = typemarker_macro(attribute_body, enum_tokens);

    println!("Expected: {}", expected);
    println!("Result: {}", result);

    assert_eq!(
        parse2::<Item>(expected).unwrap(),
        parse2::<Item>(result).unwrap()
    )
}

#[test]
fn traffic_light_full_with_names() {
    let attribute_body = quote! {
        value_name = ValueName, trait_name = TraitName
    };

    let enum_tokens = quote! {
        enum TrafficLight {
            Red,
            Yellow,
            Green,
        }
    };

    let expected = quote! {
        #[allow(non_snake_case)]
        mod TrafficLight {
            pub enum Red {}
            pub enum Yellow {}
            pub enum Green {}

            #[derive(::core::cmp::Eq, ::core::cmp::PartialEq)]
            pub enum ValueName {
                Red,
                Yellow,
                Green
            }

            pub trait TraitName: __sealed::Sealed {
                fn dynamic() -> ValueName;
            }
            impl TraitName for Red {
                fn dynamic() -> ValueName {
                    ValueName::Red
                }
            }
            impl TraitName for Yellow {
                fn dynamic() -> ValueName {
                    ValueName::Yellow
                }
            }
            impl TraitName for Green {
                fn dynamic() -> ValueName {
                    ValueName::Green
                }
            }

            mod __sealed {
                pub trait Sealed {}
                impl Sealed for super::Red {}
                impl Sealed for super::Yellow {}
                impl Sealed for super::Green {}
            }
        }
    };

    let result = typemarker_macro(attribute_body, enum_tokens);

    assert_eq!(
        parse2::<Item>(expected).unwrap(),
        parse2::<Item>(result).unwrap()
    )
}

#[test]
fn traffic_light_no_value() {
    let attribute_body = quote! {
        no_value
    };

    let enum_tokens = quote! {
        enum TrafficLight {
            Red,
            Yellow,
            Green,
        }
    };

    let expected = quote! {
        #[allow(non_snake_case)]
        mod TrafficLight {
            pub enum Red {}
            pub enum Yellow {}
            pub enum Green {}

            pub trait Trait: __sealed::Sealed {}
            impl Trait for Red {}
            impl Trait for Yellow {}
            impl Trait for Green {}

            mod __sealed {
                pub trait Sealed {}
                impl Sealed for super::Red {}
                impl Sealed for super::Yellow {}
                impl Sealed for super::Green {}
            }
        }
    };

    let result = typemarker_macro(attribute_body, enum_tokens);

    assert_eq!(
        parse2::<Item>(expected).unwrap(),
        parse2::<Item>(result).unwrap()
    )
}

#[test]
fn traffic_light_no_trait() {
    let attribute_body = quote! {
        no_trait
    };

    let enum_tokens = quote! {
        enum TrafficLight {
            Red,
            Yellow,
            Green,
        }
    };

    let expected = quote! {
        #[allow(non_snake_case)]
        mod TrafficLight {
            pub enum Red {}
            pub enum Yellow {}
            pub enum Green {}

            #[derive(::core::cmp::Eq, ::core::cmp::PartialEq)]
            pub enum Dynamic {
                Red,
                Yellow,
                Green
            }
        }
    };

    let result = typemarker_macro(attribute_body, enum_tokens);

    assert_eq!(
        parse2::<Item>(expected).unwrap(),
        parse2::<Item>(result).unwrap()
    )
}

#[test]
fn traffic_light_simple() {
    let attribute_body = quote! {
        no_trait, no_value
    };

    let enum_tokens = quote! {
        enum TrafficLight {
            Red,
            Yellow,
            Green,
        }
    };

    let expected = quote! {
        #[allow(non_snake_case)]
        mod TrafficLight {
            pub enum Red {}
            pub enum Yellow {}
            pub enum Green {}
        }
    };

    let result = typemarker_macro(attribute_body, enum_tokens);

    assert_eq!(
        parse2::<Item>(expected).unwrap(),
        parse2::<Item>(result).unwrap()
    )
}
