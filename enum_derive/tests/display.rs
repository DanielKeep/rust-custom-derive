/*
Copyright ⓒ 2015 rust-custom-derive contributors.

Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
#[macro_use] extern crate custom_derive;
#[macro_use] extern crate enum_derive;

custom_derive! {
    #[derive(EnumDisplay)]
    pub enum Get {
        Up,
        /// And
        Down,
        /** And */
        AllAround
    }
}

custom_derive! {
    #[derive(EnumDisplay)]
    pub enum Degenerate {}
}

#[test]
fn test_next_variant() {
    assert_eq!(format!("{}", Get::Up), "Up");
    assert_eq!(format!("{}", Get::Down), "Down");
    assert_eq!(format!("{}", Get::AllAround), "AllAround");
}
