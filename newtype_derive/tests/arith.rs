#[macro_use] extern crate custom_derive;
#[macro_use] extern crate newtype_derive;

custom_derive! {
    #[derive(Copy, Clone, Eq, PartialEq, Debug,
        NewtypeAdd, NewtypeAdd(ref),
        NewtypeBitAnd, NewtypeBitAnd(ref),
        NewtypeBitOr, NewtypeBitOr(ref),
        NewtypeBitXor, NewtypeBitXor(ref),
        NewtypeDiv, NewtypeDiv(ref),
        NewtypeMul, NewtypeMul(ref),
        NewtypeRem, NewtypeRem(ref),
        NewtypeSub, NewtypeSub(ref),

        NewtypeShl(), NewtypeShl(ref), NewtypeShl(<_>), NewtypeShl(ref <_>),
        NewtypeShr(), NewtypeShr(ref), NewtypeShr(<_>), NewtypeShr(ref <_>)

        // The macro recursion limit gives out beyond this point. :(
        )]
    pub struct Dummy(i32);
}

NewtypeNeg! { ()    pub struct Dummy(i32); }
NewtypeNeg! { (ref) pub struct Dummy(i32); }
NewtypeNot! { ()    pub struct Dummy(i32); }
NewtypeNot! { (ref) pub struct Dummy(i32); }
NewtypeFrom! { ()   pub struct Dummy(i32); }

#[test]
fn test_arith() {
    let a = Dummy::from(4);
    let b = Dummy::from(7);

    assert_eq!(a + b, Dummy::from(4 + 7));
    assert_eq!(&a + &b, Dummy::from(4 + 7));
    assert_eq!(a & b, Dummy::from(4 & 7));
    assert_eq!(&a & &b, Dummy::from(4 & 7));
    assert_eq!(a | b, Dummy::from(4 | 7));
    assert_eq!(&a | &b, Dummy::from(4 | 7));
    assert_eq!(a ^ b, Dummy::from(4 ^ 7));
    assert_eq!(&a ^ &b, Dummy::from(4 ^ 7));
    assert_eq!(a / b, Dummy::from(4 / 7));
    assert_eq!(&a / &b, Dummy::from(4 / 7));
    assert_eq!(a * b, Dummy::from(4 * 7));
    assert_eq!(&a * &b, Dummy::from(4 * 7));
    assert_eq!(a % b, Dummy::from(4 % 7));
    assert_eq!(&a % &b, Dummy::from(4 % 7));
    assert_eq!(a - b, Dummy::from(4 - 7));
    assert_eq!(&a - &b, Dummy::from(4 - 7));

    assert_eq!(a << b, Dummy::from(4 << 7));
    assert_eq!(&a << &b, Dummy::from(4 << 7));
    assert_eq!(a << 7, Dummy::from(4 << 7));

    assert_eq!(a >> b, Dummy::from(4 >> 7));
    assert_eq!(&a >> &b, Dummy::from(4 >> 7));
    assert_eq!(a >> 7, Dummy::from(4 >> 7));

    assert_eq!(-a, Dummy::from(-4));
    assert_eq!(-&a, Dummy::from(-4));
    assert_eq!(!a, Dummy::from(!4));
    assert_eq!(!&a, Dummy::from(!4));
}
