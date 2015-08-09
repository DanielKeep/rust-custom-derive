#[macro_use] extern crate custom_derive;
#[macro_use] extern crate newtype_derive;

custom_derive! {
    #[derive(Clone, Eq, PartialEq, Debug,
        NewtypeFrom,
        NewtypeDeref, NewtypeDerefMut,
        NewtypeIndex(usize), NewtypeIndexMut(usize)
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
