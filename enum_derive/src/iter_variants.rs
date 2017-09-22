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
    #[derive(IterVariants!($itername))]
    enum $name {
        $variant,
        ...
    }
}
```

Derives a `$name::iter_variants() -> $itername` method.  The generated `$itername` type implements `Iterator<Item=$name>`, and yields each of the enumeration's variants.  This can only be used on an enum comprised on unitary variants.

It also derives an implementation of the `IterVariants` trait.

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
use enum_derive::iter_variants::IterVariants;

let names: Vec<Cheese> = Cheese::iter_variants().collect();
assert_eq!(names, vec![Cheese::RedLeicester, Cheese::Tilsit, Cheese::Stilton]);
assert_eq!(<Cheese as IterVariants>::variants().next(), Some(Cheese::RedLeicester));
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

        impl $crate::iter_variants::IterVariants for $name {
            type Iter = $itername;

            #[inline]
            fn variants() -> Self::Iter {
                $name::iter_variants()
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

        impl $crate::iter_variants::IterVariants for $name {
            type Iter = $itername;

            #[inline]
            fn variants() -> Self::Iter {
                $name::iter_variants()
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

It also derives an implementation of the `IterVariantNames` trait.

# Example

```rust
# #[macro_use] extern crate macro_attr;
# #[macro_use] extern crate enum_derive;
macro_attr! {
    #[derive(IterVariantNames!(CurrencyVariantNames))]
    enum Currency { Pounds, FrenchFranks, Lira, DeutscheMark }
}

# fn main() {
use enum_derive::iter_variants::IterVariantNames;

let names: Vec<&str> = Currency::iter_variant_names().collect();
assert_eq!(names, vec!["Pounds", "FrenchFranks", "Lira", "DeutscheMark"]);
assert_eq!(<Currency as IterVariantNames>::variant_names().next(), Some("Pounds"));
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

        impl $crate::iter_variants::IterVariantNames for $name {
            type Iter = $itername;

            #[inline]
            fn variant_names() -> Self::Iter {
                $name::iter_variant_names()
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

        impl $crate::iter_variants::IterVariantNames for $name {
            type Iter = $itername;

            #[inline]
            fn variant_names() -> Self::Iter {
                $name::iter_variant_names()
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

pub trait IterVariants: Sized {
    type Iter: Iterator<Item=Self>;
    fn variants() -> Self::Iter;
}

pub trait IterVariantNames {
    type Iter: Iterator<Item=&'static str>;
    fn variant_names() -> Self::Iter;
}
