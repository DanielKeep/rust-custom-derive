/**
```ignore
macro_attr! {
    #[derive(StructNew!(fn $fn_name))]
    #[derive(StructNew!(pub fn $fn_name))]
    #[derive(StructNew!)] // same as `StructNew!(pub fn new)`.
    $struct
}
```

Derives a `new` function for the structure.

# Example

```rust
# #[macro_use] extern crate macro_attr;
# #[macro_use] extern crate struct_derive;
macro_attr! {
    #[derive(StructNew!)]
    #[derive(PartialEq, Debug)] // for `assert_eq!`
    struct Armament { name: &'static str, value: i32 }
}

# fn main() {
let value = Armament::new("tenpenny ones", 10);
assert_eq!(value, Armament { name: "tenpenny ones", value: 10 });
# }
```
*/
#[macro_export]
macro_rules! StructNew {
    (() $($tail:tt)*) => {
        struct_derive_parse_struct! {
            then StructNew!{ @parsed ((pub), new), },
            $($tail)*
        }
    };

    (($(pub)* fn $fn_name:ident) $($tail:tt)*) => {
        struct_derive_parse_struct! {
            then StructNew!{ @parsed ((pub), $fn_name), },
            $($tail)*
        }
    };

    (
        @parsed
        (($($fn_vis:tt)*), $fn_name:ident),
        struct {
            attrs: $attrs:tt,
            vis: $vis:tt,
            name: $name:ident,
            generics: $generics:tt,
            where: $where_:tt,
            kind: unitary,
            fields: $fields:tt,
            num_fields: $num_fields:tt,
        }
    ) => {
        struct_derive_util! {
            @as_item
            impl $name {
                $($fn_vis)* fn $fn_name() -> $name {
                    $name
                }
            }
        }
    };

    (
        @parsed
        (($($fn_vis:tt)*), $fn_name:ident),
        struct {
            attrs: $attrs:tt,
            vis: $vis:tt,
            name: $name:ident,
            generics: $generics:tt,
            where: $where_:tt,
            kind: tuple,
            fields: [
                $(
                    {
                        ord: ($f_ord:tt, $f_ord_ident:ident),
                        attrs: $f_attrs:tt,
                        vis: $f_vis:tt,
                        ty: $f_ty:ty,
                    },
                )*
            ],
            num_fields: $num_fields:tt,
        }
    ) => {
        struct_derive_util! {
            @as_item
            impl $name {
                $($fn_vis)* fn $fn_name($($f_ord_ident: $f_ty,)*) -> $name {
                    $name($($f_ord_ident,)*)
                }
            }
        }
    };

    (
        @parsed
        (($($fn_vis:tt)*), $fn_name:ident),
        struct {
            attrs: $attrs:tt,
            vis: $vis:tt,
            name: $name:ident,
            generics: $generics:tt,
            where: $where_:tt,
            kind: record,
            fields: [
                $(
                    {
                        ord: ($f_ord:tt, $f_ord_ident:ident),
                        attrs: $f_attrs:tt,
                        vis: $f_vis:tt,
                        ty: $f_ty:ty,
                        name: $f_name:ident,
                    },
                )*
            ],
            num_fields: $num_fields:tt,
        }
    ) => {
        struct_derive_util! {
            @as_item
            impl $name {
                $($fn_vis)* fn $fn_name($($f_name: $f_ty,)*) -> $name {
                    $name { $($f_name: $f_name,)* }
                }
            }
        }
    };
}

