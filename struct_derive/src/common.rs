#[doc(hidden)]
#[macro_export]
macro_rules! struct_derive_util {
    (@as_item $($i:item)*) => { $($i)* };

    (
        @call
        ($cb:ident!($($cb_arg:tt)*)),
        $($output:tt)*
    ) => {
        $cb!(
            $($cb_arg)*
            $($output)*
        )
    };

    (
        @call
        ($cb:ident!{$($cb_arg:tt)*}),
        $($output:tt)*
    ) => {
        $cb! {
            $($cb_arg)*
            $($output)*
        }
    };

    (@inc_ord_ident $cb:tt,  0) => { struct_derive_util!{ @call $cb,  1, _ord_01 } };
    (@inc_ord_ident $cb:tt,  1) => { struct_derive_util!{ @call $cb,  2, _ord_02 } };
    (@inc_ord_ident $cb:tt,  2) => { struct_derive_util!{ @call $cb,  3, _ord_03 } };
    (@inc_ord_ident $cb:tt,  3) => { struct_derive_util!{ @call $cb,  4, _ord_04 } };
    (@inc_ord_ident $cb:tt,  4) => { struct_derive_util!{ @call $cb,  5, _ord_05 } };
    (@inc_ord_ident $cb:tt,  5) => { struct_derive_util!{ @call $cb,  6, _ord_06 } };
    (@inc_ord_ident $cb:tt,  6) => { struct_derive_util!{ @call $cb,  7, _ord_07 } };
    (@inc_ord_ident $cb:tt,  7) => { struct_derive_util!{ @call $cb,  8, _ord_08 } };
    (@inc_ord_ident $cb:tt,  8) => { struct_derive_util!{ @call $cb,  9, _ord_09 } };
    (@inc_ord_ident $cb:tt,  9) => { struct_derive_util!{ @call $cb, 10, _ord_10 } };
    (@inc_ord_ident $cb:tt, 10) => { struct_derive_util!{ @call $cb, 11, _ord_11 } };
    (@inc_ord_ident $cb:tt, 11) => { struct_derive_util!{ @call $cb, 12, _ord_12 } };
    (@inc_ord_ident $cb:tt, 12) => { struct_derive_util!{ @call $cb, 13, _ord_13 } };
    (@inc_ord_ident $cb:tt, 13) => { struct_derive_util!{ @call $cb, 14, _ord_14 } };
    (@inc_ord_ident $cb:tt, 14) => { struct_derive_util!{ @call $cb, 15, _ord_15 } };
    (@inc_ord_ident $cb:tt, 15) => { struct_derive_util!{ @call $cb, 16, _ord_16 } };
    (@inc_ord_ident $cb:tt, 16) => { struct_derive_util!{ @call $cb, 17, _ord_17 } };
    (@inc_ord_ident $cb:tt, 17) => { struct_derive_util!{ @call $cb, 18, _ord_18 } };
    (@inc_ord_ident $cb:tt, 18) => { struct_derive_util!{ @call $cb, 19, _ord_19 } };
    (@inc_ord_ident $cb:tt, 19) => { struct_derive_util!{ @call $cb, 20, _ord_20 } };
    (@inc_ord_ident $cb:tt, 20) => { struct_derive_util!{ @call $cb, 21, _ord_21 } };
    (@inc_ord_ident $cb:tt, 21) => { struct_derive_util!{ @call $cb, 22, _ord_22 } };
    (@inc_ord_ident $cb:tt, 22) => { struct_derive_util!{ @call $cb, 23, _ord_23 } };
    (@inc_ord_ident $cb:tt, 23) => { struct_derive_util!{ @call $cb, 24, _ord_24 } };
    (@inc_ord_ident $cb:tt, 24) => { struct_derive_util!{ @call $cb, 25, _ord_25 } };
    (@inc_ord_ident $cb:tt, 25) => { struct_derive_util!{ @call $cb, 26, _ord_26 } };
    (@inc_ord_ident $cb:tt, 26) => { struct_derive_util!{ @call $cb, 27, _ord_27 } };
    (@inc_ord_ident $cb:tt, 27) => { struct_derive_util!{ @call $cb, 28, _ord_28 } };
    (@inc_ord_ident $cb:tt, 28) => { struct_derive_util!{ @call $cb, 29, _ord_29 } };
    (@inc_ord_ident $cb:tt, 29) => { struct_derive_util!{ @call $cb, 30, _ord_30 } };
    (@inc_ord_ident $cb:tt, 30) => { struct_derive_util!{ @call $cb, 31, _ord_31 } };
    (@inc_ord_ident $cb:tt, 31) => { struct_derive_util!{ @call $cb, 32, _ord_32 } };
}

