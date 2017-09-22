/*
Copyright ⓒ 2017 macro-attr contributors.

Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
/*!
This crate provides several macros for deriving implementations of various traits for structures.

All of these macros are designed to be used with the [`macro-attr`](https://crates.io/crates/macro-attr) crate, though they can be used independent of it.

# Using Without `macro_attr!`

Although designed to be used with `macro_attr!`, all of the macros in this crate can be used without it.  The following:

```rust
# #[macro_use] extern crate macro_attr;
# #[macro_use] extern crate struct_derive;
macro_attr! {
    #[derive(Copy, Clone, Debug, StructNew!)]
    pub struct Meters { value: f32 }
}
# fn main() {}
```

Can also be written as:

```rust
# #[macro_use] extern crate struct_derive;
#[derive(Copy, Clone, Debug)]
pub struct Meters { value: f32 }

StructNew! { () pub struct Meters { value: f32 } }
# fn main() {}
```
*/
#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use] mod common;
#[macro_use] mod type_index;

pub use type_index::{TypeIndex, TypeIndexMut};
