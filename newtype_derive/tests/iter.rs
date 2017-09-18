/*
Copyright ⓒ 2015-2017 macro-attr contributors.

Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
#![cfg(iter_sum_product)]
#[macro_use] extern crate macro_attr;
#[macro_use] extern crate newtype_derive;

macro_attr! {
    #[derive(
        Clone, Eq, PartialEq, Debug,
        NewtypeSum!, NewtypeSum!(&Self),
        NewtypeProduct!, NewtypeProduct!(&Self),
    )]
    struct Dummy(i32);
}

#[test]
fn test_sum_product() {
    let dummies = &[Dummy(2), Dummy(3)];
    assert_eq!(dummies.into_iter().sum::<Dummy>(), Dummy(5));
    assert_eq!(dummies.into_iter().cloned().sum::<Dummy>(), Dummy(5));
    assert_eq!(dummies.into_iter().product::<Dummy>(), Dummy(6));
    assert_eq!(dummies.into_iter().cloned().product::<Dummy>(), Dummy(6));
}
