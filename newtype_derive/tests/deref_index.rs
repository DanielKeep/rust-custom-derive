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
    #[derive(Clone, Eq, PartialEq, Debug,
        NewtypeFrom!,
        NewtypeDeref!, NewtypeDerefMut!,
        NewtypeIndex!(usize), NewtypeIndexMut!(usize)
        )]
    pub struct Dummy(Vec<i32>);
}

#[test]
fn test_deref_index() {
    let mut a = Dummy::from(vec![1, 2, 3]);

    assert_eq!(a.len(), 3);
    a.push(4);
    assert_eq!(&**a, &[1, 2, 3, 4][..]);

    assert_eq!(a[1], 2);
    a[2] = 5;
    assert_eq!(a[2], 5);
}
