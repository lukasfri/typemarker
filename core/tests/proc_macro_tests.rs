use quote::quote;
use syn::{parse2, Item};
use typemarker_core::typestate_macro;

#[test]
fn traffic_light_full() {
    let attribute_body = quote! {
        #[typestate]
    };

    let enum_tokens = quote! {
        enum TrafficLight {
            Red,
            Yellow,
            Green,
        }
    };

    let expected = quote! {
        mod TrafficLight {
            pub enum Red {}
            pub enum Yellow {}
            pub enum Green {}

            pub enum Dynamic {
                Red,
                Yellow,
                Green
            }

            pub trait Trait: sealed::Sealed {
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

            mod sealed {
                pub trait Sealed {}
                impl Sealed for super::Red {}
                impl Sealed for super::Yellow {}
                impl Sealed for super::Green {}
            }
        }
    };

    let result = typestate_macro(attribute_body, enum_tokens);

    assert_eq!(
        parse2::<Item>(expected).unwrap(),
        parse2::<Item>(result).unwrap()
    )
}

#[test]
fn traffic_light_no_value() {
    let attribute_body = quote! {
        #[typestate(no_value)]
    };

    let enum_tokens = quote! {
        enum TrafficLight {
            Red,
            Yellow,
            Green,
        }
    };

    let expected = quote! {
        mod TrafficLight {
            pub enum Red {}
            pub enum Yellow {}
            pub enum Green {}

            pub trait Trait: sealed::Sealed {}
            impl Trait for Red {}
            impl Trait for Yellow {}
            impl Trait for Green {}

            mod sealed {
                pub trait Sealed {}
                impl Sealed for super::Red {}
                impl Sealed for super::Yellow {}
                impl Sealed for super::Green {}
            }
        }
    };

    let result = typestate_macro(attribute_body, enum_tokens);

    assert_eq!(
        parse2::<Item>(expected).unwrap(),
        parse2::<Item>(result).unwrap()
    )
}

#[test]
fn traffic_light_no_trait() {
    let attribute_body = quote! {
        #[typestate(no_trait)]
    };

    let enum_tokens = quote! {
        enum TrafficLight {
            Red,
            Yellow,
            Green,
        }
    };

    let expected = quote! {
        mod TrafficLight {
            pub enum Red {}
            pub enum Yellow {}
            pub enum Green {}

            pub enum Dynamic {
                Red,
                Yellow,
                Green
            }
        }
    };

    let result = typestate_macro(attribute_body, enum_tokens);

    assert_eq!(
        parse2::<Item>(expected).unwrap(),
        parse2::<Item>(result).unwrap()
    )
}

#[test]
fn traffic_light_simple() {
    let attribute_body = quote! {
        #[typestate(no_trait, no_value)]
    };

    let enum_tokens = quote! {
        enum TrafficLight {
            Red,
            Yellow,
            Green,
        }
    };

    let expected = quote! {
        mod TrafficLight {
            pub enum Red {}
            pub enum Yellow {}
            pub enum Green {}
        }
    };

    let result = typestate_macro(attribute_body, enum_tokens);

    assert_eq!(
        parse2::<Item>(expected).unwrap(),
        parse2::<Item>(result).unwrap()
    )
}
