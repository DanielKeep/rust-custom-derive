/*
Copyright ⓒ 2017 macro-attr contributors.

Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
#[doc(hidden)
#[macro_export]
macro_rules! enum_derive_util {
    (@as_expr $e:expr) => {$e};
    (@as_item $($i:item)+) => {$($i)+};
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
        enum_derive_util! {
            @error item
            "enum_derive: cannot parse unitary variants from enum with non-unitary variants."
        }
    };

    (
        @collect_unary_variants ($callback:ident { $($args:tt)* }),
        ($(,)*) -> ($($out:tt)*)
    ) => {
        enum_derive_util! {
            @as_item
            $callback!{ $($args)* ($($out)*) }
        }
    };

    (
        @collect_unary_variants $fixed:tt,
        (#[$_attr:meta] $($tail:tt)*) -> ($($out:tt)*)
    ) => {
        enum_derive_util! {
            @collect_unary_variants $fixed,
            ($($tail)*) -> ($($out)*)
        }
    };

    (
        @collect_unary_variants $fixed:tt,
        ($var_name:ident($var_ty:ty), $($tail:tt)*) -> ($($out:tt)*)
    ) => {
        enum_derive_util! {
            @collect_unary_variants $fixed,
            ($($tail)*) -> ($($out)* $var_name($var_ty),)
        }
    };

    (
        @collect_unary_variants $fixed:tt,
        ($var_name:ident(pub $var_ty:ty), $($tail:tt)*) -> ($($out:tt)*)
    ) => {
        enum_derive_util! {
            @collect_unary_variants $fixed,
            ($($tail)*) -> ($($out)* $var_name($var_ty),)
        }
    };

    (
        @collect_unary_variants ($name:ident),
        ($var:ident $_struct:tt, $($tail:tt)*) -> ($($_out:tt)*)
    ) => {
        enum_derive_util! {
            @error item
            "enum_derive: cannot parse unary variants from enum with non-unary tuple variants."
        }
    };

    (
        @collect_variant_names ($callback:ident { $($args:tt)* }),
        ($(,)*) -> ($($out:tt)*)
    ) => {
        enum_derive_util! {
            @as_item
            $callback!{ $($args)* ($($out)*) }
        }
    };

    (
        @collect_variant_names $fixed:tt,
        (#[$_attr:meta] $($tail:tt)*) -> ($($out:tt)*)
    ) => {
        enum_derive_util! {
            @collect_variant_names $fixed,
            ($($tail)*) -> ($($out)*)
        }
    };

    (
        @collect_variant_names $fixed:tt,
        ($var_name:ident, $($tail:tt)*) -> ($($out:tt)*)
    ) => {
        enum_derive_util! {
            @collect_variant_names $fixed,
            ($($tail)*) -> ($($out)* $var_name,)
        }
    };

    (
        @collect_variant_names $fixed:tt,
        ($var_name:ident($($_var_tuple:tt)*), $($tail:tt)*) -> ($($out:tt)*)
    ) => {
        enum_derive_util! {
            @collect_variant_names $fixed,
            ($($tail)*) -> ($($out)* $var_name,)
        }
    };

    (
        @collect_variant_names $fixed:tt,
        ($var_name:ident { $($_var_struct:tt)* }, $($tail:tt)*) -> ($($out:tt)*)
    ) => {
        enum_derive_util! {
            @collect_variant_names $fixed,
            ($($tail)*) -> ($($out)* $var_name,)
        }
    };

    (@error item $msg:tt) => {
        const $msg: () = ();
    };
}
