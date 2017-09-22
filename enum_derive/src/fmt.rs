/*
Copyright ⓒ 2017 macro-attr contributors.

Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
/**
```ignore
macro_attr! {
    #[derive(EnumDisplay!)]
    enum $name {
        $variant,
        ...
    }
}
```

Derives `Display`, which outputs the name of the variant.  This is equivalent to the behaviour of a derived `Debug` implementation.  This can only be used on an enum comprised on unitary variants.

# Example

```rust
# #[macro_use] extern crate macro_attr;
# #[macro_use] extern crate enum_derive;
macro_attr! {
    #[derive(EnumDisplay!)]
    enum Play { Ball, Store, TheScottish }
}

# fn main() {
assert_eq!(format!("{}", Play::TheScottish), String::from("TheScottish"));
# }
```
*/
#[macro_export]
macro_rules! EnumDisplay {
    (
        @expand $name:ident ()
    ) => {
        enum_derive_util! {
            @as_item
            impl ::std::fmt::Display for $name {
                fn fmt(&self, _: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    loop {} // unreachable
                }
            }
        }
    };

    (
        @expand $name:ident ($($var_names:ident),*)
    ) => {
        enum_derive_util! {
            @as_item
            impl ::std::fmt::Display for $name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    EnumDisplay!(@arms ($name, self, f), ($($var_names)*) -> ())
                }
            }
        }
    };

    (
        @arms ($name:ident, $self_:expr, $f:ident), ($a:ident) -> ($($body:tt)*)
    ) => {
        enum_derive_util! {
            @as_expr
            match *$self_ {
                $($body)*
                $name::$a => write!($f, stringify!($a)),
            }
        }
    };

    (
        @arms ($name:ident, $self_:expr, $f:ident), ($a:ident $b:ident $($rest:tt)*) -> ($($body:tt)*)
    ) => {
        EnumDisplay! {
            @arms ($name, $self_, $f), ($b $($rest)*)
            -> (
                $($body)*
                $name::$a => write!($f, stringify!($a)),
            )
        }
    };

    (() pub enum $name:ident { $($body:tt)* }) => {
        enum_derive_util! {
            @collect_unitary_variants
            (EnumDisplay { @expand $name }),
            ($($body)*,) -> ()
        }
    };

    (() enum $name:ident { $($body:tt)* }) => {
        enum_derive_util! {
            @collect_unitary_variants
            (EnumDisplay { @expand $name }),
            ($($body)*,) -> ()
        }
    };
}
