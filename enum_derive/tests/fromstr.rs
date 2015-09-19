#[macro_use] extern crate custom_derive;
#[macro_use] extern crate enum_derive;

use ::std::str::FromStr;

custom_derive! {
    #[derive(Debug, PartialEq, FromStr)]
    pub enum Get {
        Up,
        /// And
        Down,
        /** And */
        AllAround
    }
}

custom_derive! {
    #[derive(Debug, PartialEq, FromStr)]
    pub enum Degenerate {

    }
}

#[test]
fn test_next_variant() {
    assert_eq!(Get::from_str("Up").unwrap(), Get::Up);
    assert_eq!(Get::from_str("Down").unwrap(), Get::Down);
    assert_eq!(Get::from_str("AllAround").unwrap(), Get::AllAround);
}
