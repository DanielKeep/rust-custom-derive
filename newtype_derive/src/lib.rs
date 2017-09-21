/*
Copyright ⓒ 2015-2017 macro-attr contributors.

Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
/*!
This crate provides several macros for deriving implementations of various traits for "newtype" wrappers (*i.e.* tuple structs with a single element).  That is, given a tuple struct with exactly one field (*e.g.* `struct Buckets(i32)`), these macros will derive "obvious" implementations of traits such as `Add`, `Neg`, `Index`, `Deref`, `From`, etc.

All of these macros are designed to be used with the [`macro-attr`](https://crates.io/crates/macro-attr) crate, though they can be used independent of it.

# Using Without `macro_attr!`

Although designed to be used with `macro_attr!`, all of the macros in this crate can be used without it.  The following:

```rust
# #[macro_use] extern crate macro_attr;
# #[macro_use] extern crate newtype_derive;
macro_attr! {
    #[derive(Copy, Clone, Debug, NewtypeFrom!, NewtypeAdd!, NewtypeAdd!(f32))]
    pub struct Meters(f32);
}
# fn main() {}
```

Can also be written as:

```rust
# #[macro_use] extern crate newtype_derive;
#[derive(Copy, Clone, Debug)]
pub struct Meters(f32);

NewtypeFrom! { () pub struct Meters(f32); }
NewtypeAdd! { () pub struct Meters(f32); }
NewtypeAdd! { (f32) pub struct Meters(f32); }
# fn main() {}
```
*/

/*
# `Newtype$binop` Template

Given `/\/\/\s*(ntbop\s+([A-Za-z0-9]+),\s*([a-z_]+))\n(^#\[.+?\]$\n)*^macro_rules!.*?\{$\n(^ +.*?$\n)*^\}$/`,

```
// \1
#[macro_export]
macro_rules! Newtype\2 {
    ((*) $($tts:tt)*) => {
        Newtype\2! { () $($tts)* }
        Newtype\2! { (&self) $($tts)* }
        Newtype\2! { (&Self) $($tts)* }
        Newtype\2! { (&self, Self) $($tts)* }
    };
    (() $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::\2)::\3, kind: simple, item: $($tts)* }
    };
    ((&self) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::\2)::\3, kind: simple_ref, item: $($tts)* }
    };
    ((&self, $($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::\2)::\3, kind: ref_rhs_rewrap($($rhs)*), item: $($tts)* }
    };
    (($($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::\2)::\3, kind: rhs_rewrap($($rhs)*), item: $($tts)* }
    };
}
```

# `Newtype$binopass` Template

Given `/\/\/\s*(ntbopass\s+([A-Za-z0-9]+),\s*([a-z_]+))\n(^#\[.+?\]$\n)*^macro_rules!.*?\{$\n(^ +.*?$\n)*^\}$/`,

```
// \1
#[macro_export]
#[cfg(op_assign)]
macro_rules! Newtype\2 {
    ((*) $($tts:tt)*) => {
        Newtype\2! { () $($tts)* }
        Newtype\2! { (&self) $($tts)* }
    };
    (() $($tts:tt)*) => {
        newtype_wrap_bin_op_assign! { trait: (::std::ops::\2)::\3, kind: simple, item: $($tts)* }
    };
    (($($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op_assign! { trait: (::std::ops::\2)::\3, kind: rhs($($rhs)*), item: $($tts)* }
    };
}
```

# `Newtype$unop` Template

Given `/\/\/\s*(ntuop\s+([A-Za-z0-9]+),\s*([a-z_]+))\n(^#\[.+?\]$\n)*^macro_rules!.*?\{$\n(^ +.*?$\n)*^\}$/`,

```
// \1
#[macro_export]
macro_rules! Newtype\2 {
    ((*) $($tts:tt)*) => {
        Newtype\2! { () $($tts)* }
        Newtype\2! { (&self) $($tts)* }
    };
    (() $($tts:tt)*) => {
        newtype_wrap_un_op! { trait: (::std::ops::\2)::\3, kind: simple, item: $($tts)* }
    };
    ((&self) $($tts:tt)*) => {
        newtype_wrap_un_op! { trait: (::std::ops::\2)::\3, kind: simple_ref, item: $($tts)* }
    };
}
```
*/
#![cfg_attr(not(feature = "std"), no_std)]

#[doc(hidden)]
#[macro_export]
macro_rules! newtype_as_item {
    ($i:item) => {$i};
}

