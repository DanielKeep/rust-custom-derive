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

# Overview

This crate provides macros to derive the following methods for unitary variant enums:

- `IterVariants` derives `iter_variants()`, which returns an iterator over the variants of the enum in lexical order.
- `IterVariantNames` derives `iter_variant_names()`, which returns an iterator over the string names of the variants of the enum in lexical order.

Both of these accept a single deriving form.  Taking `IterVariants` as an example, it must be invoked like so:

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
