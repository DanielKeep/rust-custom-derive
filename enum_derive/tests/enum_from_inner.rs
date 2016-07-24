/*
Copyright ⓒ 2016 rust-custom-derive contributors.

Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
#[macro_use] extern crate custom_derive;
#[macro_use] extern crate enum_derive;

custom_derive! {
    #[derive(Debug, PartialEq, EnumFromInner)]
    pub enum Value {
        Unit(()),
        Int(i64),
        Str(&'static str),
    }
}

custom_derive! {
    #[derive(Debug, PartialEq, EnumFromInner)]
    pub enum Degenerate {}
}

#[test]
fn test_from_inner() {
    assert_eq!(Into::<Value>::into(()), Value::Unit(()));
    assert_eq!(Into::<Value>::into(42i64), Value::Int(42));
    assert_eq!(Into::<Value>::into("fry"), Value::Str("fry"));

    assert_eq!({ let v: Value = From::from(()); v }, Value::Unit(()));
    assert_eq!({ let v: Value = From::from(42i64); v }, Value::Int(42));
    assert_eq!({ let v: Value = From::from("fry"); v }, Value::Str("fry"));
}
