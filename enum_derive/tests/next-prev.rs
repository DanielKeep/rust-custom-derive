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
    #[derive(Debug, PartialEq, NextVariant, PrevVariant)]
    pub enum Get {
        Up,
        /// And
        Down,
        /** And */
        AllAround
    }
}

// We can't test this since it *literally* can't be called.
custom_derive! {
    #[derive(NextVariant, PrevVariant)]
    enum Nada {}
}

#[test]
fn test_next_variant() {
    assert_eq!(Get::Up.next_variant(),        Some(Get::Down));
    assert_eq!(Get::Down.next_variant(),      Some(Get::AllAround));
    assert_eq!(Get::AllAround.next_variant(), None);

    assert_eq!(Get::Up.prev_variant(),        None);
    assert_eq!(Get::Down.prev_variant(),      Some(Get::Up));
    assert_eq!(Get::AllAround.prev_variant(), Some(Get::Down));
}
