pub trait TryFrom<Src> {
    type Err;
    fn try_from(src: Src) -> Result<Self, Self::Err>;
}

#[macro_export]
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
        impl $crate::TryFrom<$prim> for $name {
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
        (#[$_attr:meta] $($tail:tt)*) -> $var_names:tt
    ) => {
        TryFrom! {
            @skip_meta $fixed,
            ($($tail)*) -> $var_names
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

    (
        @skip_meta $fixed:tt,
        (#[$_attr:meta] $($tail:tt)*) -> $var_names:tt
    ) => {
        TryFrom! {
            @skip_meta $fixed,
            ($($tail)*) -> $var_names
        }
    };

    (
        @skip_meta $fixed:tt,
        ($var:ident $($tail:tt)*) -> $var_names:tt
    ) => {
        TryFrom! {
            @collect_variants $fixed,
            ($var $($tail)*) -> $var_names
        }
    };
}


