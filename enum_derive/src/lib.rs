/*
Copyright ⓒ 2015-2017 macro-attr contributors.

Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
/*!
This crate provides several macros for deriving some useful methods for unitary enums (*i.e.* enums where variants do not have payloads).

All of these macros are designed to be used with the [`macro-attr`](https://crates.io/crates/macro-attr) crate, though they can be used independent of it.

> **Note**: see also the `TryFrom!` macro provided by the [`conv`](https://crates.io/crates/conv) crate to derive a function for creating enum values from integer values.

# Using Without `macro_attr!`

Although designed to be used with `macro_attr!`, all of the macros in this crate can be used without it.  The following:

```rust
# #[macro_use] extern crate macro_attr;
# #[macro_use] extern crate enum_derive;
macro_attr! {
    #[derive(Copy, Clone, Debug, IterVariants!(Vars))]
    enum ItAintRight { BabeNo, NoNo, BoyBoy }
}
# fn main() {}
```

Can also be written as:

```rust
# #[macro_use] extern crate macro_attr;
# #[macro_use] extern crate enum_derive;
#[derive(Copy, Clone, Debug)]
enum ItAintRight { BabeNo, NoNo, BoyBoy }

IterVariants! { (Vars) enum ItAintRight { BabeNo, NoNo, BoyBoy } }
# fn main() {}
```
*/
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))] extern crate core as std;

use std::fmt;

