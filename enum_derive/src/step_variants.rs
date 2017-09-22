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
