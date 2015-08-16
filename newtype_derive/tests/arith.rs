#![feature(trace_macros)]
#![recursion_limit = "128"]
#[macro_use] extern crate custom_derive;
#[macro_use] extern crate newtype_derive;

// trace_macros!(true);

custom_derive! {
    #[derive(Copy, Clone, Eq, PartialEq, Debug,
        NewtypeAdd, NewtypeAdd(&self), NewtypeAdd(i32), NewtypeAdd(&self, i32),
        NewtypeBitAnd, NewtypeBitAnd(&self),
        NewtypeBitOr, NewtypeBitOr(&self),
        NewtypeBitXor, NewtypeBitXor(&self),
        NewtypeDiv, NewtypeDiv(&self),
        NewtypeMul, NewtypeMul(&self),
        NewtypeRem, NewtypeRem(&self),
        NewtypeSub, NewtypeSub(&self),

        NewtypeShl(), NewtypeShl(&self), NewtypeShl(usize), NewtypeShl(&self, usize),
        NewtypeShr(), NewtypeShr(&self), NewtypeShr(usize), NewtypeShr(&self, usize),

        NewtypeNeg, NewtypeNeg(&self),
        NewtypeNot, NewtypeNot(ref),

        NewtypeFrom
        )]
    pub struct Dummy(i32);
}

#[test]
fn test_arith() {
    let a = Dummy::from(4);
    let b = Dummy::from(7);

    assert_eq!(a + b, Dummy::from(4 + 7));
    assert_eq!(&a + &b, Dummy::from(4 + 7));
    assert_eq!(a + 7, Dummy::from(4 + 7));
    assert_eq!(&a + 7, Dummy::from(4 + 7));
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
