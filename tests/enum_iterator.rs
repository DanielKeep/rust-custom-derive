#[macro_use] extern crate custom_derive;

macro_rules! EnumIterator {
    (() $(pub)* enum $name:ident { $($body:tt)* }) => {
        EnumIterator! {
            @collect_variants ($name),
            ($($body)*,) -> ()
        }
    };

    (
        @collect_variants ($name:ident),
        ($(,)*) -> ($($var_names:ident,)*)
    ) => {
        type NameIter = ::std::vec::IntoIter<&'static str>;
        type VariantIter = ::std::vec::IntoIter<$name>;

        impl $name {
            #[allow(dead_code)]
            pub fn iter_variants() -> VariantIter {
                vec![$($name::$var_names),*].into_iter()
            }

            #[allow(dead_code)]
            pub fn iter_variant_names() -> NameIter {
                vec![$(stringify!($var_names)),*].into_iter()
            }
        }
    };

    (
        @collect_variants $fixed:tt,
        ($var:ident $(= $_val:expr)*, $($tail:tt)*) -> ($($var_names:tt)*)
    ) => {
        EnumIterator! {
            @collect_variants $fixed,
            ($($tail)*) -> ($($var_names)* $var,)
        }
    };

    (
        @collect_variants ($name:ident),
        ($var:ident $_struct:tt, $($tail:tt)*) -> ($($var_names:tt)*)
    ) => {
        const _error: () = concat!(
            "cannot derive EnumIterator for ",
            stringify!($name),
            ", due to non-unitary variant ",
            stringify!($var),
            "."
        );
    };
}

custom_derive! {
    #[derive(Debug, PartialEq, EnumIterator)]
    enum Get { Up, Down, AllAround }
}

#[test]
fn test_enum_iterator() {
    let vs: Vec<_> = Get::iter_variant_names().zip(Get::iter_variants()).collect();
    assert_eq!(&*vs, &[("Up", Get::Up), ("Down", Get::Down), ("AllAround", Get::AllAround)]);
}