#[doc(hidden)]
#[macro_export]
macro_rules! newtype_wrap_bin_op {
    (
        trait: ($($tr:tt)*)::$meth:ident,
        kind: simple,
        item: $(pub)* struct $name:ident(pub $t:ty);
    ) => {
        newtype_as_item! {
            impl $($tr)*<$name> for $name {
                type Output = $name;

                #[inline]
                fn $meth(self, rhs: Self) -> $name {
                    $name((self.0).$meth(rhs.0))
                }
            }
        }
    };

    (
        trait: ($($tr:tt)*)::$meth:ident,
        kind: simple,
        item: $(pub)* struct $name:ident($t:ty);
    ) => {
        newtype_as_item! {
            impl $($tr)*<$name> for $name {
                type Output = $name;

                #[inline]
                fn $meth(self, rhs: Self) -> $name {
                    $name((self.0).$meth(rhs.0))
                }
            }
        }
    };

    (
        trait: ($($tr:tt)*)::$meth:ident,
        kind: simple_ref,
        item: $(pub)* struct $name:ident(pub $t:ty);
    ) => {
        newtype_as_item! {
            impl<'a> $($tr)*<&'a $name> for &'a $name {
                type Output = $name;

                #[inline]
                fn $meth(self, rhs: Self) -> $name {
                    $name((self.0).$meth(rhs.0))
                }
            }
        }
    };

    (
        trait: ($($tr:tt)*)::$meth:ident,
        kind: simple_ref,
        item: $(pub)* struct $name:ident($t:ty);
    ) => {
        newtype_as_item! {
            impl<'a> $($tr)*<&'a $name> for &'a $name {
                type Output = $name;

                #[inline]
                fn $meth(self, rhs: Self) -> $name {
                    $name((self.0).$meth(rhs.0))
                }
            }
        }
    };

    (
        trait: ($($tr:tt)*)::$meth:ident,
        kind: rhs_rewrap(&Self),
        item: $(pub)* struct $name:ident(pub $t:ty);
    ) => {
        newtype_as_item! {
            impl<'a> $($tr)*<&'a $name> for $name {
                type Output = $name;

                #[inline]
                fn $meth(self, rhs: &'a $name) -> $name {
                    $name((self.0).$meth(&rhs.0))
                }
            }
        }
    };

    (
        trait: ($($tr:tt)*)::$meth:ident,
        kind: rhs_rewrap(&Self),
        item: $(pub)* struct $name:ident($t:ty);
    ) => {
        newtype_as_item! {
            impl<'a> $($tr)*<&'a $name> for $name {
                type Output = $name;

                #[inline]
                fn $meth(self, rhs: &'a $name) -> $name {
                    $name((self.0).$meth(&rhs.0))
                }
            }
        }
    };

    (
        trait: ($($tr:tt)*)::$meth:ident,
        kind: rhs_rewrap($rhs:ty),
        item: $(pub)* struct $name:ident(pub $t:ty);
    ) => {
        newtype_as_item! {
            impl $($tr)*<$rhs> for $name {
                type Output = $name;

                #[inline]
                fn $meth(self, rhs: $rhs) -> $name {
                    $name((self.0).$meth(rhs))
                }
            }
        }
    };

    (
        trait: ($($tr:tt)*)::$meth:ident,
        kind: rhs_rewrap($rhs:ty),
        item: $(pub)* struct $name:ident($t:ty);
    ) => {
        newtype_as_item! {
            impl $($tr)*<$rhs> for $name {
                type Output = $name;

                #[inline]
                fn $meth(self, rhs: $rhs) -> $name {
                    $name((self.0).$meth(rhs))
                }
            }
        }
    };

    (
        trait: ($($tr:tt)*)::$meth:ident,
        kind: ref_rhs_rewrap(Self),
        item: $(pub)* struct $name:ident(pub $t:ty);
    ) => {
        newtype_as_item! {
            impl<'a> $($tr)*<$name> for &'a $name {
                type Output = $name;

                #[inline]
                fn $meth(self, rhs: $name) -> $name {
                    $name((self.0).$meth(rhs.0))
                }
            }
        }
    };

    (
        trait: ($($tr:tt)*)::$meth:ident,
        kind: ref_rhs_rewrap(Self),
        item: $(pub)* struct $name:ident($t:ty);
    ) => {
        newtype_as_item! {
            impl<'a> $($tr)*<$name> for &'a $name {
                type Output = $name;

                #[inline]
                fn $meth(self, rhs: $name) -> $name {
                    $name((self.0).$meth(rhs.0))
                }
            }
        }
    };

    (
        trait: ($($tr:tt)*)::$meth:ident,
        kind: ref_rhs_rewrap($rhs:ty),
        item: $(pub)* struct $name:ident(pub $t:ty);
    ) => {
        newtype_as_item! {
            impl<'a> $($tr)*<$rhs> for &'a $name {
                type Output = $name;

                #[inline]
                fn $meth(self, rhs: $rhs) -> $name {
                    $name((self.0).$meth(rhs))
                }
            }
        }
    };

    (
        trait: ($($tr:tt)*)::$meth:ident,
        kind: ref_rhs_rewrap($rhs:ty),
        item: $(pub)* struct $name:ident($t:ty);
    ) => {
        newtype_as_item! {
            impl<'a> $($tr)*<$rhs> for &'a $name {
                type Output = $name;

                #[inline]
                fn $meth(self, rhs: $rhs) -> $name {
                    $name((self.0).$meth(rhs))
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! newtype_wrap_bin_op_assign {
    (
        trait: ($($tr:tt)*)::$meth:ident,
        kind: simple,
        item: $(pub)* struct $name:ident(pub $t:ty);
    ) => {
        newtype_as_item! {
            impl $($tr)*<$name> for $name {
                #[inline]
                fn $meth(&mut self, rhs: Self) {
                    (self.0).$meth(rhs.0)
                }
            }
        }
    };

    (
        trait: ($($tr:tt)*)::$meth:ident,
        kind: simple,
        item: $(pub)* struct $name:ident($t:ty);
    ) => {
        newtype_as_item! {
            impl $($tr)*<$name> for $name {
                #[inline]
                fn $meth(&mut self, rhs: Self) {
                    (self.0).$meth(rhs.0)
                }
            }
        }
    };

    (
        trait: ($($tr:tt)*)::$meth:ident,
        kind: rhs(&Self),
        item: $(pub)* struct $name:ident(pub $t:ty);
    ) => {
        newtype_as_item! {
            impl<'a> $($tr)*<&'a $name> for $name {
                #[inline]
                fn $meth(&mut self, rhs: &'a $name) {
                    (self.0).$meth(rhs.0)
                }
            }
        }
    };

    (
        trait: ($($tr:tt)*)::$meth:ident,
        kind: rhs(&Self),
        item: $(pub)* struct $name:ident($t:ty);
    ) => {
        newtype_as_item! {
            impl<'a> $($tr)*<&'a $name> for $name {
                #[inline]
                fn $meth(&mut self, rhs: &'a $name) {
                    (self.0).$meth(rhs.0)
                }
            }
        }
    };

    (
        trait: ($($tr:tt)*)::$meth:ident,
        kind: rhs($rhs:ty),
        item: $(pub)* struct $name:ident(pub $t:ty);
    ) => {
        newtype_as_item! {
            impl $($tr)*<$rhs> for $name {
                #[inline]
                fn $meth(&mut self, rhs: $rhs) {
                    (self.0).$meth(rhs)
                }
            }
        }
    };

    (
        trait: ($($tr:tt)*)::$meth:ident,
        kind: rhs($rhs:ty),
        item: $(pub)* struct $name:ident($t:ty);
    ) => {
        newtype_as_item! {
            impl $($tr)*<$rhs> for $name {
                #[inline]
                fn $meth(&mut self, rhs: $rhs) {
                    (self.0).$meth(rhs)
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! newtype_wrap_un_op {
    (
        trait: ($($tr:tt)*)::$meth:ident,
        kind: simple,
        item: $(pub)* struct $name:ident(pub $t:ty);
    ) => {
        newtype_as_item! {
            impl $($tr)* for $name {
                type Output = $name;

                #[inline]
                fn $meth(self) -> $name {
                    $name((self.0).$meth())
                }
            }
        }
    };

    (
        trait: ($($tr:tt)*)::$meth:ident,
        kind: simple,
        item: $(pub)* struct $name:ident($t:ty);
    ) => {
        newtype_as_item! {
            impl $($tr)* for $name {
                type Output = $name;

                #[inline]
                fn $meth(self) -> $name {
                    $name((self.0).$meth())
                }
            }
        }
    };

    (
        trait: ($($tr:tt)*)::$meth:ident,
        kind: simple_ref,
        item: $(pub)* struct $name:ident(pub $t:ty);
    ) => {
        newtype_as_item! {
            impl<'a> $($tr)* for &'a $name {
                type Output = $name;

                #[inline]
                fn $meth(self) -> $name {
                    $name((self.0).$meth())
                }
            }
        }
    };

    (
        trait: ($($tr:tt)*)::$meth:ident,
        kind: simple_ref,
        item: $(pub)* struct $name:ident($t:ty);
    ) => {
        newtype_as_item! {
            impl<'a> $($tr)* for &'a $name {
                type Output = $name;

                #[inline]
                fn $meth(self) -> $name {
                    $name((self.0).$meth())
                }
            }
        }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeAdd!)]
    #[derive(NewtypeAdd!($rhs_ty))]
    #[derive(NewtypeAdd!(&self))]
    #[derive(NewtypeAdd!(&self, $rhs_ty))]
    struct $name($inner_ty);
}
// or:
macro_attr! {
    #[derive(NewtypeAdd!(*))]
    struct $name($inner_ty);
}
```

Derives an implementation of the `Add` trait by deferring to the implementation of the inner type.  The first four invocation forms shown above correspond to the following implementations:

- `impl Add<$name, Output=$name> for $name`
- `impl Add<$rhs_ty, Output=$name> for $name`
- `impl<'a> Add<&'a $name, Output=$name> for &'a $name`
- `impl<'a> Add<$rhs_ty, Output=$name> for &'a $name`

The last form expands to all combinations of `$name` and `&$name` as the arguments.
*/
// ntbop Add,      add
#[macro_export]
macro_rules! NewtypeAdd {
    ((*) $($tts:tt)*) => {
        NewtypeAdd! { () $($tts)* }
        NewtypeAdd! { (&self) $($tts)* }
        NewtypeAdd! { (&Self) $($tts)* }
        NewtypeAdd! { (&self, Self) $($tts)* }
    };
    (() $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Add)::add, kind: simple, item: $($tts)* }
    };
    ((&self) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Add)::add, kind: simple_ref, item: $($tts)* }
    };
    ((&self, $($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Add)::add, kind: ref_rhs_rewrap($($rhs)*), item: $($tts)* }
    };
    (($($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Add)::add, kind: rhs_rewrap($($rhs)*), item: $($tts)* }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeAddAssign!)]
    #[derive(NewtypeAddAssign!(&Self))]
    #[derive(NewtypeAddAssign!($rhs_ty))]
    struct $name($inner_ty);
}
// or:
macro_attr! {
    #[derive(NewtypeAddAssign!(*))]
    struct $name($inner_ty);
}
```

Derives an implementation of the `AddAssign` trait by deferring to the implementation of the inner type.  The first three invocation forms shown above correspond to the following implementations:

- `impl AddAssign<$name> for $name`
- `impl<'a> AddAssign<&'a $name> for $name`
- `impl AddAssign<$rhs_ty> for $name`

The last form expands to the first two forms.
*/
// ntbopass AddAssign, add_assign
#[macro_export]
#[cfg(op_assign)]
macro_rules! NewtypeAddAssign {
    ((*) $($tts:tt)*) => {
        NewtypeAddAssign! { () $($tts)* }
        NewtypeAddAssign! { (&self) $($tts)* }
    };
    (() $($tts:tt)*) => {
        newtype_wrap_bin_op_assign! { trait: (::std::ops::AddAssign)::add_assign, kind: simple, item: $($tts)* }
    };
    (($($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op_assign! { trait: (::std::ops::AddAssign)::add_assign, kind: rhs($($rhs)*), item: $($tts)* }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeBitAnd!)]
    #[derive(NewtypeBitAnd!($rhs_ty))]
    #[derive(NewtypeBitAnd!(&self))]
    #[derive(NewtypeBitAnd!(&self, $rhs_ty))]
    struct $name($inner_ty);
}
// or:
macro_attr! {
    #[derive(NewtypeBitAnd!(*))]
    struct $name($inner_ty);
}
```

Derives an implementation of the `BitAnd` trait by deferring to the implementation of the inner type.  The first four invocation forms shown above correspond to the following implementations:

- `impl BitAnd<$name, Output=$name> for $name`
- `impl BitAnd<$rhs_ty, Output=$name> for $name`
- `impl<'a> BitAnd<&'a $name, Output=$name> for &'a $name`
- `impl<'a> BitAnd<$rhs_ty, Output=$name> for &'a $name`

The last form expands to all combinations of `$name` and `&$name` as the arguments.
*/
// ntbop BitAnd,   bitand
#[macro_export]
macro_rules! NewtypeBitAnd {
    ((*) $($tts:tt)*) => {
        NewtypeBitAnd! { () $($tts)* }
        NewtypeBitAnd! { (&self) $($tts)* }
        NewtypeBitAnd! { (&Self) $($tts)* }
        NewtypeBitAnd! { (&self, Self) $($tts)* }
    };
    (() $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::BitAnd)::bitand, kind: simple, item: $($tts)* }
    };
    ((&self) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::BitAnd)::bitand, kind: simple_ref, item: $($tts)* }
    };
    ((&self, $($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::BitAnd)::bitand, kind: ref_rhs_rewrap($($rhs)*), item: $($tts)* }
    };
    (($($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::BitAnd)::bitand, kind: rhs_rewrap($($rhs)*), item: $($tts)* }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeBitAndAssign!)]
    #[derive(NewtypeBitAndAssign!(&Self))]
    #[derive(NewtypeBitAndAssign!($rhs_ty))]
    struct $name($inner_ty);
}
// or:
macro_attr! {
    #[derive(NewtypeBitAndAssign!(*))]
    struct $name($inner_ty);
}
```

Derives an implementation of the `BitAndAssign` trait by deferring to the implementation of the inner type.  The first three invocation forms shown above correspond to the following implementations:

- `impl BitAndAssign<$name> for $name`
- `impl<'a> BitAndAssign<&'a $name> for $name`
- `impl BitAndAssign<$rhs_ty> for $name`

The last form expands to the first two forms.
*/
// ntbopass BitAndAssign, bitand_assign
#[macro_export]
#[cfg(op_assign)]
macro_rules! NewtypeBitAndAssign {
    ((*) $($tts:tt)*) => {
        NewtypeBitAndAssign! { () $($tts)* }
        NewtypeBitAndAssign! { (&self) $($tts)* }
    };
    (() $($tts:tt)*) => {
        newtype_wrap_bin_op_assign! { trait: (::std::ops::BitAndAssign)::bitand_assign, kind: simple, item: $($tts)* }
    };
    (($($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op_assign! { trait: (::std::ops::BitAndAssign)::bitand_assign, kind: rhs($($rhs)*), item: $($tts)* }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeBitOr!)]
    #[derive(NewtypeBitOr!($rhs_ty))]
    #[derive(NewtypeBitOr!(&self))]
    #[derive(NewtypeBitOr!(&self, $rhs_ty))]
    struct $name($inner_ty);
}
// or:
macro_attr! {
    #[derive(NewtypeBitOr!(*))]
    struct $name($inner_ty);
}
```

