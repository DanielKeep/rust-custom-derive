#[macro_use] extern crate custom_derive;
extern crate rustc_serialize;

custom_derive! {
    #[derive(Clone, Hash, RustcEncodable, RustcDecodable, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Copy)]
    pub struct Dummy(u32);
}

#[test]
fn test_passthru_derive() {}
