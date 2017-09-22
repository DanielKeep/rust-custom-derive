/*
Copyright ⓒ 2017 macro-attr contributors.

Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
use std::fmt;

/**
```ignore
macro_attr! {
    #[derive(EnumFromStr!)]
    enum $name {
        $variant,
        ...
    }
}
```

Derives `FromStr`, allowing `std::parse` to be used.  It checks for an exact match of the variant names.  This can only be used on an enum comprised on unitary variants.

# Example

```rust
# #[macro_use] extern crate macro_attr;
# #[macro_use] extern crate enum_derive;
macro_attr! {
    #[derive(EnumFromStr!)]
    #[derive(Debug, PartialEq)] // for assert_eq!
    enum Uses { DestroyingHouseholdPests, InHospitals, AbsorbingWater }
}

# fn main() {
assert_eq!("InHospitals".parse(), Ok(Uses::InHospitals));
# }
```
*/
#[macro_export]
macro_rules! EnumFromStr {
    (
        @expand ($($pub_:tt)*) $name:ident ()
    ) => {
        enum_derive_util! {
            @as_item
            impl ::std::str::FromStr for $name {
                type Err = $crate::ParseEnumError;

                fn from_str(_: &str) -> ::std::result::Result<Self, Self::Err> {
                    Err($crate::ParseEnumError)
                }
            }
        }
    };

    (
        @expand ($($pub_:tt)*) $name:ident ($($var_names:ident),*)
    ) => {
        enum_derive_util! {
            @as_item
            impl ::std::str::FromStr for $name {
                type Err = $crate::ParseEnumError;

                #[inline]
                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    EnumFromStr!(@arms ($name, s), ($($var_names)*) -> ())
                }
            }
        }
    };

    (
        @arms ($name:ident, $s:ident), ($a:ident) -> ($($body:tt)*)
    ) => {
        enum_derive_util! {
            @as_expr
            match $s {
                $($body)*
                stringify!($a) => ::std::result::Result::Ok($name::$a),
                _ => ::std::result::Result::Err($crate::ParseEnumError)
            }
        }
    };

    (
        @arms ($name:ident, $s:ident), ($a:ident $b:ident $($rest:tt)*) -> ($($body:tt)*)
    ) => {
        EnumFromStr! {
            @arms ($name, $s), ($b $($rest)*)
            -> (
                $($body)*
                stringify!($a) => ::std::result::Result::Ok($name::$a),
            )
        }
    };

    (() pub enum $name:ident { $($body:tt)* }) => {
        enum_derive_util! {
            @collect_unitary_variants
            (EnumFromStr { @expand (pub) $name }),
            ($($body)*,) -> ()
        }
    };

    (() enum $name:ident { $($body:tt)* }) => {
        enum_derive_util! {
            @collect_unitary_variants
            (EnumFromStr { @expand () $name }),
            ($($body)*,) -> ()
        }
    };
}

/**
This is the error type used for derived implementations of `FromStr` for unitary enums.

See the crate documentation for the `EnumFromStr!` macro.
*/
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParseEnumError;

impl fmt::Display for ParseEnumError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "provided string did not match any enum variant")
    }
}

#[cfg(feature = "std")]
impl ::std::error::Error for ParseEnumError {
    fn description(&self) -> &str {
        "provided string did not match any enum variant"
    }
}