Derives an implementation of the `BitOr` trait by deferring to the implementation of the inner type.  The first four invocation forms shown above correspond to the following implementations:

- `impl BitOr<$name, Output=$name> for $name`
- `impl BitOr<$rhs_ty, Output=$name> for $name`
- `impl<'a> BitOr<&'a $name, Output=$name> for &'a $name`
- `impl<'a> BitOr<$rhs_ty, Output=$name> for &'a $name`

The last form expands to all combinations of `$name` and `&$name` as the arguments.
*/
// ntbop BitOr,    bitor
#[macro_export]
macro_rules! NewtypeBitOr {
    ((*) $($tts:tt)*) => {
        NewtypeBitOr! { () $($tts)* }
        NewtypeBitOr! { (&self) $($tts)* }
        NewtypeBitOr! { (&Self) $($tts)* }
        NewtypeBitOr! { (&self, Self) $($tts)* }
    };
    (() $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::BitOr)::bitor, kind: simple, item: $($tts)* }
    };
    ((&self) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::BitOr)::bitor, kind: simple_ref, item: $($tts)* }
    };
    ((&self, $($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::BitOr)::bitor, kind: ref_rhs_rewrap($($rhs)*), item: $($tts)* }
    };
    (($($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::BitOr)::bitor, kind: rhs_rewrap($($rhs)*), item: $($tts)* }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeBitOrAssign!)]
    #[derive(NewtypeBitOrAssign!(&Self))]
    #[derive(NewtypeBitOrAssign!($rhs_ty))]
    struct $name($inner_ty);
}
// or:
macro_attr! {
    #[derive(NewtypeBitOrAssign!(*))]
    struct $name($inner_ty);
}
```

Derives an implementation of the `BitOrAssign` trait by deferring to the implementation of the inner type.  The first three invocation forms shown above correspond to the following implementations:

- `impl BitOrAssign<$name> for $name`
- `impl<'a> BitOrAssign<&'a $name> for $name`
- `impl BitOrAssign<$rhs_ty> for $name`

The last form expands to the first two forms.
*/
// ntbopass BitOrAssign, bitor_assign
#[macro_export]
#[cfg(op_assign)]
macro_rules! NewtypeBitOrAssign {
    ((*) $($tts:tt)*) => {
        NewtypeBitOrAssign! { () $($tts)* }
        NewtypeBitOrAssign! { (&self) $($tts)* }
    };
    (() $($tts:tt)*) => {
        newtype_wrap_bin_op_assign! { trait: (::std::ops::BitOrAssign)::bitor_assign, kind: simple, item: $($tts)* }
    };
    (($($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op_assign! { trait: (::std::ops::BitOrAssign)::bitor_assign, kind: rhs($($rhs)*), item: $($tts)* }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeBitXor!)]
    #[derive(NewtypeBitXor!($rhs_ty))]
    #[derive(NewtypeBitXor!(&self))]
    #[derive(NewtypeBitXor!(&self, $rhs_ty))]
    struct $name($inner_ty);
}
// or:
macro_attr! {
    #[derive(NewtypeBitXor!(*))]
    struct $name($inner_ty);
}
```

