/*
Copyright ⓒ 2016 macro-attr contributors.

Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
#[macro_use] extern crate enum_derive;
#[macro_use] extern crate macro_attr;

pub trait SizeOf {
    fn size_of(&self) -> usize;
}

impl SizeOf for u32 {
    fn size_of(&self) -> usize {
        4
    }
}

impl SizeOf for u64 {
    fn size_of(&self) -> usize {
        8
    }
}

pub trait Replace<T> {
    fn replace(&mut self, v: T);
}

impl Replace<u32> for u32 {
    fn replace(&mut self, v: u32) {
        *self = v;
    }
}

impl Replace<u32> for u64 {
    fn replace(&mut self, v: u32) {
        *self = v as u64;
    }
}

macro_attr! {
    #[derive(
        Debug, PartialEq,
        EnumInnerAsTrait!(pub as_size_of -> &SizeOf),
        EnumInnerAsTrait!(as_replace -> &mut Replace<u32>),
        EnumInnerAsTrait!(pub as_display -> &std::fmt::Display),
    )]
    pub enum Value {
        U32(u32),
        U64(u64),
    }
}

macro_attr! {
    #[derive(
        Debug, PartialEq,
        EnumInnerAsTrait!(as_size_of -> &SizeOf),
        EnumInnerAsTrait!(pub as_replace -> &mut Replace<u32>),
    )]
    pub enum Degenerate {}
}

#[test]
fn test_inner_as_trait() {
    assert_eq!(Value::U32(42).as_size_of().size_of(), 4);
    assert_eq!(Value::U64(24).as_size_of().size_of(), 8);
    assert_eq!(&*format!("{}", Value::U32(42).as_display()), "42");
    assert_eq!(&*format!("{}", Value::U64(24).as_display()), "24");
    {
        let mut v = Value::U32(42);
        v.as_replace().replace(1701);
        assert_eq!(v, Value::U32(1701));
    }
    {
        let mut v = Value::U64(24);
        v.as_replace().replace(81);
        assert_eq!(v, Value::U64(81));
    }
}

#[allow(dead_code)]
fn test_inner_as_trait_degenerate(mut v: Degenerate) {
    let _ = v.as_size_of();
    let _ = v.as_replace();
}
