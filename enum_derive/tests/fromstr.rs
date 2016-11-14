/*
Copyright ⓒ 2015 macro-attr contributors.

Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
#[macro_use] extern crate enum_derive;
#[macro_use] extern crate macro_attr;

macro_attr! {
    #[derive(Debug, PartialEq, EnumFromStr!)]
    pub enum Get {
        Up,
        /// And
        Down,
        /** And */
        AllAround
    }
}

macro_attr! {
    #[derive(Debug, PartialEq, EnumFromStr!)]
    pub enum Degenerate {}
}

#[test]
fn test_next_variant() {
    use enum_derive::ParseEnumError;

    assert_eq!("Up".parse(), Ok(Get::Up));
    assert_eq!("Down".parse(), Ok(Get::Down));
    assert_eq!("AllAround".parse(), Ok(Get::AllAround));
    assert_eq!("Edgy".parse::<Degenerate>(), Err(ParseEnumError));

    assert_eq!("Singularity".parse::<Degenerate>(), Err(ParseEnumError));
}
