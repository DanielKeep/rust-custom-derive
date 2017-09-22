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
#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use] mod common;
#[macro_use] mod arith_bin;
#[macro_use] mod arith_una;
#[macro_use] mod fmt;
#[macro_use] mod forward;
#[macro_use] mod iter;

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