#[doc(hidden)]
#[macro_export]
macro_rules! struct_derive_parse_struct {
    (
        then $cb:ident!$cb_arg:tt,
        $(#[$($attrs:tt)*])* pub struct $name:ident $($tail:tt)*
    ) => {
        struct_derive_parse_struct! {
            @with_where
            (($cb!$cb_arg), [$(#[$($attrs)*])*], (pub), $name),
            { constr: [], params: [], ltimes: [], tnames: [], .. },
            { clause: [], preds: [], .. },
            $($tail)*
        }
    };

    (
        then $cb:ident!$cb_arg:tt,
        $(#[$($attrs:tt)*])* struct $name:ident $($tail:tt)*
    ) => {
        struct_derive_parse_struct! {
            @with_where
            (($cb!$cb_arg), [$(#[$($attrs)*])*], (), $name),
            { constr: [], params: [], ltimes: [], tnames: [], .. },
            { clause: [], preds: [], .. },
            $($tail)*
        }
    };

    (
        @with_where
        ($cb:tt, $attrs:tt, $vis:tt, $name:ident),
        $generics:tt,
        $where_:tt,
        ;
    ) => {
        struct_derive_util! {
            @call $cb,
            struct {
                attrs: $attrs,
                vis: $vis,
                name: $name,
                generics: $generics,
                where: $where_,
                kind: unitary,
                fields: [],
                num_fields: 0,
            }
        }
    };

    (
        @with_where
        $prefix:tt,
        $generics:tt,
        $where_:tt,
        ; ($($body:tt)*)
    ) => {
        struct_derive_parse_struct! {
            @parse_fields
            ($prefix, $generics, $where_),
            [],
            [],
            ($($body)*,),
            0, _ord_00
        }
    };

    (
        @with_where
        $prefix:tt,
        $generics:tt,
        $where_:tt,
        {$($body:tt)*}
    ) => {
        struct_derive_parse_struct! {
            @parse_fields
            ($prefix, $generics, $where_),
            [],
            [],
            {$($body)*,},
            0, _ord_00
        }
    };

    (
        @parse_fields
        (
            (
                $cb:tt,
                $attrs:tt,
                $vis:tt,
                $name:ident
            ),
            $generics:tt,
            $where_:tt
        ),
        $fields:tt,
        $_attrs:tt,
        ($(,)*),
        $ord:tt, $_ord_ident:tt
    ) => {
        struct_derive_util! {
            @call $cb,
            struct {
                attrs: $attrs,
                vis: $vis,
                name: $name,
                generics: $generics,
                where: $where_,
                kind: tuple,
                fields: $fields,
                num_fields: $ord,
            }
        }
    };

    (
        @parse_fields
        $prefix2:tt,
        $fields:tt,
        [$($attrs:tt)*],
        (#[$($attr:tt)*] $($tail:tt)*),
        $ord:tt, $ord_ident:tt
    ) => {
        struct_derive_parse_struct! {
            @parse_fields
            $prefix2,
            $fields,
            [$($attrs)* #[$($attr)*]],
            ($($tail)*),
            $ord, $ord_ident
        }
    };

    (
        @parse_fields
        $prefix2:tt,
        [$($fields:tt)*],
        $attrs:tt,
        (pub $fty:ty, $($tail:tt)*),
        $ord:tt, $ord_ident:tt
    ) => {
        struct_derive_util! {
            @inc_ord_ident
            (struct_derive_parse_struct! {
                @parse_fields
                $prefix2,
                [
                    $($fields)*
                    {
                        ord: ($ord, $ord_ident),
                        attrs: $attrs,
                        vis: (pub),
                        ty: $fty,
                    },
                ],
                [],
                ($($tail)*),
            }),
            $ord
        }
    };

    (
        @parse_fields
        $prefix2:tt,
        [$($fields:tt)*],
        $attrs:tt,
        ($fty:ty, $($tail:tt)*),
        $ord:tt, $ord_ident:tt
    ) => {
        struct_derive_util! {
            @inc_ord_ident
            (struct_derive_parse_struct! {
                @parse_fields
                $prefix2,
                [
                    $($fields)*
                    {
                        ord: ($ord, $ord_ident),
                        attrs: $attrs,
                        vis: (),
                        ty: $fty,
                    },
                ],
                [],
                ($($tail)*),
            }),
            $ord
        }
    };

    (
        @parse_fields
        (
            (
                $cb:tt,
                $attrs:tt,
                $vis:tt,
                $name:ident
            ),
            $generics:tt,
            $where_:tt
        ),
        $fields:tt,
        $_attrs:tt,
        { $(,)* },
        $ord:tt, $_ord_ident:tt
    ) => {
        struct_derive_util! {
            @call $cb,
            struct {
                attrs: $attrs,
                vis: $vis,
                name: $name,
                generics: $generics,
                where: $where_,
                kind: record,
                fields: $fields,
                num_fields: $ord,
            }
        }
    };

    (
        @parse_fields
        $prefix2:tt,
        $fields:tt,
        [$($attrs:tt)*],
        { #[$($attr:tt)*] $($tail:tt)* },
        $ord:tt, $ord_ident:tt
    ) => {
        struct_derive_parse_struct! {
            @parse_fields
            $prefix2,
            $fields,
            [$($attrs)* #[$($attr)*]],
            { $($tail)* },
            $ord, $ord_ident
        }
    };

    (
        @parse_fields
        $prefix2:tt,
        [$($fields:tt)*],
        $attrs:tt,
        { pub $fname:ident: $fty:ty, $($tail:tt)* },
        $ord:tt, $ord_ident:tt
    ) => {
        struct_derive_util! {
            @inc_ord_ident
            (struct_derive_parse_struct! {
                @parse_fields
                $prefix2,
                [
                    $($fields)*
                    {
                        ord: ($ord, $ord_ident),
                        attrs: $attrs,
                        vis: (pub),
                        ty: $fty,
                        name: $fname,
                    },
                ],
                [],
                {$($tail)*},
            }),
            $ord
        }
    };

    (
        @parse_fields
        $prefix2:tt,
        [$($fields:tt)*],
        $attrs:tt,
        { $fname:ident: $fty:ty, $($tail:tt)* },
        $ord:tt, $ord_ident:tt
    ) => {
        struct_derive_util! {
            @inc_ord_ident
            (struct_derive_parse_struct! {
                @parse_fields
                $prefix2,
                [
                    $($fields)*
                    {
                        ord: ($ord, $ord_ident),
                        attrs: $attrs,
                        vis: (),
                        ty: $fty,
                        name: $fname,
                    },
                ],
                [],
                {$($tail)*},
            }),
            $ord
        }
    };
}
