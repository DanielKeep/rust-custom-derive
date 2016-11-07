/*
Copyright ⓒ 2016 rust-custom-derive contributors.

Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
#[macro_use] extern crate custom_derive;

custom_derive! {
    const DUMMY_CONST: () = ();
}

custom_derive! {
    enum DummyEnum { V }
}

custom_derive! {
    fn dummy_fn() {}
}

custom_derive! {
    impl DummyStruct {
        fn f(self) {}
    }
}

custom_derive! {
    mod dummy_mod {
        pub fn f() {}
    }
}

custom_derive! {
    static DUMMY_STATIC: () = ();
}

custom_derive! {
    struct DummyStruct;
}

custom_derive! {
    trait DummyTrait {}
}

custom_derive! {
    type DummyType = ();
}

custom_derive! {
    use self::dummy_fn as dummy_use;
}

#[cfg(never)]
mod super_dummy {
    custom_derive! {
        extern crate dummy_crate;
    }

    custom_derive! {
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