Derives an implementation of the `BitXor` trait by deferring to the implementation of the inner type.  The first four invocation forms shown above correspond to the following implementations:

- `impl BitXor<$name, Output=$name> for $name`
- `impl BitXor<$rhs_ty, Output=$name> for $name`
- `impl<'a> BitXor<&'a $name, Output=$name> for &'a $name`
- `impl<'a> BitXor<$rhs_ty, Output=$name> for &'a $name`

The last form expands to all combinations of `$name` and `&$name` as the arguments.
*/
// ntbop BitXor,   bitxor
#[macro_export]
macro_rules! NewtypeBitXor {
    ((*) $($tts:tt)*) => {
        NewtypeBitXor! { () $($tts)* }
        NewtypeBitXor! { (&self) $($tts)* }
        NewtypeBitXor! { (&Self) $($tts)* }
        NewtypeBitXor! { (&self, Self) $($tts)* }
    };
    (() $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::BitXor)::bitxor, kind: simple, item: $($tts)* }
    };
    ((&self) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::BitXor)::bitxor, kind: simple_ref, item: $($tts)* }
    };
    ((&self, $($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::BitXor)::bitxor, kind: ref_rhs_rewrap($($rhs)*), item: $($tts)* }
    };
    (($($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::BitXor)::bitxor, kind: rhs_rewrap($($rhs)*), item: $($tts)* }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeBitXorAssign!)]
    #[derive(NewtypeBitXorAssign!(&Self))]
    #[derive(NewtypeBitXorAssign!($rhs_ty))]
    struct $name($inner_ty);
}
// or:
macro_attr! {
    #[derive(NewtypeBitXorAssign!(*))]
    struct $name($inner_ty);
}
```

Derives an implementation of the `BitXorAssign` trait by deferring to the implementation of the inner type.  The first three invocation forms shown above correspond to the following implementations:

- `impl BitXorAssign<$name> for $name`
- `impl<'a> BitXorAssign<&'a $name> for $name`
- `impl BitXorAssign<$rhs_ty> for $name`

The last form expands to the first two forms.
*/
// ntbopass BitXorAssign, bitxor_assign
#[macro_export]
#[cfg(op_assign)]
macro_rules! NewtypeBitXorAssign {
    ((*) $($tts:tt)*) => {
        NewtypeBitXorAssign! { () $($tts)* }
        NewtypeBitXorAssign! { (&self) $($tts)* }
    };
    (() $($tts:tt)*) => {
        newtype_wrap_bin_op_assign! { trait: (::std::ops::BitXorAssign)::bitxor_assign, kind: simple, item: $($tts)* }
    };
    (($($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op_assign! { trait: (::std::ops::BitXorAssign)::bitxor_assign, kind: rhs($($rhs)*), item: $($tts)* }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeDiv!)]
    #[derive(NewtypeDiv!($rhs_ty))]
    #[derive(NewtypeDiv!(&self))]
    #[derive(NewtypeDiv!(&self, $rhs_ty))]
    struct $name($inner_ty);
}
// or:
macro_attr! {
    #[derive(NewtypeDiv!(*))]
    struct $name($inner_ty);
}
```

Derives an implementation of the `Div` trait by deferring to the implementation of the inner type.  The first four invocation forms shown above correspond to the following implementations:

- `impl Div<$name, Output=$name> for $name`
- `impl Div<$rhs_ty, Output=$name> for $name`
- `impl<'a> Div<&'a $name, Output=$name> for &'a $name`
- `impl<'a> Div<$rhs_ty, Output=$name> for &'a $name`

