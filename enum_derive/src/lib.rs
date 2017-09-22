/*
Copyright ⓒ 2015-2017 macro-attr contributors.

Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
/*!
This crate provides several macros for deriving some useful methods for unitary enums (*i.e.* enums where variants do not have payloads).

All of these macros are designed to be used with the [`macro-attr`](https://crates.io/crates/macro-attr) crate, though they can be used independent of it.

> **Note**: see also the `TryFrom!` macro provided by the [`conv`](https://crates.io/crates/conv) crate to derive a function for creating enum values from integer values.

# Using Without `macro_attr!`

Although designed to be used with `macro_attr!`, all of the macros in this crate can be used without it.  The following:

```rust
# #[macro_use] extern crate macro_attr;
# #[macro_use] extern crate enum_derive;
macro_attr! {
    #[derive(Copy, Clone, Debug, IterVariants!(Vars))]
    enum ItAintRight { BabeNo, NoNo, BoyBoy }
}
# fn main() {}
```

Can also be written as:

```rust
# #[macro_use] extern crate macro_attr;
# #[macro_use] extern crate enum_derive;
#[derive(Copy, Clone, Debug)]
enum ItAintRight { BabeNo, NoNo, BoyBoy }

IterVariants! { (Vars) enum ItAintRight { BabeNo, NoNo, BoyBoy } }
# fn main() {}
```
*/
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))] extern crate core as std;

#[macro_use] mod common;
#[macro_use] mod fmt;
#[macro_use] mod from_str;
#[macro_use] mod inner;
#[macro_use] pub mod iter_variants;
#[macro_use] mod step_variants;
#[macro_use] mod tag;

pub use from_str::ParseEnumError;
