/*
Copyright ⓒ 2015 macro-attr contributors.

Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
#[macro_use] extern crate macro_attr;
#[macro_use] extern crate newtype_derive;

macro_attr! {
    #[derive(Copy, Clone, Eq, PartialEq, Debug,
        NewtypeAdd!(*),
        NewtypeNeg!(*),
        NewtypeFrom!
        )]
    pub struct Dummy(i32);
}

macro_attr! {
    #[derive(Copy, Clone, Eq, PartialEq, Debug,
        NewtypeAdd!(*),
        NewtypeNeg!(*),
        NewtypeFrom!
        )]
    pub struct DummyPub(pub i32);
}

#[test]
fn test_arith() {
    let a = Dummy::from(4);
    let b = Dummy::from(7);
    let c = Dummy::from(11);

    assert_eq!(a + b, c);
    assert_eq!(&a + b, c);
    assert_eq!(a + &b, c);
    assert_eq!(&a + &b, c);

    assert_eq!(-a, Dummy::from(-4));
    assert_eq!(-&a, Dummy::from(-4));

    let _ = DummyPub(0);
}