The last form expands to all combinations of `$name` and `&$name` as the arguments.
*/
// ntbop Div,      div
#[macro_export]
macro_rules! NewtypeDiv {
    ((*) $($tts:tt)*) => {
        NewtypeDiv! { () $($tts)* }
        NewtypeDiv! { (&self) $($tts)* }
        NewtypeDiv! { (&Self) $($tts)* }
        NewtypeDiv! { (&self, Self) $($tts)* }
    };
    (() $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Div)::div, kind: simple, item: $($tts)* }
    };
    ((&self) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Div)::div, kind: simple_ref, item: $($tts)* }
    };
    ((&self, $($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Div)::div, kind: ref_rhs_rewrap($($rhs)*), item: $($tts)* }
    };
    (($($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Div)::div, kind: rhs_rewrap($($rhs)*), item: $($tts)* }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeDivAssign!)]
    #[derive(NewtypeDivAssign!(&Self))]
    #[derive(NewtypeDivAssign!($rhs_ty))]
    struct $name($inner_ty);
}
// or:
macro_attr! {
    #[derive(NewtypeDivAssign!(*))]
    struct $name($inner_ty);
}
```

Derives an implementation of the `DivAssign` trait by deferring to the implementation of the inner type.  The first three invocation forms shown above correspond to the following implementations:

- `impl DivAssign<$name> for $name`
- `impl<'a> DivAssign<&'a $name> for $name`
- `impl DivAssign<$rhs_ty> for $name`

The last form expands to the first two forms.
*/
// ntbopass DivAssign, div_assign
#[macro_export]
#[cfg(op_assign)]
macro_rules! NewtypeDivAssign {
    ((*) $($tts:tt)*) => {
        NewtypeDivAssign! { () $($tts)* }
        NewtypeDivAssign! { (&self) $($tts)* }
    };
    (() $($tts:tt)*) => {
        newtype_wrap_bin_op_assign! { trait: (::std::ops::DivAssign)::div_assign, kind: simple, item: $($tts)* }
    };
    (($($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op_assign! { trait: (::std::ops::DivAssign)::div_assign, kind: rhs($($rhs)*), item: $($tts)* }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeMul!)]
    #[derive(NewtypeMul!($rhs_ty))]
    #[derive(NewtypeMul!(&self))]
    #[derive(NewtypeMul!(&self, $rhs_ty))]
    struct $name($inner_ty);
}
// or:
macro_attr! {
    #[derive(NewtypeMul!(*))]
    struct $name($inner_ty);
}
```

Derives an implementation of the `Mul` trait by deferring to the implementation of the inner type.  The first four invocation forms shown above correspond to the following implementations:

- `impl Mul<$name, Output=$name> for $name`
- `impl Mul<$rhs_ty, Output=$name> for $name`
- `impl<'a> Mul<&'a $name, Output=$name> for &'a $name`
- `impl<'a> Mul<$rhs_ty, Output=$name> for &'a $name`

The last form expands to all combinations of `$name` and `&$name` as the arguments.
*/
// ntbop Mul,      mul
#[macro_export]
macro_rules! NewtypeMul {
    ((*) $($tts:tt)*) => {
        NewtypeMul! { () $($tts)* }
        NewtypeMul! { (&self) $($tts)* }
        NewtypeMul! { (&Self) $($tts)* }
        NewtypeMul! { (&self, Self) $($tts)* }
    };
    (() $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Mul)::mul, kind: simple, item: $($tts)* }
    };
    ((&self) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Mul)::mul, kind: simple_ref, item: $($tts)* }
    };
    ((&self, $($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Mul)::mul, kind: ref_rhs_rewrap($($rhs)*), item: $($tts)* }
    };
    (($($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Mul)::mul, kind: rhs_rewrap($($rhs)*), item: $($tts)* }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeMulAssign!)]
    #[derive(NewtypeMulAssign!(&Self))]
    #[derive(NewtypeMulAssign!($rhs_ty))]
    struct $name($inner_ty);
}
// or:
macro_attr! {
    #[derive(NewtypeMulAssign!(*))]
    struct $name($inner_ty);
}
```

Derives an implementation of the `MulAssign` trait by deferring to the implementation of the inner type.  The first three invocation forms shown above correspond to the following implementations:

- `impl MulAssign<$name> for $name`
- `impl<'a> MulAssign<&'a $name> for $name`
- `impl MulAssign<$rhs_ty> for $name`

The last form expands to the first two forms.
*/
// ntbopass MulAssign, mul_assign
#[macro_export]
#[cfg(op_assign)]
macro_rules! NewtypeMulAssign {
    ((*) $($tts:tt)*) => {
        NewtypeMulAssign! { () $($tts)* }
        NewtypeMulAssign! { (&self) $($tts)* }
    };
    (() $($tts:tt)*) => {
        newtype_wrap_bin_op_assign! { trait: (::std::ops::MulAssign)::mul_assign, kind: simple, item: $($tts)* }
    };
    (($($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op_assign! { trait: (::std::ops::MulAssign)::mul_assign, kind: rhs($($rhs)*), item: $($tts)* }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeRem!)]
    #[derive(NewtypeRem!($rhs_ty))]
    #[derive(NewtypeRem!(&self))]
    #[derive(NewtypeRem!(&self, $rhs_ty))]
    struct $name($inner_ty);
}
// or:
macro_attr! {
    #[derive(NewtypeRem!(*))]
    struct $name($inner_ty);
}
```

Derives an implementation of the `Rem` trait by deferring to the implementation of the inner type.  The first four invocation forms shown above correspond to the following implementations:

- `impl Rem<$name, Output=$name> for $name`
- `impl Rem<$rhs_ty, Output=$name> for $name`
- `impl<'a> Rem<&'a $name, Output=$name> for &'a $name`
- `impl<'a> Rem<$rhs_ty, Output=$name> for &'a $name`

The last form expands to all combinations of `$name` and `&$name` as the arguments.
*/
// ntbop Rem,      rem
#[macro_export]
macro_rules! NewtypeRem {
    ((*) $($tts:tt)*) => {
        NewtypeRem! { () $($tts)* }
        NewtypeRem! { (&self) $($tts)* }
        NewtypeRem! { (&Self) $($tts)* }
        NewtypeRem! { (&self, Self) $($tts)* }
    };
    (() $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Rem)::rem, kind: simple, item: $($tts)* }
    };
    ((&self) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Rem)::rem, kind: simple_ref, item: $($tts)* }
    };
    ((&self, $($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Rem)::rem, kind: ref_rhs_rewrap($($rhs)*), item: $($tts)* }
    };
    (($($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Rem)::rem, kind: rhs_rewrap($($rhs)*), item: $($tts)* }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeRemAssign!)]
    #[derive(NewtypeRemAssign!(&Self))]
    #[derive(NewtypeRemAssign!($rhs_ty))]
    struct $name($inner_ty);
}
// or:
macro_attr! {
    #[derive(NewtypeRemAssign!(*))]
    struct $name($inner_ty);
}
```

Derives an implementation of the `RemAssign` trait by deferring to the implementation of the inner type.  The first three invocation forms shown above correspond to the following implementations:

- `impl RemAssign<$name> for $name`
- `impl<'a> RemAssign<&'a $name> for $name`
- `impl RemAssign<$rhs_ty> for $name`

The last form expands to the first two forms.
*/
// ntbopass RemAssign, rem_assign
#[macro_export]
#[cfg(op_assign)]
macro_rules! NewtypeRemAssign {
    ((*) $($tts:tt)*) => {
        NewtypeRemAssign! { () $($tts)* }
        NewtypeRemAssign! { (&self) $($tts)* }
    };
    (() $($tts:tt)*) => {
        newtype_wrap_bin_op_assign! { trait: (::std::ops::RemAssign)::rem_assign, kind: simple, item: $($tts)* }
    };
    (($($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op_assign! { trait: (::std::ops::RemAssign)::rem_assign, kind: rhs($($rhs)*), item: $($tts)* }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeSub!)]
    #[derive(NewtypeSub!($rhs_ty))]
    #[derive(NewtypeSub!(&self))]
    #[derive(NewtypeSub!(&self, $rhs_ty))]
    struct $name($inner_ty);
}
// or:
macro_attr! {
    #[derive(NewtypeSub!(*))]
    struct $name($inner_ty);
}
```

