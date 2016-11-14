# `macro-attr`

This crate provides the `macro_attr!` macro that enables the use of custom, macro-based attributes and `derivations`.

**Links**

* [Latest Release](https://crates.io/crates/macro-attr/)
* [Latest Docs](https://docs.rs/crate/macro-attr/)
* [Repository](https://github.com/DanielKeep/rust-custom-derive)

## Compatibility

`macro-attr` is compatible with Rust 1.2 and higher.

## Example

```rust
#[macro_use] extern crate macro_attr;

// Define some traits to be derived.

trait TypeName {
    fn type_name() -> &'static str;
}

trait ReprType {
    type Repr;
}

// Define macros which derive implementations of these macros.

macro_rules! TypeName {
    // We can support any kind of item we want.
    (() $(pub)* enum $name:ident $($tail:tt)*) => { TypeName! { @impl $name } };
    (() $(pub)* struct $name:ident $($tail:tt)*) => { TypeName! { @impl $name } };

    // Inner rule to cut down on repetition.
    (@impl $name:ident) => {
        impl TypeName for $name {
            fn type_name() -> &'static str { stringify!($name) }
        }
    };
}

macro_rules! ReprType {
    // Note that we use a "derivation argument" here for the `$repr` type.
    (($repr:ty) $(pub)* enum $name:ident $($tail:tt)*) => {
        impl ReprType for $name {
            type Repr = $repr;
        }
    };
}

// Here is a macro that *modifies* the item.

macro_rules! rename_to {
    (
        ($new_name:ident),
        then $cb:tt,
        $(#[$($attrs:tt)*])*
        enum $_old_name:ident $($tail:tt)*
    ) => {
        macro_attr_callback! {
            $cb,
            $(#[$($attrs)*])*
            enum $new_name $($tail)*
        }
    };
}

macro_attr! {
    #[allow(dead_code)]
    #[derive(Clone, Copy, Debug, ReprType!(u8), TypeName!)]
    #[rename_to!(Bar)]
    #[repr(u8)]
    enum Foo { A, B }
}

fn main() {
    let bar = Bar::B;
    let v = bar as <Bar as ReprType>::Repr;
    let msg = format!("{}: {:?} ({:?})", Bar::type_name(), bar, v);
    assert_eq!(msg, "Bar: B (1)");
}
```

## License

Licensed under either of

* MIT license (see [LICENSE](LICENSE) or <http://opensource.org/licenses/MIT>)
* Apache License, Version 2.0 (see [LICENSE](LICENSE) or <http://www.apache.org/licenses/LICENSE-2.0>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be dual licensed as above, without any additional terms or conditions.
