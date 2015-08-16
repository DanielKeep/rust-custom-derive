#[macro_export]
macro_rules! enum_derive_util {
    (@as_expr $e:expr) => {$e};
    (@as_item $i:item) => {$i};
    (@first_expr $head:expr, $($tail:expr),*) => {$head};
    (@first_expr $head:expr) => {$head};

    (
        @collect_unitary_variants ($callback:ident { $($args:tt)* }),
        ($(,)*) -> ($($var_names:ident,)*)
    ) => {
        enum_derive_util! {
            @as_item
            $callback!{ $($args)* ($($var_names),*) }
        }
    };

    (
        @collect_unitary_variants $fixed:tt,
        (#[$_attr:meta] $($tail:tt)*) -> ($($var_names:tt)*)
    ) => {
        enum_derive_util! {
            @collect_unitary_variants $fixed,
            ($($tail)*) -> ($($var_names)*)
        }
    };

    (
        @collect_unitary_variants $fixed:tt,
        ($var:ident $(= $_val:expr)*, $($tail:tt)*) -> ($($var_names:tt)*)
    ) => {
        enum_derive_util! {
            @collect_unitary_variants $fixed,
            ($($tail)*) -> ($($var_names)* $var,)
        }
    };

    (
        @collect_unitary_variants ($name:ident),
        ($var:ident $_struct:tt, $($tail:tt)*) -> ($($var_names:tt)*)
    ) => {
        const _error: () = "cannot parse unitary variants from enum with non-unitary variants";
    };
}

#[macro_export]
macro_rules! IterVariants {
    (
        @expand ($($pub_:tt)*) $itername:ident, $name:ident ()
    ) => {
        enum_derive_util! { @as_item $($pub_)* struct $itername; }

        impl ::std::iter::Iterator for $itername {
            type Item = $name;
            fn next(&mut self) -> Option<Self::Item> {
                None
            }
        }

        enum_derive_util! {
            @as_item
            impl $name {
                #[allow(dead_code)]
                $($pub_)* fn iter_variants() -> $itername {
                    $itername
                }
            }
        }
    };

    (
        @expand ($($pub_:tt)*) $itername:ident, $name:ident ($($var_names:ident),*)
    ) => {
        enum_derive_util! { @as_item $($pub_)* struct $itername(Option<$name>); }

        IterVariants! { @iter ($itername, $name), ($($var_names,)*) -> () }

        enum_derive_util! {
            @as_item
            impl $name {
                #[allow(dead_code)]
                $($pub_)* fn iter_variants() -> $itername {
                    $itername(Some(enum_derive_util!(@first_expr $($name::$var_names),+)))
                }
            }
        }
    };

    (
        @iter ($itername:ident, $name:ident), () -> ($($body:tt)*)
    ) => {
        enum_derive_util! {
            @as_item
            impl ::std::iter::Iterator for $itername {
                type Item = $name;
                fn next(&mut self) -> Option<Self::Item> {
                    let next_item = match self.0 {
                        $($body)*
                        None => None
                    };
                    ::std::mem::replace(&mut self.0, next_item)
                }
            }
        }
    };

    (
        @iter ($itername:ident, $name:ident), ($a:ident, $b:ident, $($rest:tt)*) -> ($($body:tt)*)
    ) => {
        IterVariants! {
            @iter ($itername, $name), ($b, $($rest)*)
            -> (
                $($body)*
                Some($name::$a) => Some($name::$b),
            )
        }
    };

    (
        @iter ($itername:ident, $name:ident), ($a:ident,) -> ($($body:tt)*)
    ) => {
        IterVariants! {
            @iter ($itername, $name), ()
            -> (
                $($body)*
                Some($name::$a) => None,
            )
        }
    };

    (($itername:ident) pub enum $name:ident { $($body:tt)* }) => {
        enum_derive_util! {
            @collect_unitary_variants
            (IterVariants { @expand (pub) $itername, $name }),
            ($($body)*,) -> ()
        }
    };

    (($itername:ident) enum $name:ident { $($body:tt)* }) => {
        enum_derive_util! {
            @collect_unitary_variants
            (IterVariants { @expand () $itername, $name }),
            ($($body)*,) -> ()
        }
    };
}

#[macro_export]
macro_rules! IterVariantNames {
    (
        @expand priv $itername:ident, $name:ident ($($var_names:ident),*)
    ) => {
        type $itername = ::std::vec::IntoIter<&'static str>;

        impl $name {
            #[allow(dead_code)]
            fn iter_variant_names() -> $itername {
                vec![$(stringify!($var_names)),*].into_iter()
            }
        }
    };

    (($itername:ident) $(pub)* enum $name:ident { $($body:tt)* }) => {
        enum_derive_util! {
            @collect_unitary_variants
            (IterVariantNames { @expand priv $itername, $name }),
            ($($body)*,) -> ()
        }
    };
}

#[macro_export]
macro_rules! IterVariantNames {
    (
        @expand ($($pub_:tt)*) $itername:ident, $name:ident ()
    ) => {
        enum_derive_util! { @as_item $($pub_)* struct $itername; }

        impl ::std::iter::Iterator for $itername {
            type Item = &'static str;
            fn next(&mut self) -> Option<Self::Item> {
                None
            }
        }

        enum_derive_util! {
            @as_item
            impl $name {
                #[allow(dead_code)]
                $($pub_)* fn iter_variant_names() -> $itername {
                    $itername
                }
            }
        }
    };

    (
        @expand ($($pub_:tt)*) $itername:ident, $name:ident ($($var_names:ident),*)
    ) => {
        enum_derive_util! { @as_item $($pub_)* struct $itername(Option<$name>); }

        IterVariantNames! { @iter ($itername, $name), ($($var_names,)*) -> () }

        enum_derive_util! {
            @as_item
            impl $name {
                #[allow(dead_code)]
                $($pub_)* fn iter_variant_names() -> $itername {
                    $itername(Some(enum_derive_util!(@first_expr $($name::$var_names),+)))
                }
            }
        }
    };

    (
        @iter ($itername:ident, $name:ident), () -> ($($body:tt)*)
    ) => {
        enum_derive_util! {
            @as_item
            impl ::std::iter::Iterator for $itername {
                type Item = &'static str;
                fn next(&mut self) -> Option<Self::Item> {
                    let (next_state, result) = match self.0 {
                        $($body)*
                        None => (None, None)
                    };
                    self.0 = next_state;
                    result
                }
            }
        }
    };

    (
        @iter ($itername:ident, $name:ident), ($a:ident, $b:ident, $($rest:tt)*) -> ($($body:tt)*)
    ) => {
        IterVariantNames! {
            @iter ($itername, $name), ($b, $($rest)*)
            -> (
                $($body)*
                Some($name::$a) => (Some($name::$b), Some(stringify!($a))),
            )
        }
    };

    (
        @iter ($itername:ident, $name:ident), ($a:ident,) -> ($($body:tt)*)
    ) => {
        IterVariantNames! {
            @iter ($itername, $name), ()
            -> (
                $($body)*
                Some($name::$a) => (None, Some(stringify!($a))),
            )
        }
    };

    (($itername:ident) pub enum $name:ident { $($body:tt)* }) => {
        enum_derive_util! {
            @collect_unitary_variants
            (IterVariantNames { @expand (pub) $itername, $name }),
            ($($body)*,) -> ()
        }
    };

    (($itername:ident) enum $name:ident { $($body:tt)* }) => {
        enum_derive_util! {
            @collect_unitary_variants
            (IterVariantNames { @expand () $itername, $name }),
            ($($body)*,) -> ()
        }
    };
}