Derives an implementation of the `Sub` trait by deferring to the implementation of the inner type.  The first four invocation forms shown above correspond to the following implementations:

- `impl Sub<$name, Output=$name> for $name`
- `impl Sub<$rhs_ty, Output=$name> for $name`
- `impl<'a> Sub<&'a $name, Output=$name> for &'a $name`
- `impl<'a> Sub<$rhs_ty, Output=$name> for &'a $name`

The last form expands to all combinations of `$name` and `&$name` as the arguments.
*/
// ntbop Sub,      sub
#[macro_export]
macro_rules! NewtypeSub {
    ((*) $($tts:tt)*) => {
        NewtypeSub! { () $($tts)* }
        NewtypeSub! { (&self) $($tts)* }
        NewtypeSub! { (&Self) $($tts)* }
        NewtypeSub! { (&self, Self) $($tts)* }
    };
    (() $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Sub)::sub, kind: simple, item: $($tts)* }
    };
    ((&self) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Sub)::sub, kind: simple_ref, item: $($tts)* }
    };
    ((&self, $($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Sub)::sub, kind: ref_rhs_rewrap($($rhs)*), item: $($tts)* }
    };
    (($($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Sub)::sub, kind: rhs_rewrap($($rhs)*), item: $($tts)* }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeSubAssign!)]
    #[derive(NewtypeSubAssign!(&Self))]
    #[derive(NewtypeSubAssign!($rhs_ty))]
    struct $name($inner_ty);
}
// or:
macro_attr! {
    #[derive(NewtypeSubAssign!(*))]
    struct $name($inner_ty);
}
```

Derives an implementation of the `SubAssign` trait by deferring to the implementation of the inner type.  The first three invocation forms shown above correspond to the following implementations:

- `impl SubAssign<$name> for $name`
- `impl<'a> SubAssign<&'a $name> for $name`
- `impl SubAssign<$rhs_ty> for $name`

The last form expands to the first two forms.
*/
// ntbopass SubAssign, sub_assign
#[macro_export]
#[cfg(op_assign)]
macro_rules! NewtypeSubAssign {
    ((*) $($tts:tt)*) => {
        NewtypeSubAssign! { () $($tts)* }
        NewtypeSubAssign! { (&self) $($tts)* }
    };
    (() $($tts:tt)*) => {
        newtype_wrap_bin_op_assign! { trait: (::std::ops::SubAssign)::sub_assign, kind: simple, item: $($tts)* }
    };
    (($($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op_assign! { trait: (::std::ops::SubAssign)::sub_assign, kind: rhs($($rhs)*), item: $($tts)* }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeShl!)]
    #[derive(NewtypeShl!($rhs_ty))]
    #[derive(NewtypeShl!(&self))]
    #[derive(NewtypeShl!(&self, $rhs_ty))]
    struct $name($inner_ty);
}
// or:
macro_attr! {
    #[derive(NewtypeShl!(*))]
    struct $name($inner_ty);
}
```

Derives an implementation of the `Shl` trait by deferring to the implementation of the inner type.  The first four invocation forms shown above correspond to the following implementations:

- `impl Shl<$name, Output=$name> for $name`
- `impl Shl<$rhs_ty, Output=$name> for $name`
- `impl<'a> Shl<&'a $name, Output=$name> for &'a $name`
- `impl<'a> Shl<$rhs_ty, Output=$name> for &'a $name`

The last form expands to all combinations of `$name` and `&$name` as the arguments.
*/
// ntbop Shl,      shl
#[macro_export]
macro_rules! NewtypeShl {
    ((*) $($tts:tt)*) => {
        NewtypeShl! { () $($tts)* }
        NewtypeShl! { (&self) $($tts)* }
        NewtypeShl! { (&Self) $($tts)* }
        NewtypeShl! { (&self, Self) $($tts)* }
    };
    (() $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Shl)::shl, kind: simple, item: $($tts)* }
    };
    ((&self) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Shl)::shl, kind: simple_ref, item: $($tts)* }
    };
    ((&self, $($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Shl)::shl, kind: ref_rhs_rewrap($($rhs)*), item: $($tts)* }
    };
    (($($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Shl)::shl, kind: rhs_rewrap($($rhs)*), item: $($tts)* }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeShlAssign!)]
    #[derive(NewtypeShlAssign!(&Self))]
    #[derive(NewtypeShlAssign!($rhs_ty))]
    struct $name($inner_ty);
}
// or:
macro_attr! {
    #[derive(NewtypeShlAssign!(*))]
    struct $name($inner_ty);
}
```

Derives an implementation of the `ShlAssign` trait by deferring to the implementation of the inner type.  The first three invocation forms shown above correspond to the following implementations:

- `impl ShlAssign<$name> for $name`
- `impl<'a> ShlAssign<&'a $name> for $name`
- `impl ShlAssign<$rhs_ty> for $name`

The last form expands to the first two forms.
*/
// ntbopass ShlAssign, shl_assign
#[macro_export]
#[cfg(op_assign)]
macro_rules! NewtypeShlAssign {
    ((*) $($tts:tt)*) => {
        NewtypeShlAssign! { () $($tts)* }
        NewtypeShlAssign! { (&self) $($tts)* }
    };
    (() $($tts:tt)*) => {
        newtype_wrap_bin_op_assign! { trait: (::std::ops::ShlAssign)::shl_assign, kind: simple, item: $($tts)* }
    };
    (($($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op_assign! { trait: (::std::ops::ShlAssign)::shl_assign, kind: rhs($($rhs)*), item: $($tts)* }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeShr!)]
    #[derive(NewtypeShr!($rhs_ty))]
    #[derive(NewtypeShr!(&self))]
    #[derive(NewtypeShr!(&self, $rhs_ty))]
    struct $name($inner_ty);
}
// or:
macro_attr! {
    #[derive(NewtypeShr!(*))]
    struct $name($inner_ty);
}
```

Derives an implementation of the `Shr` trait by deferring to the implementation of the inner type.  The first four invocation forms shown above correspond to the following implementations:

- `impl Shr<$name, Output=$name> for $name`
- `impl Shr<$rhs_ty, Output=$name> for $name`
- `impl<'a> Shr<&'a $name, Output=$name> for &'a $name`
- `impl<'a> Shr<$rhs_ty, Output=$name> for &'a $name`

