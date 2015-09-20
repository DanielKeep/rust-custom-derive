#[macro_use] extern crate custom_derive;
#[macro_use] extern crate enum_derive;

custom_derive! {
    #[derive(Debug, PartialEq, EnumFromStr)]
    pub enum Get {
        Up,
        /// And
        Down,
        /** And */
        AllAround
    }
}

custom_derive! {
    #[derive(Debug, PartialEq, EnumFromStr)]
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
