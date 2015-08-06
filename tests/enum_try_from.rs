#[macro_use] extern crate custom_derive;
use custom_derive::TryFrom;

custom_derive! {
    #[derive(Debug, PartialEq, TryFrom(u8))]
    enum Get {
        /// The +Z direction
        Up,
        /// The -Z direction
        Down,
        /// Just... everywhere
        AllAround
    }
}

#[test]
fn test_try_from() {
    assert_eq!(Get::try_from(0u8), Ok(Get::Up));
    assert_eq!(Get::try_from(1u8), Ok(Get::Down));
    assert_eq!(Get::try_from(2u8), Ok(Get::AllAround));
    assert_eq!(Get::try_from(3u8), Err(3u8));
}