The last form expands to all combinations of `$name` and `&$name` as the arguments.
*/
// ntbop Shr,      shr
#[macro_export]
macro_rules! NewtypeShr {
    ((*) $($tts:tt)*) => {
        NewtypeShr! { () $($tts)* }
        NewtypeShr! { (&self) $($tts)* }
        NewtypeShr! { (&Self) $($tts)* }
        NewtypeShr! { (&self, Self) $($tts)* }
    };
    (() $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Shr)::shr, kind: simple, item: $($tts)* }
    };
    ((&self) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Shr)::shr, kind: simple_ref, item: $($tts)* }
    };
    ((&self, $($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Shr)::shr, kind: ref_rhs_rewrap($($rhs)*), item: $($tts)* }
    };
    (($($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Shr)::shr, kind: rhs_rewrap($($rhs)*), item: $($tts)* }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeShrAssign!)]
    #[derive(NewtypeShrAssign!(&Self))]
    #[derive(NewtypeShrAssign!($rhs_ty))]
    struct $name($inner_ty);
}
// or:
macro_attr! {
    #[derive(NewtypeShrAssign!(*))]
    struct $name($inner_ty);
}
```

Derives an implementation of the `ShrAssign` trait by deferring to the implementation of the inner type.  The first three invocation forms shown above correspond to the following implementations:

- `impl ShrAssign<$name> for $name`
- `impl<'a> ShrAssign<&'a $name> for $name`
- `impl ShrAssign<$rhs_ty> for $name`

The last form expands to the first two forms.
*/
// ntbopass ShrAssign, shr_assign
#[macro_export]
#[cfg(op_assign)]
macro_rules! NewtypeShrAssign {
    ((*) $($tts:tt)*) => {
        NewtypeShrAssign! { () $($tts)* }
        NewtypeShrAssign! { (&self) $($tts)* }
    };
    (() $($tts:tt)*) => {
        newtype_wrap_bin_op_assign! { trait: (::std::ops::ShrAssign)::shr_assign, kind: simple, item: $($tts)* }
    };
    (($($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op_assign! { trait: (::std::ops::ShrAssign)::shr_assign, kind: rhs($($rhs)*), item: $($tts)* }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeNeg!)]
    #[derive(NewtypeNeg!(&self))]
    struct $name($inner_ty);
}
// or:
macro_attr! {
    #[derive(NewtypeNeg!(*))]
    struct $name($inner_ty);
}
```

Derives an implementation of the `Neg` trait by deferring to the implementation of the inner type.  The first two invocation forms shown above correspond to the following implementations:

- `impl Neg<Output=$name> for $name`
- `impl<'a> Neg<Output=$name> for &'a $name`

The last form expands to the first two forms.
*/
// ntuop Neg,      neg
#[macro_export]
macro_rules! NewtypeNeg {
    ((*) $($tts:tt)*) => {
        NewtypeNeg! { () $($tts)* }
        NewtypeNeg! { (&self) $($tts)* }
    };
    (() $($tts:tt)*) => {
        newtype_wrap_un_op! { trait: (::std::ops::Neg)::neg, kind: simple, item: $($tts)* }
    };
    ((&self) $($tts:tt)*) => {
        newtype_wrap_un_op! { trait: (::std::ops::Neg)::neg, kind: simple_ref, item: $($tts)* }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeNot!)]
    #[derive(NewtypeNot!(&self))]
    struct $name($inner_ty);
}
// or:
macro_attr! {
    #[derive(NewtypeNot!(*))]
    struct $name($inner_ty);
}
```

Derives an implementation of the `Not` trait by deferring to the implementation of the inner type.  The first two invocation forms shown above correspond to the following implementations:

- `impl Not<Output=$name> for $name`
- `impl<'a> Not<Output=$name> for &'a $name`

The last form expands to the first two forms.
*/
// ntuop Not,      not
#[macro_export]
macro_rules! NewtypeNot {
    ((*) $($tts:tt)*) => {
        NewtypeNot! { () $($tts)* }
        NewtypeNot! { (&self) $($tts)* }
    };
    (() $($tts:tt)*) => {
        newtype_wrap_un_op! { trait: (::std::ops::Not)::not, kind: simple, item: $($tts)* }
    };
    ((&self) $($tts:tt)*) => {
        newtype_wrap_un_op! { trait: (::std::ops::Not)::not, kind: simple_ref, item: $($tts)* }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeDeref!)]
    struct $name($inner_ty);
}
```

Derives an implementation of the `Deref` trait by deferring to the implementation of the inner type.
*/
// ntf Deref
#[macro_export]
macro_rules! NewtypeDeref {
    (() $(pub)* struct $name:ident(pub $t0:ty);) => {
        impl ::std::ops::Deref for $name {
            type Target = $t0;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };

    (() $(pub)* struct $name:ident($t0:ty);) => {
        impl ::std::ops::Deref for $name {
            type Target = $t0;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeDerefMut!)]
    struct $name($inner_ty);
}
```

Derives an implementation of the `DerefMut` trait by deferring to the implementation of the inner type.
*/
// ntf DerefMut
#[macro_export]
macro_rules! NewtypeDerefMut {
    (() $(pub)* struct $name:ident(pub $t0:ty);) => {
        impl ::std::ops::DerefMut for $name {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };

    (() $(pub)* struct $name:ident($t0:ty);) => {
        impl ::std::ops::DerefMut for $name {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeIndex!($index_ty))]
    struct $name($inner_ty);
}
```

Derives an implementation of the `Index<$index_ty>` trait by deferring to the implementation of the inner type.
*/
// nti Index
#[macro_export]
macro_rules! NewtypeIndex {
    (($index_ty:ty) $(pub)* struct $name:ident(pub $t0:ty);) => {
        impl ::std::ops::Index<$index_ty> for $name {
            type Output = <$t0 as ::std::ops::Index<$index_ty>>::Output;

            #[inline]
            fn index(&self, index: $index_ty) -> &Self::Output {
                (&self.0).index(index)
            }
        }
    };

    (($index_ty:ty) $(pub)* struct $name:ident($t0:ty);) => {
        impl ::std::ops::Index<$index_ty> for $name {
            type Output = <$t0 as ::std::ops::Index<$index_ty>>::Output;

            #[inline]
            fn index(&self, index: $index_ty) -> &Self::Output {
                (&self.0).index(index)
            }
        }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeIndexMut!($index_ty))]
    struct $name($inner_ty);
}
```

Derives an implementation of the `IndexMut<$index_ty>` trait by deferring to the implementation of the inner type.
*/
// nti IndexMut
#[macro_export]
macro_rules! NewtypeIndexMut {
    (($index_ty:ty) $(pub)* struct $name:ident(pub $t0:ty);) => {
        impl ::std::ops::IndexMut<$index_ty> for $name {
            #[inline]
            fn index_mut(&mut self, index: $index_ty) -> &mut Self::Output {
                (&mut self.0).index_mut(index)
            }
        }
    };

    (($index_ty:ty) $(pub)* struct $name:ident($t0:ty);) => {
        impl ::std::ops::IndexMut<$index_ty> for $name {
            #[inline]
            fn index_mut(&mut self, index: $index_ty) -> &mut Self::Output {
                (&mut self.0).index_mut(index)
            }
        }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeFrom!)]
    struct $name($inner_ty);
}
```

