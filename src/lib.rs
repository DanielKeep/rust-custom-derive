/*!
Provides a very bare-bones way of having custom `derive` support in stable Rust.
*/
#[macro_export]
macro_rules! custom_derive {
    (
        $(#[$($attrs:tt)*])*
        enum $($it:tt)*
    ) => {
        custom_derive! {
            @split_attrs
            ($(#[$($attrs)*],)*), (), (),
            (enum $($it)*)
        }
    };

    (
        $(#[$($attrs:tt)*])*
        pub $($it:tt)*
    ) => {
        custom_derive! {
            @split_attrs
            ($(#[$($attrs)*],)*), (), (),
            (pub $($it)*)
        }
    };

    (
        $(#[$($attrs:tt)*])*
        struct $($it:tt)*
    ) => {
        custom_derive! {
            @split_attrs
            ($(#[$($attrs)*],)*), (), (),
            (struct $($it)*)
        }
    };

    (@as_item $($i:item)*) => {$($i)*};

    (
        @split_attrs
        (),
        $non_derives:tt,
        $derives:tt,
        $it:tt
    ) => {
        custom_derive! {
            @split_derive_attrs
            { $non_derives, $it },
            $derives,
            (),
            ()
        }
    };

    (
        @split_attrs
        (#[derive($($new_drv:ident $(($($new_drv_args:tt)*))*),*)], $(#[$($attrs:tt)*],)*),
        $non_derives:tt,
        ($($derives:ident,)*),
        $it:tt
    ) => {
        custom_derive! {
            @split_attrs
            ($(#[$($attrs)*],)*),
            $non_derives,
            ($($derives,)* $($new_drv $(($($new_drv_args)*))*,)*),
            $it
        }
    };

    (
        @split_attrs
        (#[$new_attr:meta], $(#[$($attrs:tt)*],)*),
        ($($non_derives:tt)*),
        $derives:tt,
        $it:tt
    ) => {
        custom_derive! {
            @split_attrs
            ($(#[$($attrs)*],)*),
            ($($non_derives)* #[$new_attr],),
            $derives,
            $it
        }
    };

    /*
    Built-in derives:
        Clone, Hash, RustcEncodable, RustcDecodable, PartialEq, Eq, PartialOrd, Ord,
        Debug, Default, Send, Sync, Copy,
    */

    (@split_derive_attrs
        { ($(#[$($non_derives:tt)*],)*), ($($it:tt)*) },
        (), ($($bi_drvs:ident,)*), ($($user_drvs:tt)*)
    ) => {
        custom_derive! {
            @as_item
            #[derive($($bi_drvs,)*)]
            $(#[$($non_derives)*])*
            $($it)*
        }

        custom_derive! {
            @expand_user_drvs
            ($($user_drvs)*), ($($it)*)
        }
    };

    (@split_derive_attrs
        $fixed:tt,
        (Clone, $($tail:tt)*), ($($bi_drvs:ident,)*), $user_drvs:tt
    ) => {
        custom_derive! {
            @split_derive_attrs
            $fixed,
            ($($tail)*), ($($bi_drvs,)* Clone,), $user_drvs
        }
    };

    (@split_derive_attrs
        $fixed:tt,
        (RustcEncodable, $($tail:tt)*), ($($bi_drvs:ident,)*), $user_drvs:tt
    ) => {
        custom_derive! {
            @split_derive_attrs
            $fixed,
            ($($tail)*), ($($bi_drvs,)* RustcEncodable,), $user_drvs
        }
    };

    (@split_derive_attrs
        $fixed:tt,
        (RustcDecodable, $($tail:tt)*), ($($bi_drvs:ident,)*), $user_drvs:tt
    ) => {
        custom_derive! {
            @split_derive_attrs
            $fixed,
            ($($tail)*), ($($bi_drvs,)* RustcDecodable,), $user_drvs
        }
    };

    (@split_derive_attrs
        $fixed:tt,
        (PartialEq, $($tail:tt)*), ($($bi_drvs:ident,)*), $user_drvs:tt
    ) => {
        custom_derive! {
            @split_derive_attrs
            $fixed,
            ($($tail)*), ($($bi_drvs,)* PartialEq,), $user_drvs
        }
    };

    (@split_derive_attrs
        $fixed:tt,
        (Eq, $($tail:tt)*), ($($bi_drvs:ident,)*), $user_drvs:tt
    ) => {
        custom_derive! {
            @split_derive_attrs
            $fixed,
            ($($tail)*), ($($bi_drvs,)* Eq,), $user_drvs
        }
    };

    (@split_derive_attrs
        $fixed:tt,
        (PartialOrd, $($tail:tt)*), ($($bi_drvs:ident,)*), $user_drvs:tt
    ) => {
        custom_derive! {
            @split_derive_attrs
            $fixed,
            ($($tail)*), ($($bi_drvs,)* PartialOrd,), $user_drvs
        }
    };

    (@split_derive_attrs
        $fixed:tt,
        (Ord, $($tail:tt)*), ($($bi_drvs:ident,)*), $user_drvs:tt
    ) => {
        custom_derive! {
            @split_derive_attrs
            $fixed,
            ($($tail)*), ($($bi_drvs,)* Ord,), $user_drvs
        }
    };

    (@split_derive_attrs
        $fixed:tt,
        (Debug, $($tail:tt)*), ($($bi_drvs:ident,)*), $user_drvs:tt
    ) => {
        custom_derive! {
            @split_derive_attrs
            $fixed,
            ($($tail)*), ($($bi_drvs,)* Debug,), $user_drvs
        }
    };

    (@split_derive_attrs
        $fixed:tt,
        (Default, $($tail:tt)*), ($($bi_drvs:ident,)*), $user_drvs:tt
    ) => {
        custom_derive! {
            @split_derive_attrs
            $fixed,
            ($($tail)*), ($($bi_drvs,)* Default,), $user_drvs
        }
    };

    (@split_derive_attrs
        $fixed:tt,
        (Send ,$($tail:tt)*), ($($bi_drvs:ident,)*), $user_drvs:tt
    ) => {
        custom_derive! {
            @split_derive_attrs
            $fixed,
            ($($tail)*), ($($bi_drvs,)* Send,), $user_drvs
        }
    };

    (@split_derive_attrs
        $fixed:tt,
        (Sync, $($tail:tt)*), ($($bi_drvs:ident,)*), $user_drvs:tt
    ) => {
        custom_derive! {
            @split_derive_attrs
            $fixed,
            ($($tail)*), ($($bi_drvs,)* Sync,), $user_drvs
        }
    };

    (@split_derive_attrs
        $fixed:tt,
        (Copy, $($tail:tt)*), ($($bi_drvs:ident,)*), $user_drvs:tt
    ) => {
        custom_derive! {
            @split_derive_attrs
            $fixed,
            ($($tail)*), ($($bi_drvs,)* Copy,), $user_drvs
        }
    };

    (@split_derive_attrs
        $fixed:tt,
        ($new_user:ident, $($tail:tt)*), $bi_drvs:tt, ($($user_drvs:tt)*)
    ) => {
        custom_derive! {
            @split_derive_attrs
            $fixed, ($($tail)*), $bi_drvs, ($($user_drvs)* $new_user(),)
        }
    };

    (@split_derive_attrs
        $fixed:tt,
        ($new_user:ident ($($new_user_args:tt)*), $($tail:tt)*), $bi_drvs:tt, ($($user_drvs:tt)*)
    ) => {
        custom_derive! {
            @split_derive_attrs
            $fixed, ($($tail)*), $bi_drvs, ($($user_drvs)* $new_user($($new_user_args)*),)
        }
    };

    (@expand_user_drvs
        (), ($($it:tt)*)
    ) => {};

    (@expand_user_drvs
        ($user_drv:ident $arg:tt, $($tail:tt)*), ($($it:tt)*)
    ) => {
        $user_drv! { $arg $($it)* }
        custom_derive! {
            @expand_user_drvs
            ($($tail)*), ($($it)*)
        }
    };
}
