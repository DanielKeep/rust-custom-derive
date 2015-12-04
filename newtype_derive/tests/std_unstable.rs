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