Derives two implementations of the `From` trait.  Specifically, it derives:

- `impl From<$inner_ty> for $name`
- `impl From<$name> for $inner_ty`

This enables packing/unpacking the newtype structure with `From` and `Into`.
*/
// ntX From
#[macro_export]
macro_rules! NewtypeFrom {
    (() $(pub)* struct $name:ident(pub $t0:ty);) => {
        impl ::std::convert::From<$t0> for $name {
            #[inline]
            fn from(v: $t0) -> Self {
                $name(v)
            }
        }
        impl ::std::convert::From<$name> for $t0 {
            #[inline]
            fn from(v: $name) -> Self {
                v.0
            }
        }
    };

    (() $(pub)* struct $name:ident($t0:ty);) => {
        impl ::std::convert::From<$t0> for $name {
            #[inline]
            fn from(v: $t0) -> Self {
                $name(v)
            }
        }
        impl ::std::convert::From<$name> for $t0 {
            #[inline]
            fn from(v: $name) -> Self {
                v.0
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! newtype_fmt {
    ($fmt_trait:ident, $name:ident) => {
        impl ::std::fmt::$fmt_trait for $name {
            fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::$fmt_trait::fmt(&self.0, fmt)
            }
        }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeBinary!)]
    struct $name($inner_ty);
}
```

Derives an implementation of the `Binary` trait by deferring to the implementation of the inner type.
*/
// ntf Binary
#[macro_export]
macro_rules! NewtypeBinary {
    (() $(pub)* struct $name:ident $_field:tt;) => {
        newtype_fmt! { Binary, $name }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeDebug!)]
    struct $name($inner_ty);
}
```

Derives an implementation of the `Debug` trait by deferring to the implementation of the inner type.
*/
// ntf Debug
#[macro_export]
macro_rules! NewtypeDebug {
    (() $(pub)* struct $name:ident $_field:tt;) => {
        newtype_fmt! { Debug, $name }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeDisplay!)]
    struct $name($inner_ty);
}
```

Derives an implementation of the `Display` trait by deferring to the implementation of the inner type.
*/
// ntf Display
#[macro_export]
macro_rules! NewtypeDisplay {
    (() $(pub)* struct $name:ident $_field:tt;) => {
        newtype_fmt! { Display, $name }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeLowerExp!)]
    struct $name($inner_ty);
}
```

Derives an implementation of the `LowerExp` trait by deferring to the implementation of the inner type.
*/
// ntf LowerExp
#[macro_export]
macro_rules! NewtypeLowerExp {
    (() $(pub)* struct $name:ident $_field:tt;) => {
        newtype_fmt! { LowerExp, $name }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeLowerHex!)]
    struct $name($inner_ty);
}
```

Derives an implementation of the `LowerHex` trait by deferring to the implementation of the inner type.
*/
// ntf LowerHex
#[macro_export]
macro_rules! NewtypeLowerHex {
    (() $(pub)* struct $name:ident $_field:tt;) => {
        newtype_fmt! { LowerHex, $name }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeOctal!)]
    struct $name($inner_ty);
}
```

Derives an implementation of the `Octal` trait by deferring to the implementation of the inner type.
*/
// ntf Octal
#[macro_export]
macro_rules! NewtypeOctal {
    (() $(pub)* struct $name:ident $_field:tt;) => {
        newtype_fmt! { Octal, $name }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypePointer!)]
    struct $name($inner_ty);
}
```

Derives an implementation of the `Pointer` trait by deferring to the implementation of the inner type.
*/
// ntf Pointer
#[macro_export]
macro_rules! NewtypePointer {
    (() $(pub)* struct $name:ident $_field:tt;) => {
        newtype_fmt! { Pointer, $name }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeUpperExp!)]
    struct $name($inner_ty);
}
```

Derives an implementation of the `UpperExp` trait by deferring to the implementation of the inner type.
*/
// ntf UpperExp
#[macro_export]
macro_rules! NewtypeUpperExp {
    (() $(pub)* struct $name:ident $_field:tt;) => {
        newtype_fmt! { UpperExp, $name }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeUpperHex!)]
    struct $name($inner_ty);
}
```

Derives an implementation of the `UpperHex` trait by deferring to the implementation of the inner type.
*/
// ntf UpperHex
#[macro_export]
macro_rules! NewtypeUpperHex {
    (() $(pub)* struct $name:ident $_field:tt;) => {
        newtype_fmt! { UpperHex, $name }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeProduct!)]
    #[derive(NewtypeProduct!(&Self))]
    struct $name($inner_ty);
}
```

Derives an implementation of the `Product` trait by deferring to the implementation of the inner type.  The two invocation forms shown above correspond to the following implementations:

- `impl Product<$name> for $name`
- `impl<'a> Product<&'a $name> for $name`
*/
// ntiter (w+)
#[cfg(iter_sum_product)]
#[macro_export]
macro_rules! NewtypeProduct {
    ($arg:tt $(pub)* struct $name:ident(pub $t0:ty);) => {
        NewtypeProduct! { $arg struct $name($t0); }
    };

    (() $(pub)* struct $name:ident($t0:ty);) => {
        impl ::std::iter::Product<$name> for $name {
            #[inline]
            fn product<I>(iter: I) -> Self
            where I: Iterator<Item=$name> {
                $name(iter.map(|e| e.0).product::<$t0>())
            }
        }
    };

    ((&Self) $(pub)* struct $name:ident($t0:ty);) => {
        impl<'a> ::std::iter::Product<&'a $name> for $name {
            #[inline]
            fn product<I>(iter: I) -> Self
            where I: Iterator<Item=&'a $name> {
                $name(iter.map(|e| &e.0).product::<$t0>())
            }
        }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeSum!)]
    #[derive(NewtypeSum!(&Self))]
    struct $name($inner_ty);
}
```

Derives an implementation of the `Sum` trait by deferring to the implementation of the inner type.  The two invocation forms shown above correspond to the following implementations:

- `impl Sum<$name> for $name`
- `impl<'a> Sum<&'a $name> for $name`
*/
// ntiter (w+)
#[cfg(iter_sum_product)]
#[macro_export]
macro_rules! NewtypeSum {
    ($arg:tt $(pub)* struct $name:ident(pub $t0:ty);) => {
        NewtypeSum! { $arg struct $name($t0); }
    };

    (() $(pub)* struct $name:ident($t0:ty);) => {
        impl ::std::iter::Sum<$name> for $name {
            #[inline]
            fn sum<I>(iter: I) -> Self
            where I: Iterator<Item=$name> {
                $name(iter.map(|e| e.0).sum::<$t0>())
            }
        }
    };

    ((&Self) $(pub)* struct $name:ident($t0:ty);) => {
        impl<'a> ::std::iter::Sum<&'a $name> for $name {
            #[inline]
            fn sum<I>(iter: I) -> Self
            where I: Iterator<Item=&'a $name> {
                $name(iter.map(|e| &e.0).sum::<$t0>())
            }
        }
    };
}
