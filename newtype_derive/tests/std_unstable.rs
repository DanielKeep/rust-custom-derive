/*
Copyright ⓒ 2015 rust-custom-derive contributors.

Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
#![cfg(feature="std-unstable")]
#![feature(zero_one)]

#[macro_use] extern crate custom_derive;
#[macro_use] extern crate newtype_derive;

use std::num::{One, Zero};

custom_derive! {
    #[derive(Eq, PartialEq, Debug, NewtypeOne, NewtypeZero)]
    struct Dummy(i32);
}

#[test]
fn test_one_zero() {
    assert_eq!(Dummy::zero(), Dummy(0));
    assert_eq!(Dummy::one(), Dummy(1));
}
