#[macro_use] extern crate custom_derive;

trait TryFrom<Src>: Sized {
    type Err;
    fn try_from(src: Src) -> Result<Self, Self::Err>;
}

macro_rules! TryFrom {
    (($prim:ty) $(pub)* enum $name:ident { $($body:tt)* }) => {
        TryFrom! {
            @collect_variants ($name, $prim),
            ($($body)*,) -> ()
        }
    };

    (
        @collect_variants ($name:ident, $prim:ty),
        ($(,)*) -> ($($var_names:ident,)*)
    ) => {
        impl TryFrom<$prim> for $name {
            type Err = $prim;
            fn try_from(src: $prim) -> Result<$name, $prim> {
                $(
                    if src == $name::$var_names as $prim {
                        return Ok($name::$var_names);
                    }
                )*
                Err(src)
            }
        }
    };

    (
        @collect_variants $fixed:tt,
        ($var:ident $(= $_val:expr)*, $($tail:tt)*) -> ($($var_names:tt)*)
    ) => {
        TryFrom! {
            @collect_variants $fixed,
            ($($tail)*) -> ($($var_names)* $var,)
        }
    };

    (
        @collect_variants ($name:ident),
        ($var:ident $_struct:tt, $($tail:tt)*) -> ($($var_names:tt)*)
    ) => {
        const _error: () = concat!(
            "cannot derive TryFrom for ",
            stringify!($name),
            ", due to non-unitary variant ",
            stringify!($var),
            "."
        );
    };
}

custom_derive! {
    #[derive(Debug, PartialEq, TryFrom(u8))]
    enum Get { Up, Down, AllAround }
}

#[test]
fn test_try_from() {
    assert_eq!(Get::try_from(0u8), Ok(Get::Up));
    assert_eq!(Get::try_from(1u8), Ok(Get::Down));
    assert_eq!(Get::try_from(2u8), Ok(Get::AllAround));
    assert_eq!(Get::try_from(3u8), Err(3u8));
}
