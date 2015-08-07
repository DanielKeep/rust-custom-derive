#[macro_use] extern crate custom_derive;

macro_rules! Dummy {
    ($($tts:tt)*) => {};
}

custom_derive! {
    #[derive(Dummy)]
    enum Foo { Bar }
}

#[test]
fn test_empty_bi_derives() {
    let _ = Foo::Bar;
}