#[doc(hidden)]
#[macro_export]
macro_rules! enum_derive_util {
    (@as_expr $e:expr) => {$e};
    (@as_item $($i:item)+) => {$($i)+};
    (@first_expr $head:expr, $($tail:expr),*) => {$head};
    (@first_expr $head:expr) => {$head};

    (
        @collect_unitary_variants ($callback:ident { $($args:tt)* }),
        ($(,)*) -> ($($var_names:ident,)*)
    ) => {
        enum_derive_util! {
            @as_item
            $callback!{ $($args)* ($($var_names),*) }
        }
    };

    (
        @collect_unitary_variants $fixed:tt,
        (#[$_attr:meta] $($tail:tt)*) -> ($($var_names:tt)*)
    ) => {
        enum_derive_util! {
            @collect_unitary_variants $fixed,
            ($($tail)*) -> ($($var_names)*)
        }
    };

    (
        @collect_unitary_variants $fixed:tt,
        ($var:ident $(= $_val:expr)*, $($tail:tt)*) -> ($($var_names:tt)*)
    ) => {
        enum_derive_util! {
            @collect_unitary_variants $fixed,
            ($($tail)*) -> ($($var_names)* $var,)
        }
    };

    (
        @collect_unitary_variants ($name:ident),
        ($var:ident $_struct:tt, $($tail:tt)*) -> ($($var_names:tt)*)
    ) => {
        enum_derive_util! {
            @error item
            "enum_derive: cannot parse unitary variants from enum with non-unitary variants."
        }
    };

    (
        @collect_unary_variants ($callback:ident { $($args:tt)* }),
        ($(,)*) -> ($($out:tt)*)
    ) => {
        enum_derive_util! {
            @as_item
            $callback!{ $($args)* ($($out)*) }
        }
    };

    (
        @collect_unary_variants $fixed:tt,
        (#[$_attr:meta] $($tail:tt)*) -> ($($out:tt)*)
    ) => {
        enum_derive_util! {
            @collect_unary_variants $fixed,
            ($($tail)*) -> ($($out)*)
        }
    };

    (
        @collect_unary_variants $fixed:tt,
        ($var_name:ident($var_ty:ty), $($tail:tt)*) -> ($($out:tt)*)
    ) => {
        enum_derive_util! {
            @collect_unary_variants $fixed,
            ($($tail)*) -> ($($out)* $var_name($var_ty),)
        }
    };

    (
        @collect_unary_variants $fixed:tt,
        ($var_name:ident(pub $var_ty:ty), $($tail:tt)*) -> ($($out:tt)*)
    ) => {
        enum_derive_util! {
            @collect_unary_variants $fixed,
            ($($tail)*) -> ($($out)* $var_name($var_ty),)
        }
    };

    (
        @collect_unary_variants ($name:ident),
        ($var:ident $_struct:tt, $($tail:tt)*) -> ($($_out:tt)*)
    ) => {
        enum_derive_util! {
            @error item
            "enum_derive: cannot parse unary variants from enum with non-unary tuple variants."
        }
    };

    (@error item $msg:tt) => {
        const $msg: () = ();
    };
}

/**
```ignore
macro_attr! {
    #[derive(IterVariants!($itername))]
    enum $name {
        $variant,
        ...
    }
}
```

Derives a `$name::iter_variants() -> $itername` method.  The generated `$itername` type implements `Iterator<Item=$name>`, and yields each of the enumeration's variants.  This can only be used on an enum comprised on unitary variants.

# Example

```rust
# #[macro_use] extern crate macro_attr;
# #[macro_use] extern crate enum_derive;
macro_attr! {
    #[derive(IterVariants!(CheeseVariants))]
    #[derive(Debug, PartialEq)] // for assert_eq!
    enum Cheese { RedLeicester, Tilsit, Stilton }
}

# fn main() {
let names: Vec<Cheese> = Cheese::iter_variants().collect();
assert_eq!(names, vec![Cheese::RedLeicester, Cheese::Tilsit, Cheese::Stilton]);
# }
```
*/
#[macro_export]
macro_rules! IterVariants {
    (
        @expand ($($pub_:tt)*) $itername:ident, $name:ident ()
    ) => {
        enum_derive_util! { @as_item $($pub_)* struct $itername; }

        impl ::std::iter::Iterator for $itername {
            type Item = $name;

            #[inline]
            fn next(&mut self) -> ::std::option::Option<Self::Item> {
                None
            }

            #[inline]
            fn size_hint(&self) -> (usize, ::std::option::Option<usize>) {
                (0, Some(0))
            }
        }

        impl ::std::iter::ExactSizeIterator for $itername { }

        enum_derive_util! {
            @as_item
            impl $name {
                #[allow(dead_code)]
                #[inline]
                $($pub_)* fn iter_variants() -> $itername {
                    $itername
                }
            }
        }
    };

    (
        @expand ($($pub_:tt)*) $itername:ident, $name:ident ($($var_names:ident),*)
    ) => {
        enum_derive_util! { @as_item $($pub_)* struct $itername(::std::option::Option<$name>); }

        IterVariants! { @iter ($itername, $name), ($($var_names,)*) -> () () (0usize) }

        enum_derive_util! {
            @as_item
            impl $name {
                #[allow(dead_code)]
                #[inline]
                $($pub_)* fn iter_variants() -> $itername {
                    $itername(::std::option::Option::Some(enum_derive_util!(@first_expr $($name::$var_names),+)))
                }
            }
        }
    };

    (
        @iter ($itername:ident, $name:ident), () -> ($($next_body:tt)*) ($($size_body:tt)*) ($($count:tt)*)
    ) => {
        enum_derive_util! {
            @as_item
            impl ::std::iter::Iterator for $itername {
                type Item = $name;

                #[inline]
                fn next(&mut self) -> ::std::option::Option<Self::Item> {
                    let next_item = match self.0 {
                        $($next_body)*
                        None => None
                    };
                    ::std::mem::replace(&mut self.0, next_item)
                }

                #[inline]
                fn size_hint(&self) -> (usize, ::std::option::Option<usize>) {
                    let variants = $($count)*;
                    let progress = match self.0 {
                        $($size_body)*
                        None => variants
                    };
                    (variants - progress, ::std::option::Option::Some(variants - progress))
                }
            }

            impl ::std::iter::ExactSizeIterator for $itername { }
        }
    };

    (
        @iter ($itername:ident, $name:ident), ($a:ident, $b:ident, $($rest:tt)*) -> ($($next_body:tt)*) ($($size_body:tt)*) ($($count:tt)*)
    ) => {
        IterVariants! {
            @iter ($itername, $name), ($b, $($rest)*)
            -> (
                $($next_body)*
                ::std::option::Option::Some($name::$a) => ::std::option::Option::Some($name::$b),
            )
            (
                $($size_body)*
                ::std::option::Option::Some($name::$a) => $($count)*,
            )
            ($($count)* + 1usize)
        }
    };

    (
        @iter ($itername:ident, $name:ident), ($a:ident,) -> ($($next_body:tt)*) ($($size_body:tt)*) ($($count:tt)*)
    ) => {
        IterVariants! {
            @iter ($itername, $name), ()
            -> (
                $($next_body)*
                ::std::option::Option::Some($name::$a) => ::std::option::Option::None,
            )
            (
                $($size_body)*
                ::std::option::Option::Some($name::$a) => $($count)*,
            )
            ($($count)* + 1usize)
        }
    };

    (($itername:ident) pub enum $name:ident { $($body:tt)* }) => {
        enum_derive_util! {
            @collect_unitary_variants
            (IterVariants { @expand (pub) $itername, $name }),
            ($($body)*,) -> ()
        }
    };

    (($itername:ident) enum $name:ident { $($body:tt)* }) => {
        enum_derive_util! {
            @collect_unitary_variants
            (IterVariants { @expand () $itername, $name }),
            ($($body)*,) -> ()
        }
    };
}

/**
```ignore
macro_attr! {
    #[derive(IterVariantNames!($itername))]
    enum $name {
        $variant,
        ...
    }
}
```

Derives a `$name::iter_variant_names() -> $itername` method.  The generated `$itername` type implements `Iterator<Item=&'static str>`, and yields the name of each of the enumeration's variants.  This can only be used on an enum comprised on unitary variants.

# Example

```rust
# #[macro_use] extern crate macro_attr;
# #[macro_use] extern crate enum_derive;
macro_attr! {
    #[derive(IterVariantNames!(CurrencyVariantNames))]
    enum Currency { Pounds, FrenchFranks, Lira, DeutscheMark }
}

# fn main() {
let names: Vec<&str> = Currency::iter_variant_names().collect();
assert_eq!(names, vec!["Pounds", "FrenchFranks", "Lira", "DeutscheMark"]);
# }
```
*/
#[macro_export]
macro_rules! IterVariantNames {
    (
        @expand ($($pub_:tt)*) $itername:ident, $name:ident ()
    ) => {
        enum_derive_util! { @as_item $($pub_)* struct $itername; }

        impl ::std::iter::Iterator for $itername {
            type Item = &'static str;

            #[inline]
            fn next(&mut self) -> ::std::option::Option<Self::Item> {
                None
            }

            #[inline]
            fn size_hint(&self) -> (usize, ::std::option::Option<usize>) {
                (0, Some(0))
            }
        }

        impl ::std::iter::ExactSizeIterator for $itername { }

        enum_derive_util! {
            @as_item
            impl $name {
                #[allow(dead_code)]
                #[inline]
                $($pub_)* fn iter_variant_names() -> $itername {
                    $itername
                }
            }
        }
    };

    (
        @expand ($($pub_:tt)*) $itername:ident, $name:ident ($($var_names:ident),*)
    ) => {
        enum_derive_util! { @as_item $($pub_)* struct $itername(::std::option::Option<$name>); }

        IterVariantNames! { @iter ($itername, $name), ($($var_names,)*) -> () () (0usize) }

        enum_derive_util! {
            @as_item
            impl $name {
                #[allow(dead_code)]
                #[inline]
                $($pub_)* fn iter_variant_names() -> $itername {
                    $itername(::std::option::Option::Some(enum_derive_util!(@first_expr $($name::$var_names),+)))
                }
            }
        }
    };

    (
        @iter ($itername:ident, $name:ident), () -> ($($next_body:tt)*) ($($size_body:tt)*) ($($count:tt)*)
    ) => {
        enum_derive_util! {
            @as_item
            impl ::std::iter::Iterator for $itername {
                type Item = &'static str;

                #[inline]
                fn next(&mut self) -> ::std::option::Option<Self::Item> {
                    let (next_state, result) = match self.0 {
                        $($next_body)*
                        ::std::option::Option::None => (::std::option::Option::None, ::std::option::Option::None)
                    };
                    self.0 = next_state;
                    result
                }

                #[inline]
                fn size_hint(&self) -> (usize, ::std::option::Option<usize>) {
                    let variants = $($count)*;
                    let progress = match self.0 {
                        $($size_body)*
                        None => variants
                    };
                    (variants - progress, ::std::option::Option::Some(variants - progress))
                }
            }

            impl ::std::iter::ExactSizeIterator for $itername { }
        }
    };

    (
        @iter ($itername:ident, $name:ident), ($a:ident, $b:ident, $($rest:tt)*) -> ($($next_body:tt)*) ($($size_body:tt)*) ($($count:tt)*)
    ) => {
        IterVariantNames! {
            @iter ($itername, $name), ($b, $($rest)*)
            -> (
                $($next_body)*
                ::std::option::Option::Some($name::$a)
                    => (::std::option::Option::Some($name::$b), ::std::option::Option::Some(stringify!($a))),
            )
            (
                $($size_body)*
                ::std::option::Option::Some($name::$a) => $($count)*,
            )
            ($($count)* + 1usize)
        }
    };

    (
        @iter ($itername:ident, $name:ident), ($a:ident,) -> ($($next_body:tt)*) ($($size_body:tt)*) ($($count:tt)*)
    ) => {
        IterVariantNames! {
            @iter ($itername, $name), ()
            -> (
                $($next_body)*
                ::std::option::Option::Some($name::$a)
                    => (::std::option::Option::None, ::std::option::Option::Some(stringify!($a))),
            )
            (
                $($size_body)*
                ::std::option::Option::Some($name::$a) => $($count)*,
            )
            ($($count)* + 1usize)
        }
    };

    (($itername:ident) pub enum $name:ident { $($body:tt)* }) => {
        enum_derive_util! {
            @collect_unitary_variants
            (IterVariantNames { @expand (pub) $itername, $name }),
            ($($body)*,) -> ()
        }
    };

    (($itername:ident) enum $name:ident { $($body:tt)* }) => {
        enum_derive_util! {
            @collect_unitary_variants
            (IterVariantNames { @expand () $itername, $name }),
            ($($body)*,) -> ()
        }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NextVariant!)]
    enum $name {
        $variant,
        ...
    }
}
```

Derives a `$name::next_variant(&self) -> Option<$name>` method, which returns the next variant, or `None` when called on the last.  This can only be used on an enum comprised on unitary variants.

# Example

```rust
# #[macro_use] extern crate macro_attr;
# #[macro_use] extern crate enum_derive;
macro_attr! {
    #[derive(NextVariant!)]
    #[derive(Debug, PartialEq)] // for assert_eq!
    enum Counting { One, Two, Five, ThreeSir, Three }
}

# fn main() {
assert_eq!(Counting::Two.next_variant(), Some(Counting::Five));
# }
```
*/
#[macro_export]
macro_rules! NextVariant {
    (
        @expand ($($pub_:tt)*) $name:ident ()
    ) => {
        enum_derive_util! {
            @as_item
            impl $name {
                #[allow(dead_code)]
                #[inline]
                $($pub_)* fn next_variant(&self) -> ::std::option::Option<$name> {
                    loop {} // unreachable
                }
            }
        }
    };

    (
        @expand ($($pub_:tt)*) $name:ident ($($var_names:ident),*)
    ) => {
        enum_derive_util! {
            @as_item
            impl $name {
                #[allow(dead_code)]
                #[inline]
                $($pub_)* fn next_variant(&self) -> ::std::option::Option<$name> {
                    NextVariant!(@arms ($name, self), ($($var_names)*) -> ())
                }
            }
        }
    };

    (
        @arms ($name:ident, $self_:expr), ($a:ident) -> ($($body:tt)*)
    ) => {
        enum_derive_util! {
            @as_expr
            match *$self_ {
                $($body)*
                $name::$a => ::std::option::Option::None
            }
        }
    };

    (
        @arms ($name:ident, $self_:expr), ($a:ident $b:ident $($rest:tt)*) -> ($($body:tt)*)
    ) => {
        NextVariant! {
            @arms ($name, $self_), ($b $($rest)*)
            -> (
                $($body)*
                $name::$a => ::std::option::Option::Some($name::$b),
            )
        }
    };

    (() pub enum $name:ident { $($body:tt)* }) => {
        enum_derive_util! {
            @collect_unitary_variants
            (NextVariant { @expand (pub) $name }),
            ($($body)*,) -> ()
        }
    };

    (() enum $name:ident { $($body:tt)* }) => {
        enum_derive_util! {
            @collect_unitary_variants
            (NextVariant { @expand () $name }),
            ($($body)*,) -> ()
        }
    };
}

/**
```ignore
macro_attr! {
    #[derive(PrevVariant!)]
    enum $name {
        $variant,
        ...
    }
}
```

Derives a `$name::prev_variant(&self) -> Option<$name>` method, which returns the previous variant, or `None` when called on the last.  This can only be used on an enum comprised on unitary variants.

# Example

```rust
# #[macro_use] extern crate macro_attr;
# #[macro_use] extern crate enum_derive;
macro_attr! {
    #[derive(PrevVariant!)]
    #[derive(Debug, PartialEq)] // for assert_eq!
    enum Sketches { ItsMozart, Italian, WhizzoButter, ItsTheArts }
}

# fn main() {
assert_eq!(Sketches::ItsMozart.prev_variant(), None);
# }
```
*/
#[macro_export]
macro_rules! PrevVariant {
    (
        @expand ($($pub_:tt)*) $name:ident ()
    ) => {
        enum_derive_util! {
            @as_item
            impl $name {
                #[allow(dead_code)]
                #[inline]
                $($pub_)* fn prev_variant(&self) -> ::std::option::Option<$name> {
                    loop {} // unreachable
                }
            }
        }
    };

    (
        @expand ($($pub_:tt)*) $name:ident ($($var_names:ident),*)
    ) => {
        enum_derive_util! {
            @as_item
            impl $name {
                #[allow(dead_code)]
                #[inline]
                $($pub_)* fn prev_variant(&self) -> ::std::option::Option<$name> {
                    PrevVariant!(@arms ($name, self), (::std::option::Option::None, $($var_names)*) -> ())
                }
            }
        }
    };

    (
        @arms ($name:ident, $self_:expr), ($prev:expr, $a:ident) -> ($($body:tt)*)
    ) => {
        enum_derive_util! {
            @as_expr
            match *$self_ {
                $($body)*
                $name::$a => $prev
            }
        }
    };

    (
        @arms ($name:ident, $self_:expr), ($prev:expr, $a:ident $($rest:tt)*) -> ($($body:tt)*)
    ) => {
        PrevVariant! {
            @arms ($name, $self_), (::std::option::Option::Some($name::$a), $($rest)*)
            -> (
                $($body)*
                $name::$a => $prev,
            )
        }
    };

    (() pub enum $name:ident { $($body:tt)* }) => {
        enum_derive_util! {
            @collect_unitary_variants
            (PrevVariant { @expand (pub) $name }),
            ($($body)*,) -> ()
        }
    };

    (() enum $name:ident { $($body:tt)* }) => {
        enum_derive_util! {
            @collect_unitary_variants
            (PrevVariant { @expand () $name }),
            ($($body)*,) -> ()
        }
    };
}

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

/**
```ignore
macro_attr! {
    #[derive(EnumFromInner!)]
    enum $name {
        $variant($payload),
        ...
    }
}
```

Derives `From<$payload>` for each variant.  This requires that all variants be a unary tuple, and each variant's corresponding payload type is not used in any other variant.

# Example

```rust
# #[macro_use] extern crate macro_attr;
# #[macro_use] extern crate enum_derive;
macro_attr! {
    #[derive(EnumFromInner!)]
    #[derive(Debug, PartialEq)] // for assert_eq!
    enum Products { Simpsons(&'static str), Wang(i32), It(f64) }
}

# fn main() {
assert_eq!(Products::from("Just the right length"),
    Products::Simpsons("Just the right length"));
# }
```
*/
#[macro_export]
macro_rules! EnumFromInner {
    (
        @expand $name:ident ($($var_names:ident($var_tys:ty),)*)
    ) => {
        $(
            impl ::std::convert::From<$var_tys> for $name {
                #[inline]
                fn from(v: $var_tys) -> $name {
                    $name::$var_names(v)
                }
            }
        )*
    };

    (() $(pub)* enum $name:ident { $($body:tt)* }) => {
        enum_derive_util! {
            @collect_unary_variants
            (EnumFromInner { @expand $name }),
            ($($body)*,) -> ()
        }
    };
}

/**
```ignore
macro_attr! {
    #[derive(EnumInnerAsTrait!(pub $fn_name -> &$trait))]
    #[derive(EnumInnerAsTrait!(pub $fn_name -> &mut $trait))]
    #[derive(EnumInnerAsTrait!($fn_name -> &$trait))]
    #[derive(EnumInnerAsTrait!($fn_name -> &mut $trait))]
    enum $name {
        $variant($payload),
        ...
    }
}
```

Derives a method to return a borrowed pointer to the payload value, cast to a trait object.  Requires all variants to be unary, and for all variant payloads to implement the trait.

```rust
# #[macro_use] extern crate macro_attr;
# #[macro_use] extern crate enum_derive;
use std::fmt::Display;
use std::ops::AddAssign;
macro_attr! {
    #[derive(EnumInnerAsTrait!(pub as_display -> &Display))]
    #[derive(EnumInnerAsTrait!(pub as_add_assign -> &mut AddAssign<i32>))]
    enum Comestibles { Egg(i32), Sausage(i32), Bacon(i32), Spam(i32) }
}

# fn main() {
let mut breakfast = Comestibles::Spam(109);
breakfast.as_add_assign().add_assign(23);
assert_eq!(format!("{}", breakfast.as_display()), "132");
# }
```
*/
#[macro_export]
macro_rules! EnumInnerAsTrait {
    (
        @expand (pub $fn_name:ident -> &mut $tr:ty), $($tail:tt)*
    ) => {
        EnumInnerAsTrait! { @expand_inner (pub), $fn_name, (mut), $tr, $($tail)* }
    };

    (
        @expand (pub $fn_name:ident -> &$tr:ty), $($tail:tt)*
    ) => {
        EnumInnerAsTrait! { @expand_inner (pub), $fn_name, (), $tr, $($tail)* }
    };

    (
        @expand ($fn_name:ident -> &mut $tr:ty), $($tail:tt)*
    ) => {
        EnumInnerAsTrait! { @expand_inner (), $fn_name, (mut), $tr, $($tail)* }
    };

    (
        @expand ($fn_name:ident -> &$tr:ty), $($tail:tt)*
    ) => {
        EnumInnerAsTrait! { @expand_inner (), $fn_name, (), $tr, $($tail)* }
    };

    (
        @expand_inner
        ($($vis:tt)*), $fn_name:ident, (mut), $tr:ty,
        $ty_name:ident,
        ($($var_names:ident($_var_tys:ty),)*)
    ) => {
        enum_derive_util! {
            @as_item
            impl $ty_name {
                #[inline]
                $($vis)* fn $fn_name(&mut self) -> &mut $tr {
                    match *self {
                        $(
                            $ty_name::$var_names(ref mut v) => v as &mut $tr,
                        )*
                    }
                }
            }
        }
    };

    (
        @expand_inner
        ($($vis:tt)*), $fn_name:ident, (), $tr:ty,
        $ty_name:ident,
        ($($var_names:ident($_var_tys:ty),)*)
    ) => {
        enum_derive_util! {
            @as_item
            impl $ty_name {
                #[inline]
                $($vis)* fn $fn_name(&self) -> &$tr {
                    match *self {
                        $(
                            $ty_name::$var_names(ref v) => v as &$tr,
                        )*
                    }
                }
            }
        }
    };

    ($arg:tt $(pub)* enum $name:ident { $($body:tt)* }) => {
        enum_derive_util! {
            @collect_unary_variants
            (EnumInnerAsTrait { @expand $arg, $name, }),
            ($($body)*,) -> ()
        }
    };
}
