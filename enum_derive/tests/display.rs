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
