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
# #[cfg(op_assign)] mod test {
use std::fmt::Display;
use std::ops::AddAssign;
macro_attr! {
    #[derive(EnumInnerAsTrait!(pub as_display -> &Display))]
    #[derive(EnumInnerAsTrait!(pub as_add_assign -> &mut AddAssign<i32>))]
    enum Comestibles { Egg(i32), Sausage(i32), Bacon(i32), Spam(i32) }
}

# pub fn run_test() {
let mut breakfast = Comestibles::Spam(109);
breakfast.as_add_assign().add_assign(23);
assert_eq!(format!("{}", breakfast.as_display()), "132");
# }
# }
# #[cfg(not(op_assign))] mod test { pub fn run_test() {} }
# fn main() { test::run_test() }
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
