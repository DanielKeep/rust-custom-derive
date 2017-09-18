/*
Copyright ⓒ 2015 macro-attr contributors.

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

# Example

Derive iterators that yield all variants of an enum.

```rust
#[macro_use] extern crate macro_attr;
#[macro_use] extern crate enum_derive;

macro_attr! {
    #[derive(Debug, PartialEq, Eq,
        IterVariants!(CandyVariants), IterVariantNames!(CandyVariantNames))]
    pub enum Candy { Musk, FruitRock, BoPeeps, LemonSherbert }
}

# fn main() {
let vars: CandyVariants = Candy::iter_variants();
let names: CandyVariantNames = Candy::iter_variant_names();
assert_eq!(&*vars.zip(names).collect::<Vec<_>>(), &[
    (Candy::Musk, "Musk"),
    (Candy::FruitRock, "FruitRock"),
    (Candy::BoPeeps, "BoPeeps"),
    (Candy::LemonSherbert, "LemonSherbert"),
]);
# }
```

Alternately, derive `next_variant` and `prev_variant` methods.

```rust
#[macro_use] extern crate macro_attr;
#[macro_use] extern crate enum_derive;

use Hanagami::*;

macro_attr! {
    #[derive(Debug, PartialEq, Eq, NextVariant!, PrevVariant!)]
    pub enum Hanagami { Sakigami, Hasugami, Tsutagami }
}

# fn main() {
assert_eq!(Sakigami.next_variant(), Some(Hasugami));
assert_eq!(Hasugami.next_variant(), Some(Tsutagami));
assert_eq!(Tsutagami.next_variant(), None);

assert_eq!(Sakigami.prev_variant(), None);
assert_eq!(Hasugami.prev_variant(), Some(Sakigami));
assert_eq!(Tsutagami.prev_variant(), Some(Hasugami));
# }
```

# Overview

This crate provides macros to derive the following methods for unitary variant enums:

- `EnumDisplay!` derives `Display`, which outputs the name of the variant.  Note that for unitary variants, this is identical to the behaviour of a derived `Debug` implementation.
- `EnumFromStr!` derives `FromStr`, allowing `str::parse` to be used.  It requires an exact match of the variant name.
- `IterVariants!` derives `iter_variants()`, which returns an iterator over the variants of the enum in lexical order.
- `IterVariantNames!` derives `iter_variant_names()`, which returns an iterator over the string names of the variants of the enum in lexical order.
- `NextVariant!` derives `next_variant(&self)`, which returns the next variant, or `None` when called for the last.
- `PrevVariant!` derives `prev_variant(&self)`, which returns the previous variant, or `None` when called for the first.
- `EnumFromInner!` derives `From<T>` for each variant's payload, assuming all variants are unary.
- `EnumInnerAsTrait!` derives a method to return a borrowed pointer to the inner value, cast to a trait object.

Both of the `IterVariant*!` macros accept a single deriving form.  Taking `IterVariants!` as an example, it must be invoked like so:

```rust
# #[macro_use] extern crate macro_attr;
# #[macro_use] extern crate enum_derive;
macro_attr! {
    #[derive(IterVariants!(GetVariants))]
    pub enum Get { Up, Down, AllAround }
}
# fn main() {}
```

The argument is the name of the iterator type that will be generated.  Neither macro imposes any naming requirements, save the obvious: the name must not conflict with any other types.

`EnumInnerAsTrait!` accepts a single deriving form that specifies the name of the method to be derived, whether the borrow should be mutable, and the trait of interest.  For example:

```rust
# #[macro_use] extern crate macro_attr;
# #[macro_use] extern crate enum_derive;
macro_attr! {
    #[derive(EnumInnerAsTrait!(pub as_display -> &std::fmt::Display))]
    enum Value {
        U32(u32),
        U64(u64),
    }
}

# fn main() {
let s = format!("{}", Value::U64(42).as_display());
assert_eq!(&s[..], "42");
# }
```

The other macros take no arguments.

The methods and iterator types generated will be public if the enum itself is public; otherwise, they will be private.

## Using Without `macro_attr!`

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

## Other Examples

This shows how to use `Display` and `FromStr` to perform string round-tripping of enums.

```rust
#[macro_use] extern crate macro_attr;
#[macro_use] extern crate enum_derive;

macro_attr! {
    #[derive(Debug, PartialEq, EnumDisplay!, EnumFromStr!)]
    pub enum TrollDigit { One, Two, Three, Many, Lots }
}

fn to_troll(mut n: u32) -> String {
    use std::fmt::Write;
    let mut s = String::new();

    if n == 0 {
        panic!("I dun' see nuffin'; how's I s'posed to count it?!");
    }

    while n > 0 {
        let (del, dig) = match n {
            n if n >= 16 => (16, TrollDigit::Lots),
            n if n >= 4 => (4, TrollDigit::Many),
            n if n >= 3 => (3, TrollDigit::Three),
            n if n >= 2 => (2, TrollDigit::Two),
            _ => (1, TrollDigit::One),
        };
        n -= del;
        if s.len() > 0 { s.push_str(" "); }
        write!(&mut s, "{}", dig).unwrap();
    }

    s
}

fn from_troll(s: &str) -> Result<u32, enum_derive::ParseEnumError> {
    let mut n = 0;
    for word in s.split_whitespace() {
        n += match try!(word.parse()) {
            TrollDigit::One => 1,
            TrollDigit::Two => 2,
            TrollDigit::Three => 3,
            TrollDigit::Many => 4,
            TrollDigit::Lots => 16,
        };
    }
    if n == 0 {
        Err(enum_derive::ParseEnumError)
    } else {
        Ok(n)
    }
}

# fn main() {
let number = 42;
let troll_number = to_troll(number);
assert_eq!(troll_number, "Lots Lots Many Many Two");
assert_eq!(from_troll(&troll_number), Ok(number));
# }
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

#[macro_export]
macro_rules! IterVariants {
    (
        @expand ($($pub_:tt)*) $itername:ident, $name:ident ()
    ) => {
        enum_derive_util! { @as_item $($pub_)* struct $itername; }

        impl ::std::iter::Iterator for $itername {
            type Item = $name;
            fn next(&mut self) -> ::std::option::Option<Self::Item> {
                None
            }

            fn size_hint(&self) -> (usize, ::std::option::Option<usize>) {
                (0, Some(0))
            }
        }

        impl ::std::iter::ExactSizeIterator for $itername { }

        enum_derive_util! {
            @as_item
            impl $name {
                #[allow(dead_code)]
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
                fn next(&mut self) -> ::std::option::Option<Self::Item> {
                    let next_item = match self.0 {
                        $($next_body)*
                        None => None
                    };
                    ::std::mem::replace(&mut self.0, next_item)
                }

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

#[macro_export]
macro_rules! IterVariantNames {
    (
        @expand ($($pub_:tt)*) $itername:ident, $name:ident ()
    ) => {
        enum_derive_util! { @as_item $($pub_)* struct $itername; }

        impl ::std::iter::Iterator for $itername {
            type Item = &'static str;
            fn next(&mut self) -> ::std::option::Option<Self::Item> {
                None
            }

            fn size_hint(&self) -> (usize, ::std::option::Option<usize>) {
                (0, Some(0))
            }
        }

        impl ::std::iter::ExactSizeIterator for $itername { }

        enum_derive_util! {
            @as_item
            impl $name {
                #[allow(dead_code)]
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
                fn next(&mut self) -> ::std::option::Option<Self::Item> {
                    let (next_state, result) = match self.0 {
                        $($next_body)*
                        ::std::option::Option::None => (::std::option::Option::None, ::std::option::Option::None)
                    };
                    self.0 = next_state;
                    result
                }

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

#[macro_export]
macro_rules! NextVariant {
    (
        @expand ($($pub_:tt)*) $name:ident ()
    ) => {
        enum_derive_util! {
            @as_item
            impl $name {
                #[allow(dead_code)]
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

#[macro_export]
macro_rules! PrevVariant {
    (
        @expand ($($pub_:tt)*) $name:ident ()
    ) => {
        enum_derive_util! {
            @as_item
            impl $name {
                #[allow(dead_code)]
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

#[macro_export]
macro_rules! EnumFromInner {
    (
        @expand $name:ident ($($var_names:ident($var_tys:ty),)*)
    ) => {
        $(
            impl ::std::convert::From<$var_tys> for $name {
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
