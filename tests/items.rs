/*
Copyright ⓒ 2016 macro-attr contributors.

Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
#[macro_use] extern crate macro_attr;

macro_attr! {
    const DUMMY_CONST: () = ();
}

macro_attr! {
    enum DummyEnum { V }
}

macro_attr! {
    fn dummy_fn() {}
}

macro_attr! {
    impl DummyStruct {
        fn f(self) {}
    }
}

macro_attr! {
    mod dummy_mod {
        pub fn f() {}
    }
}

macro_attr! {
    static DUMMY_STATIC: () = ();
}

macro_attr! {
    struct DummyStruct;
}

macro_attr! {
    trait DummyTrait {}
}

macro_attr! {
    type DummyType = ();
}

macro_attr! {
    use self::dummy_fn as dummy_use;
}

#[cfg(never)]
mod super_dummy {
    macro_attr! {
        extern crate dummy_crate;
    }

    macro_attr! {
        extern "C" dummy_extern();
    }
}

#[test]
fn test_items() {
    let _: () = DUMMY_CONST;
    let _: DummyEnum = DummyEnum::V;
    let _: () = dummy_fn();
    let _: () = DummyStruct.f();
    let _: () = dummy_mod::f();
    let _: DummyStruct = DummyStruct;
    let _: Option<&DummyTrait> = None;
    let _: DummyType = ();
    let _: () = DUMMY_STATIC;
    let _: () = dummy_use();
}
