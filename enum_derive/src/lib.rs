/*!
This crate provides several macros for deriving some useful methods for unitary enums (*i.e.* enums where variants do not have payloads).

All of these macros are designed to be used with the [`custom_derive`](https://crates.io/crates/custom_derive) crate, though they can be used independent of it.

> **Note**: see also the `TryFrom!` macro provided by the [`conv`](https://crates.io/crates/conv) crate to derive a function for creating enum values from integer values.

# Example

Derive iterators that yield all variants of an enum.

```rust
#[macro_use] extern crate custom_derive;
#[macro_use] extern crate enum_derive;

custom_derive! {
    #[derive(Debug, PartialEq, Eq,
        IterVariants(CandyVariants), IterVariantNames(CandyVariantNames))]
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
#[macro_use] extern crate custom_derive;
#[macro_use] extern crate enum_derive;

use Hanagami::*;

custom_derive! {
    #[derive(Debug, PartialEq, Eq, NextVariant, PrevVariant)]
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

- `IterVariants` derives `iter_variants()`, which returns an iterator over the variants of the enum in lexical order.
- `IterVariantNames` derives `iter_variant_names()`, which returns an iterator over the string names of the variants of the enum in lexical order.
- `NextVariant` derives `next_variant(&self)`, which returns the next variant, or `None` when called for the last.
- `PrevVariant` derives `prev_variant(&self)`, which returns the previous variant, or `None` when called for the first.

Both of the `IterVariant*` macros accept a single deriving form.  Taking `IterVariants` as an example, it must be invoked like so:

```rust
# #[macro_use] extern crate custom_derive;
# #[macro_use] extern crate enum_derive;
custom_derive! {
    #[derive(IterVariants(GetVariants))]
    pub enum Get { Up, Down, AllAround }
}
# fn main() {}
```

The argument is the name of the iterator type that will be generated.  Neither macro imposes any naming requirements, save the obvious: the name must not conflict with any other types.

`NextVariant` and `PrevVariant` take no arguments.

The methods and iterator types generated will be public if the enum itself is public; otherwise, they will be private.

## Using Without `custom_derive!`

Although designed to be used with `custom_derive!`, all of the macros in this crate can be used without it.  The following:

```rust
# #[macro_use] extern crate custom_derive;
# #[macro_use] extern crate enum_derive;
custom_derive! {
    #[derive(Copy, Clone, Debug, IterVariants(Vars))]
    enum ItAintRight { BabeNo, NoNo, BoyBoy }
}
# fn main() {}
```

Can also be written as:

```rust
# #[macro_use] extern crate custom_derive;
# #[macro_use] extern crate enum_derive;
#[derive(Copy, Clone, Debug)]
enum ItAintRight { BabeNo, NoNo, BoyBoy }

IterVariants! { (Vars) enum ItAintRight { BabeNo, NoNo, BoyBoy } }
# fn main() {}
```

# Fmt

Fmt implements fmt::Display for your enum.

```rust
#[macro_use] extern crate custom_derive;
#[macro_use] extern crate enum_derive;

use ::std::fmt;

custom_derive! {
    #[derive(Debug, PartialEq, Fmt)]
    pub enum Get { Up, Down, AllAround }
}

# fn main() {
assert_eq!(format!("{}", Get::Up), "Up");
assert_eq!(format!("{}", Get::Down), "Down");
assert_eq!(format!("{}", Get::AllAround), "AllAround");
# }
```

# FromStr

FromStr implements std::str::FromStr for your enum.

```rust
#[macro_use] extern crate custom_derive;
#[macro_use] extern crate enum_derive;

use ::std::str::FromStr;

custom_derive! {
    #[derive(Debug, PartialEq, FromStr)]
    pub enum Get { Up, Down, AllAround }
}

# fn main() {
assert_eq!(Get::from_str("Up").unwrap(), Get::Up);
assert_eq!(Get::from_str("Down").unwrap(), Get::Down);
assert_eq!(Get::from_str("AllAround").unwrap(), Get::AllAround);
# }
```
*/
#[doc(hidden)]
#[macro_export]
macro_rules! enum_derive_util {
    (@as_expr $e:expr) => {$e};
    (@as_item $i:item) => {$i};
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
        const _error: () = "cannot parse unitary variants from enum with non-unitary variants";
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
            fn next(&mut self) -> Option<Self::Item> {
                None
            }
        }

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
        enum_derive_util! { @as_item $($pub_)* struct $itername(Option<$name>); }

        IterVariants! { @iter ($itername, $name), ($($var_names,)*) -> () }

        enum_derive_util! {
            @as_item
            impl $name {
                #[allow(dead_code)]
                $($pub_)* fn iter_variants() -> $itername {
                    $itername(Some(enum_derive_util!(@first_expr $($name::$var_names),+)))
                }
            }
        }
    };

    (
        @iter ($itername:ident, $name:ident), () -> ($($body:tt)*)
    ) => {
        enum_derive_util! {
            @as_item
            impl ::std::iter::Iterator for $itername {
                type Item = $name;
                fn next(&mut self) -> Option<Self::Item> {
                    let next_item = match self.0 {
                        $($body)*
                        None => None
                    };
                    ::std::mem::replace(&mut self.0, next_item)
                }
            }
        }
    };

    (
        @iter ($itername:ident, $name:ident), ($a:ident, $b:ident, $($rest:tt)*) -> ($($body:tt)*)
    ) => {
        IterVariants! {
            @iter ($itername, $name), ($b, $($rest)*)
            -> (
                $($body)*
                Some($name::$a) => Some($name::$b),
            )
        }
    };

    (
        @iter ($itername:ident, $name:ident), ($a:ident,) -> ($($body:tt)*)
    ) => {
        IterVariants! {
            @iter ($itername, $name), ()
            -> (
                $($body)*
                Some($name::$a) => None,
            )
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
            fn next(&mut self) -> Option<Self::Item> {
                None
            }
        }

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
        enum_derive_util! { @as_item $($pub_)* struct $itername(Option<$name>); }

        IterVariantNames! { @iter ($itername, $name), ($($var_names,)*) -> () }

        enum_derive_util! {
            @as_item
            impl $name {
                #[allow(dead_code)]
                $($pub_)* fn iter_variant_names() -> $itername {
                    $itername(Some(enum_derive_util!(@first_expr $($name::$var_names),+)))
                }
            }
        }
    };

    (
        @iter ($itername:ident, $name:ident), () -> ($($body:tt)*)
    ) => {
        enum_derive_util! {
            @as_item
            impl ::std::iter::Iterator for $itername {
                type Item = &'static str;
                fn next(&mut self) -> Option<Self::Item> {
                    let (next_state, result) = match self.0 {
                        $($body)*
                        None => (None, None)
                    };
                    self.0 = next_state;
                    result
                }
            }
        }
    };

    (
        @iter ($itername:ident, $name:ident), ($a:ident, $b:ident, $($rest:tt)*) -> ($($body:tt)*)
    ) => {
        IterVariantNames! {
            @iter ($itername, $name), ($b, $($rest)*)
            -> (
                $($body)*
                Some($name::$a) => (Some($name::$b), Some(stringify!($a))),
            )
        }
    };

    (
        @iter ($itername:ident, $name:ident), ($a:ident,) -> ($($body:tt)*)
    ) => {
        IterVariantNames! {
            @iter ($itername, $name), ()
            -> (
                $($body)*
                Some($name::$a) => (None, Some(stringify!($a))),
            )
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
                $($pub_)* fn next_variant(&self) -> Option<$name> {
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
                $($pub_)* fn next_variant(&self) -> Option<$name> {
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
                $name::$a => None
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
                $name::$a => Some($name::$b),
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
                $($pub_)* fn prev_variant(&self) -> Option<$name> {
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
                $($pub_)* fn prev_variant(&self) -> Option<$name> {
                    PrevVariant!(@arms ($name, self), (None, $($var_names)*) -> ())
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
            @arms ($name, $self_), (Some($name::$a), $($rest)*)
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
macro_rules! Fmt {
    (
        @expand ($($pub_:tt)*) $name:ident ()
    ) => {
        enum_derive_util! {
            @as_item
            impl $name {
                #[allow(dead_code)]
                #[allow(unused_variables)]
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
            impl fmt::Display for $name {
                #[allow(dead_code)]
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    Fmt!(@arms ($name, self, f), ($($var_names)*) -> ())
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
        Fmt! {
            @arms ($name, $self_, $f), ($b $($rest)*)
            -> (
                $($body)*
                $name::$a =>  write!($f, stringify!($a)),
            )
        }
    };

    (() pub enum $name:ident { $($body:tt)* }) => {
        enum_derive_util! {
            @collect_unitary_variants
            (Fmt { @expand (pub) $name }),
            ($($body)*,) -> ()
        }
    };

    (() enum $name:ident { $($body:tt)* }) => {
        enum_derive_util! {
            @collect_unitary_variants
            (Fmt { @expand () $name }),
            ($($body)*,) -> ()
        }
    };
}

#[macro_export]
macro_rules! FromStr {
    (
        @expand ($($pub_:tt)*) $name:ident ()
    ) => {
        enum_derive_util! {
            @as_item
            impl FromStr for $name {
                type Err = ();
                #[allow(dead_code)]
                #[allow(unused_variables)]
                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    Err( () )
                }
            }
        }
    };

    (
        @expand ($($pub_:tt)*) $name:ident ($($var_names:ident),*)
    ) => {
        enum_derive_util! {
            @as_item
            impl FromStr for $name {
                type Err = ();
                #[allow(dead_code)]
                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    FromStr!(@arms ($name, s), ($($var_names)*) -> ())
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
                stringify!($a) => Ok($name::$a),
                _ => Err( () )
            }
        }
    };

    (
        @arms ($name:ident, $s:ident), ($a:ident $b:ident $($rest:tt)*) -> ($($body:tt)*)
    ) => {
        FromStr! {
            @arms ($name, $s), ($b $($rest)*)
            -> (
                $($body)*
                stringify!($a) => Ok($name::$a),
            )
        }
    };

    (() pub enum $name:ident { $($body:tt)* }) => {
        enum_derive_util! {
            @collect_unitary_variants
            (FromStr { @expand (pub) $name }),
            ($($body)*,) -> ()
        }
    };

    (() enum $name:ident { $($body:tt)* }) => {
        enum_derive_util! {
            @collect_unitary_variants
            (FromStr { @expand () $name }),
            ($($body)*,) -> ()
        }
    };
}
