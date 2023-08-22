# Typemarker

Procedural macro for easily creating multiple linked marker types, useful for the typestate pattern.

# Examples

By default, typemarker adds both a trait and a dynamic value for the marker enum called "Trait" and "Dynamic" respectively.

```rust
use core::marker::PhantomData;

use typemarker::typemarker;

#[typemarker]
enum LightColor {
    Red,
    Yellow,
    Green,
}

struct TrafficLight<Color: LightColor::Trait>(PhantomData<Color>);
impl TrafficLight<LightColor::Red> {
    fn turn_green(self) -> TrafficLight<LightColor::Green> {
        TrafficLight::<LightColor::Green>(PhantomData)
    }
}

impl<Color: LightColor::Trait> TrafficLight<Color> {
    fn can_go(&self) -> bool {
        Color::dynamic() == LightColor::Dynamic::Green
    }
}

let light = TrafficLight::<LightColor::Red>(PhantomData);
assert!(!light.can_go());

let light = light.turn_green();
assert!(light.can_go());
```

Both the trait and the dynamic value can be disabled using `no_value` and `no_trait` respectively.

```rust
use core::marker::PhantomData;

use typemarker::typemarker;

#[typemarker(no_value, no_trait)]
enum LightColor {
    Red,
    Yellow,
    Green,
}

// Compile error, LightColor::Trait does not exist on no_trait.
// struct TrafficLight<Color: LightColor::Trait>(PhantomData<Color>);

struct TrafficLight<Color>(PhantomData<Color>);
impl TrafficLight<LightColor::Red> {
    fn turn_green(self) -> TrafficLight<LightColor::Green> {
        TrafficLight::<LightColor::Green>(PhantomData)
    }
}

// Compile error, dynamic doesn't exist on no_value.
// impl<Color: LightColor::Trait> TrafficLight<Color> {
//     fn can_go(&self) -> bool {
//         Color::dynamic() == LightColor::Dynamic::Green
//     }
// }

let light = TrafficLight::<LightColor::Red>(PhantomData);
light.turn_green();
```

They can also be renamed using `trait_name = ...` and `value_name = ...`:

```rust
use core::marker::PhantomData;

use typemarker::typemarker;

#[typemarker(trait_name = TraitName, value_name = ValueName)]
enum LightColor {
    Red,
    Yellow,
    Green,
}

struct TrafficLight<Color: LightColor::TraitName>(PhantomData<Color>);
impl TrafficLight<LightColor::Red> {
    fn turn_green(self) -> TrafficLight<LightColor::Green> {
        TrafficLight::<LightColor::Green>(PhantomData)
    }
}

impl<Color: LightColor::TraitName> TrafficLight<Color> {
    fn can_go(&self) -> bool {
        Color::dynamic() == LightColor::ValueName::Green
    }
}

let light = TrafficLight::<LightColor::Red>(PhantomData);
assert!(!light.can_go());

let light = light.turn_green();
assert!(light.can_go());
```
