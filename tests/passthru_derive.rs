/*
Copyright ⓒ 2015 macro-attr contributors.

Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
#[macro_use] extern crate macro_attr;
extern crate rustc_serialize;

macro_attr! {
    #[derive(Clone, Hash, RustcEncodable, RustcDecodable, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Copy)]
    pub struct Dummy(u32);
}

#[test]
fn test_passthru_derive() {}