/**
```ignore
macro_attr! {
    #[derive(StructTypeIndex!)]
    $struct
}
```

Derives an implementation of the `TypeIndex` trait for each field in the structure.  This allows the structure to be used as a simple, fixed-element type map.

# Example

```rust
# #[macro_use] extern crate macro_attr;
# #[macro_use] extern crate struct_derive;
macro_attr! {
    #[derive(StructTypeIndex!)]
    struct Armament { name: &'static str, value: i32 }
}

# fn main() {
use struct_derive::TypeIndex;

let value = Armament { name: "cabbage crates", value: 4 };
assert_eq!(TypeIndex::<&str>::type_index(&value), &"cabbage crates");
assert_eq!(TypeIndex::<i32>::type_index(&value), &4);
# }
```
*/
#[macro_export]
macro_rules! StructTypeIndex {
    (() $($tail:tt)*) => {
        struct_derive_parse_struct! {
            then StructTypeIndex!{ @parsed },
            $($tail)*
        }
    };

    (
        @parsed
        struct {
            attrs: $attrs:tt,
            vis: $vis:tt,
            name: $name:ident,
            generics: $generics:tt,
            where: $where_:tt,
            kind: unitary,
            fields: $fields:tt,
            num_fields: $num_fields:tt,
        }
    ) => {
        // Edge case: nothing to do.
    };

    (
        @parsed
        struct {
            attrs: $attrs:tt,
            vis: $vis:tt,
            name: $name:ident,
            generics: $generics:tt,
            where: $where_:tt,
            kind: tuple,
            fields: [
                $(
                    {
                        ord: ($f_ord:tt, $f_ord_ident:ident),
                        attrs: $f_attrs:tt,
                        vis: $f_vis:tt,
                        ty: $f_ty:ty,
                    },
                )*
            ],
            num_fields: $num_fields:tt,
        }
    ) => {
        $(
            impl $crate::TypeIndex<$f_ty> for $name {
                fn type_index(&self) -> &$f_ty {
                    &self.$f_ord
                }
            }
        )*
    };

    (
        @parsed
        struct {
            attrs: $attrs:tt,
            vis: $vis:tt,
            name: $name:ident,
            generics: $generics:tt,
            where: $where_:tt,
            kind: record,
            fields: [
                $(
                    {
                        ord: ($f_ord:tt, $f_ord_ident:ident),
                        attrs: $f_attrs:tt,
                        vis: $f_vis:tt,
                        ty: $f_ty:ty,
                        name: $f_name:ident,
                    },
                )*
            ],
            num_fields: $num_fields:tt,
        }
    ) => {
        $(
            impl $crate::TypeIndex<$f_ty> for $name {
                fn type_index(&self) -> &$f_ty {
                    &self.$f_name
                }
            }
        )*
    };
}

/**
```ignore
macro_attr! {
    #[derive(StructTypeIndexMut!)]
    $struct
}
```

Derives an implementation of the `TypeIndexMut` trait for each field in the structure.  This allows the structure to be used as a simple, fixed-element type map.

# Example

```rust
# #[macro_use] extern crate macro_attr;
# #[macro_use] extern crate struct_derive;
macro_attr! {
    #[derive(StructTypeIndexMut!)]
    #[derive(Debug, PartialEq)] // for `assert_eq!`
    struct Armament { name: &'static str, value: i32 }
}

# fn main() {
use struct_derive::TypeIndexMut;

let mut value = Armament { name: "cabbage crates", value: 4 };
*value.type_index_mut() = "waspy";
*value.type_index_mut() = 3;

assert_eq!(value, Armament { name: "waspy", value: 3 });
# }
```
*/
#[macro_export]
macro_rules! StructTypeIndexMut {
    (() $($tail:tt)*) => {
        struct_derive_parse_struct! {
            then StructTypeIndexMut!{ @parsed },
            $($tail)*
        }
    };

    (
        @parsed
        struct {
            attrs: $attrs:tt,
            vis: $vis:tt,
            name: $name:ident,
            generics: $generics:tt,
            where: $where_:tt,
            kind: unitary,
            fields: $fields:tt,
            num_fields: $num_fields:tt,
        }
    ) => {
        // Edge case: nothing to do.
    };

    (
        @parsed
        struct {
            attrs: $attrs:tt,
            vis: $vis:tt,
            name: $name:ident,
            generics: $generics:tt,
            where: $where_:tt,
            kind: tuple,
            fields: [
                $(
                    {
                        ord: ($f_ord:tt, $f_ord_ident:ident),
                        attrs: $f_attrs:tt,
                        vis: $f_vis:tt,
                        ty: $f_ty:ty,
                    },
                )*
            ],
            num_fields: $num_fields:tt,
        }
    ) => {
        $(
            impl $crate::TypeIndexMut<$f_ty> for $name {
                fn type_index_mut(&mut self) -> &mut $f_ty {
                    &mut self.$f_ord
                }
            }
        )*
    };

    (
        @parsed
        struct {
            attrs: $attrs:tt,
            vis: $vis:tt,
            name: $name:ident,
            generics: $generics:tt,
            where: $where_:tt,
            kind: record,
            fields: [
                $(
                    {
                        ord: ($f_ord:tt, $f_ord_ident:ident),
                        attrs: $f_attrs:tt,
                        vis: $f_vis:tt,
                        ty: $f_ty:ty,
                        name: $f_name:ident,
                    },
                )*
            ],
            num_fields: $num_fields:tt,
        }
    ) => {
        $(
            impl $crate::TypeIndexMut<$f_ty> for $name {
                fn type_index_mut(&mut self) -> &mut $f_ty {
                    &mut self.$f_name
                }
            }
        )*
    };
}

pub trait TypeIndex<Index> {
    fn type_index(&self) -> &Index;
}

pub trait TypeIndexMut<Index> {
    fn type_index_mut(&mut self) -> &mut Index;
}
