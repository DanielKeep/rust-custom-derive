#![feature(trace_macros)]
#[macro_use] extern crate custom_derive;
#[macro_use] extern crate enum_derive;

// trace_macros!(true);

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
