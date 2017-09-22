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
    #[derive(EnumTag!(fn $fn_name -> #[derive(Debug)] enum $kind_ty))]
    #[derive(EnumTag!(pub fn $fn_name -> enum $kind_ty))]
    enum $name { ... }
}
```

Derives a method that returns a "tag" enumeration value.  The tag enumeration consists of the same variants as `$name`, except with all payloads removed.  Attributes written prior to `$kind_ty` are attached to the generated tag enumeration.

Note that macro attributes are not supported on the tag enumeration.

```rust
# #[macro_use] extern crate macro_attr;
# #[macro_use] extern crate enum_derive;
macro_attr! {
    #[derive(EnumTag!(pub fn tag -> #[derive(Debug, PartialEq)] enum ComestiblesTag))]
    enum Comestibles { Egg(i32), Sausage(i32), Bacon(i32), Spam(i32) }
}

# fn main() {
assert_eq!(Comestibles::Sausage(2).tag(), ComestiblesTag::Sausage);
# }
```
*/
#[macro_export]
macro_rules! EnumTag {
    (
        @expand ($($vis:tt)*), $name:ident, $fn_name:ident,
        {$($attrs:tt)*}, $tag_ty:ident,
        ($($var_names:ident,)*)
    ) => {
        enum_derive_util! {
            @as_item
            impl $name {
                #[inline]
                $($vis)* fn $fn_name(&self) -> $tag_ty {
                    match *self {
                        $(
                            $name::$var_names(..) => $tag_ty::$var_names,
                        )*
                    }
                }
            }

            $($attrs)*
            $($vis)* enum $tag_ty {
                $(
                    $var_names,
                )*
            }
        }
    };

    (
        (pub fn $fn_name:ident -> $(#[$($attrs:tt)*])* enum $tag_ty:ident)
        $(pub)* enum $name:ident { $($body:tt)* }
    ) => {
        enum_derive_util! {
            @collect_variant_names
            (EnumTag { @expand (pub), $name, $fn_name, {$(#[$($attrs)*])*}, $tag_ty, }),
            ($($body)*,) -> ()
        }
    };

    (
        (fn $fn_name:ident -> $(#[$($attrs:tt)*])* enum $tag_ty:ident)
        $(pub)* enum $name:ident { $($body:tt)* }
    ) => {
        enum_derive_util! {
            @collect_variant_names
            (EnumTag { @expand (), $name, $fn_name, {$(#[$($attrs)*])*}, $tag_ty, }),
            ($($body)*,) -> ()
        }
    };
}
