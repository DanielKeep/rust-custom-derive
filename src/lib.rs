/*
Copyright ⓒ 2016 macro-attr contributors.

Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
/*!
This crate provides a macro that enables the use of custom attributes backed by regular macros.

To use it, make sure you link to the crate like so:

```rust
#[macro_use] extern crate macro_attr;
# macro_rules! Dummy { (() struct $name:ident;) => {}; }
# macro_attr! { #[derive(Clone, Dummy!)] struct Foo; }
# fn main() { let _ = Foo; }
```

# Example

The `macro_attr!` macro should be used to wrap an entire *single* `enum` or `struct` declaration, including its attributes (both `derive` and others).  All attributes and derivations which whose names end with `!` will be assumed to be implemented by macros, and treated accordingly.

For example:

```rust
#[macro_use] extern crate macro_attr;

// Define some traits to be derived.

trait TypeName {
    fn type_name() -> &'static str;
}

trait ReprType {
    type Repr;
}

// Define macros which derive implementations of these macros.

macro_rules! TypeName {
    // We can support any kind of item we want.
    (() $(pub)* enum $name:ident $($tail:tt)*) => { TypeName! { @impl $name } };
    (() $(pub)* struct $name:ident $($tail:tt)*) => { TypeName! { @impl $name } };

    // Inner rule to cut down on repetition.
    (@impl $name:ident) => {
        impl TypeName for $name {
            fn type_name() -> &'static str { stringify!($name) }
        }
    };
}

macro_rules! TryFrom {
    // Note that we use a "derivation argument" here for the `$repr` type.
    (($repr:ty) $(pub)* enum $name:ident $($tail:tt)*) => {
        impl ReprType for $name {
            type Repr = $repr;
        }
    };
}

// Here is a macro that *modifies* the item.

macro_rules! rename_to {
    (
        ($new_name:ident),
        then $cb:ident!$cb_arg:tt,
        $(#[$($attrs:tt)*])*
        enum $_old_name:ident $($tail:tt)*
    ) => {
        macro_attr_callback! {
            $cb!$cb_arg,
            $(#[$($attrs)*])*
            enum $new_name $($tail)*
        }
    };
}

macro_attr! {
    #[allow(dead_code)]
    #[derive(Clone, Copy, Debug, TryFrom!(u8), TypeName!)]
    #[rename_to!(Bar)]
    #[repr(u8)]
    enum Foo { A, B }
}

fn main() {
    let bar = Bar::B;
    let v = bar as <Bar as ReprType>::Repr;
    let msg = format!("{}: {:?} ({:?})", Bar::type_name(), bar, v);
    assert_eq!(msg, "Bar: B (1)");
}
```
*/
#![cfg_attr(not(feature = "std"), no_std)]

/**
First, note that `macro_attr!` passes any arguments on the derivation attribute to the macro.  In the case of attributes *without* any arguments, `()` is passed instead.

Secondly, the macro is passed the entire item, *sans* attributes.  It is the derivation macro's job to parse the item correctly.

Third, each derivation macro is expected to result in zero or more items, not including the item itself.  As a result, it is *not* possible to mutate the item in any way, or attach additional attributes to it.

Finally, `@impl` is merely a trick to pack multiple, different functions into a single macro.  The sequence has no special meaning; it is simply *distinct* from the usual invocation syntax.
*/
#[macro_export]
macro_rules! macro_attr {
    ($($item:tt)*) => {
        macro_attr_impl! { $($item)* }
    };
}

/**
This macro exists as an implementation detail.  This is because if it *wasn't*, then the public-facing `macro_attr!` macro's documentation would be hideously unwieldy.
*/
#[doc(hidden)]
#[macro_export]
macro_rules! macro_attr_impl {
    /*

    > **Convention**: a capture named `$fixed` is used for any part of a recursive rule that is needed in the terminal case, but is not actually being *used* for the recursive part.  This avoids having to constantly repeat the full capture pattern (and makes changing it easier).

    # Primary Invocation Forms

    These need to catch any valid form of item.

    */
    (
        $(#[$($attrs:tt)*])*
        const $($it:tt)*
    ) => {
        macro_attr_impl! {
            @split_attrs
            ($(#[$($attrs)*],)*), (), (),
            (const $($it)*)
        }
    };

    (
        $(#[$($attrs:tt)*])*
        enum $($it:tt)*
    ) => {
        macro_attr_impl! {
            @split_attrs
            ($(#[$($attrs)*],)*), (), (),
            (enum $($it)*)
        }
    };

    (
        $(#[$($attrs:tt)*])*
        extern $($it:tt)*
    ) => {
        macro_attr_impl! {
            @split_attrs
            ($(#[$($attrs)*],)*), (), (),
            (extern $($it)*)
        }
    };

    (
        $(#[$($attrs:tt)*])*
        fn $($it:tt)*
    ) => {
        macro_attr_impl! {
            @split_attrs
            ($(#[$($attrs)*],)*), (), (),
            (fn $($it)*)
        }
    };

    (
        $(#[$($attrs:tt)*])*
        impl $($it:tt)*
    ) => {
        macro_attr_impl! {
            @split_attrs
            ($(#[$($attrs)*],)*), (), (),
            (impl $($it)*)
        }
    };

    (
        $(#[$($attrs:tt)*])*
        mod $($it:tt)*
    ) => {
        macro_attr_impl! {
            @split_attrs
            ($(#[$($attrs)*],)*), (), (),
            (mod $($it)*)
        }
    };

    (
        $(#[$($attrs:tt)*])*
        pub $($it:tt)*
    ) => {
        macro_attr_impl! {
            @split_attrs
            ($(#[$($attrs)*],)*), (), (),
            (pub $($it)*)
        }
    };

    (
        $(#[$($attrs:tt)*])*
        static $($it:tt)*
    ) => {
        macro_attr_impl! {
            @split_attrs
            ($(#[$($attrs)*],)*), (), (),
            (static $($it)*)
        }
    };

    (
        $(#[$($attrs:tt)*])*
        struct $($it:tt)*
    ) => {
        macro_attr_impl! {
            @split_attrs
            ($(#[$($attrs)*],)*), (), (),
            (struct $($it)*)
        }
    };

    (
        $(#[$($attrs:tt)*])*
        trait $($it:tt)*
    ) => {
        macro_attr_impl! {
            @split_attrs
            ($(#[$($attrs)*],)*), (), (),
            (trait $($it)*)
        }
    };

    (
        $(#[$($attrs:tt)*])*
        type $($it:tt)*
    ) => {
        macro_attr_impl! {
            @split_attrs
            ($(#[$($attrs)*],)*), (), (),
            (type $($it)*)
        }
    };

    (
        $(#[$($attrs:tt)*])*
        use $($it:tt)*
    ) => {
        macro_attr_impl! {
            @split_attrs
            ($(#[$($attrs)*],)*), (), (),
            (use $($it)*)
        }
    };

    /*

    # `@split_attrs`

    This is responsible for dividing all attributes on an item into two groups:

    - `#[derive(...)]`
    - Everything else.

    As part of this, it also explodes `#[derive(A, B(..), C, ...)]` into `A, B(..), C, ...`.  This is to simplify the next stage.

    */
    (
        @split_attrs
        (),
        $non_derives:tt,
        $derives:tt,
        $it:tt
    ) => {
        macro_attr_impl! {
            @split_derive_attrs
            { $non_derives, $it },
            $derives,
            (),
            ()
        }
    };

    (
        @split_attrs
        (#[derive($($new_drvs:tt)*)], $(#[$($attrs:tt)*],)*),
        $non_derives:tt,
        ($($derives:tt)*),
        $it:tt
    ) => {
        macro_attr_impl! {
            @split_attrs
            ($(#[$($attrs)*],)*),
            $non_derives,
            ($($derives)* $($new_drvs)*,),
            $it
        }
    };

    (
        @split_attrs
        (#[$mac_attr:ident!], $(#[$($attrs:tt)*],)*),
        $non_derives:tt,
        $derives:tt,
        ($($it:tt)*)
    ) => {
        $mac_attr! {
            (),
            then macro_attr_impl! {
                @split_attrs_resume
                $non_derives,
                $derives,
            },
            $(#[$($attrs)*])*
            $($it)*
        }
    };


    (
        @split_attrs
        (#[$mac_attr:ident!($($attr_args:tt)*)], $(#[$($attrs:tt)*],)*),
        $non_derives:tt,
        $derives:tt,
        ($($it:tt)*)
    ) => {
        $mac_attr! {
            ($($attr_args)*),
            then macro_attr_impl! {
                @split_attrs_resume
                $non_derives,
                $derives,
            },
            $(#[$($attrs)*])*
            $($it)*
        }
    };

    (
        @split_attrs
        (#[$mac_attr:ident~!], $(#[$($attrs:tt)*],)*),
        ($($non_derives:tt)*),
        $derives:tt,
        ($($it:tt)*)
    ) => {
        macro_attr_if_proc_macros! {
            proc_macros: {
                macro_attr_impl! {
                    @split_attrs
                    ($(#[$($attrs)*],)*),
                    ($($non_derives)* #[$mac_attr],),
                    $derives,
                    $($it)*
                }
            }
            fallback: {
                $mac_attr! {
                    (),
                    then macro_attr_impl! {
                        @split_attrs_resume
                        ($($non_derives)*),
                        $derives,
                    },
                    $(#[$($attrs)*])*
                    $($it)*
                }
            }
        }
    };

    (
        @split_attrs
        (#[$mac_attr:ident~!($($attr_args:tt)*)], $(#[$($attrs:tt)*],)*),
        ($($non_derives:tt)*),
        $derives:tt,
        ($($it:tt)*)
    ) => {
        macro_attr_if_proc_macros! {
            proc_macros: {
                macro_attr_impl! {
                    @split_attrs
                    ($(#[$($attrs)*],)*),
                    ($($non_derives)* #[$mac_attr($($attr_args)*)],),
                    $derives,
                    $($it)*
                }
            }
            fallback: {
                $mac_attr! {
                    ($($attr_args)*),
                    then macro_attr_impl! {
                        @split_attrs_resume
                        ($($non_derives)*),
                        $derives,
                    },
                    $(#[$($attrs)*])*
                    $($it)*
                }
            }
        }
    };

    (
        @split_attrs
        (#[$new_attr:meta], $(#[$($attrs:tt)*],)*),
        ($($non_derives:tt)*),
        $derives:tt,
        $it:tt
    ) => {
        macro_attr_impl! {
            @split_attrs
            ($(#[$($attrs)*],)*),
            ($($non_derives)* #[$new_attr],),
            $derives,
            $it
        }
    };


    /*

    # `@split_attrs_resume`

    Callback used to re-enter this macro after running a macro attribute.

    */

    (
        @split_attrs_resume
        $non_derives:tt,
        $derives:tt,
        $(#[$($attrs:tt)*])*
        const $($it:tt)*
    ) => {
        macro_attr_impl! {
            @split_attrs
            ($(#[$($attrs)*],)*),
            $non_derives,
            $derives,
            (const $($it)*)
        }
    };

    (
        @split_attrs_resume
        $non_derives:tt,
        $derives:tt,
        $(#[$($attrs:tt)*])*
        enum $($it:tt)*
    ) => {
        macro_attr_impl! {
            @split_attrs
            ($(#[$($attrs)*],)*),
            $non_derives,
            $derives,
            (enum $($it)*)
        }
    };

    (
        @split_attrs_resume
        $non_derives:tt,
        $derives:tt,
        $(#[$($attrs:tt)*])*
        extern $($it:tt)*
    ) => {
        macro_attr_impl! {
            @split_attrs
            ($(#[$($attrs)*],)*),
            $non_derives,
            $derives,
            (extern $($it)*)
        }
    };

    (
        @split_attrs_resume
        $non_derives:tt,
        $derives:tt,
        $(#[$($attrs:tt)*])*
        fn $($it:tt)*
    ) => {
        macro_attr_impl! {
            @split_attrs
            ($(#[$($attrs)*],)*),
            $non_derives,
            $derives,
            (fn $($it)*)
        }
    };

    (
        @split_attrs_resume
        $non_derives:tt,
        $derives:tt,
        $(#[$($attrs:tt)*])*
        impl $($it:tt)*
    ) => {
        macro_attr_impl! {
            @split_attrs
            ($(#[$($attrs)*],)*),
            $non_derives,
            $derives,
            (impl $($it)*)
        }
    };

    (
        @split_attrs_resume
        $non_derives:tt,
        $derives:tt,
        $(#[$($attrs:tt)*])*
        mod $($it:tt)*
    ) => {
        macro_attr_impl! {
            @split_attrs
            ($(#[$($attrs)*],)*),
            $non_derives,
            $derives,
            (mod $($it)*)
        }
    };

    (
        @split_attrs_resume
        $non_derives:tt,
        $derives:tt,
        $(#[$($attrs:tt)*])*
        pub $($it:tt)*
    ) => {
        macro_attr_impl! {
            @split_attrs
            ($(#[$($attrs)*],)*),
            $non_derives,
            $derives,
            (pub $($it)*)
        }
    };

    (
        @split_attrs_resume
        $non_derives:tt,
        $derives:tt,
        $(#[$($attrs:tt)*])*
        static $($it:tt)*
    ) => {
        macro_attr_impl! {
            @split_attrs
            ($(#[$($attrs)*],)*),
            $non_derives,
            $derives,
            (static $($it)*)
        }
    };

    (
        @split_attrs_resume
        $non_derives:tt,
        $derives:tt,
        $(#[$($attrs:tt)*])*
        struct $($it:tt)*
    ) => {
        macro_attr_impl! {
            @split_attrs
            ($(#[$($attrs)*],)*),
            $non_derives,
            $derives,
            (struct $($it)*)
        }
    };

    (
        @split_attrs_resume
        $non_derives:tt,
        $derives:tt,
        $(#[$($attrs:tt)*])*
        trait $($it:tt)*
    ) => {
        macro_attr_impl! {
            @split_attrs
            ($(#[$($attrs)*],)*),
            $non_derives,
            $derives,
            (trait $($it)*)
        }
    };

    (
        @split_attrs_resume
        $non_derives:tt,
        $derives:tt,
        $(#[$($attrs:tt)*])*
        type $($it:tt)*
    ) => {
        macro_attr_impl! {
            @split_attrs
            ($(#[$($attrs)*],)*),
            $non_derives,
            $derives,
            (type $($it)*)
        }
    };

    (
        @split_attrs_resume
        $non_derives:tt,
        $derives:tt,
        $(#[$($attrs:tt)*])*
        use $($it:tt)*
    ) => {
        macro_attr_impl! {
            @split_attrs
            ($(#[$($attrs)*],)*),
            $non_derives,
            $derives,
            (use $($it)*)
        }
    };


    /*

    # `@split_derive_attrs`

    This is responsible for taking the list of derivation attributes and splitting them into "built-in" and "custom" groups.

    A custom attribute is any which has a `!` after the name, like a macro.
    */

    (@split_derive_attrs
        { ($(#[$($non_derives:tt)*],)*), ($($it:tt)*) },
        ($(,)*), (), ($($user_drvs:tt)*)
    ) => {
        macro_attr_impl! {
            @as_item
            $(#[$($non_derives)*])*
            $($it)*
        }

        macro_attr_impl! {
            @expand_user_drvs
            ($($user_drvs)*), ($($it)*)
        }
    };

    (@split_derive_attrs
        { ($(#[$($non_derives:tt)*],)*), ($($it:tt)*) },
        ($(,)*), ($($bi_drvs:ident,)+), ($($user_drvs:tt)*)
    ) => {
        macro_attr_impl! {
            @as_item
            #[derive($($bi_drvs,)+)]
            $(#[$($non_derives)*])*
            $($it)*
        }

        macro_attr_impl! {
            @expand_user_drvs
            ($($user_drvs)*), ($($it)*)
        }
    };

    (@split_derive_attrs
        $fixed:tt,
        (,, $($tail:tt)*), $bi_drvs:tt, $user_drvs:tt
    ) => {
        macro_attr_impl! {
            @split_derive_attrs
            $fixed, ($($tail)*), $bi_drvs, $user_drvs
        }
    };

    (@split_derive_attrs
        $fixed:tt,
        (, $($tail:tt)*), $bi_drvs:tt, $user_drvs:tt
    ) => {
        macro_attr_impl! {
            @split_derive_attrs
            $fixed, ($($tail)*), $bi_drvs, $user_drvs
        }
    };

    /*

    ## Custom Derivations

    Now we can handle the custom derivations.  There are two forms we care about: those *with* an argument, and those *without*.

    The *reason* we care is that, in order to simplify the derivation macros, we want to detect the argument-less case and generate an empty pair of parens.

    */
    (@split_derive_attrs
        $fixed:tt,
        ($new_user:ident ! ($($new_user_args:tt)*), $($tail:tt)*), $bi_drvs:tt, ($($user_drvs:tt)*)
    ) => {
        macro_attr_impl! {
            @split_derive_attrs
            $fixed, ($($tail)*), $bi_drvs, ($($user_drvs)* $new_user($($new_user_args)*),)
        }
    };

    (@split_derive_attrs
        $fixed:tt,
        ($new_user:ident !, $($tail:tt)*), $bi_drvs:tt, ($($user_drvs:tt)*)
    ) => {
        macro_attr_impl! {
            @split_derive_attrs
            $fixed, ($($tail)*), $bi_drvs, ($($user_drvs)* $new_user(),)
        }
    };

    /*

    ## Hybrid Derivations

    These are derivations that use regular macros *or* procedural macros, depending on the version of Rust in use.

    */
    (@split_derive_attrs
        $fixed:tt,
        ($new_drv:ident ~!, $($tail:tt)*), ($($bi_drvs:ident,)*), ($($user_drvs:tt)*)
    ) => {
        macro_attr_if_proc_macros! {
            proc_macros: {
                macro_attr_impl! {
                    @split_derive_attrs
                    $fixed,
                    ($($tail)*),
                    ($($bi_drvs,)* $new_drv,),
                    ($($user_drvs)*)
                }
            }
            fallback: {
                macro_attr_impl! {
                    @split_derive_attrs
                    $fixed,
                    ($($tail)*),
                    ($($bi_drvs,)*),
                    ($($user_drvs)* $new_drv(),)
                }
            }
        }
    };

    /*

    ## Non-Macro Derivations

    All the rest.

    */
    (@split_derive_attrs
        $fixed:tt,
        ($drv:ident, $($tail:tt)*), ($($bi_drvs:ident,)*), $user_drvs:tt
    ) => {
        macro_attr_impl! {
            @split_derive_attrs
            $fixed,
            ($($tail)*), ($($bi_drvs,)* $drv,), $user_drvs
        }
    };

    /*

    # `@expand_user_drvs`

    Finally, we have a recursive rule for expanding user derivations.  This is basically just using the derivation name as a macro identifier.

    This *has* to be recursive because we need to expand two independent repetition sequences simultaneously, and this causes `macro_rules!` to throw a wobbly.  Don't want that.  So, recursive it is.

    */
    (@expand_user_drvs
        (), ($($it:tt)*)
    ) => {};

    (@expand_user_drvs
        ($user_drv:ident $arg:tt, $($tail:tt)*), ($($it:tt)*)
    ) => {
        $user_drv! { $arg $($it)* }
        macro_attr_impl! {
            @expand_user_drvs
            ($($tail)*), ($($it)*)
        }
    };

    /*

    # Miscellaneous Rules

    */
    (@as_item $($i:item)*) => {$($i)*};
}

/**
This macro invokes a "callback" macro, merging arguments together.

Essentially, it takes an arbitrary macro call `name!(args...)`, plus some sequence of `new_args...`, and expands `name!(args... new_args...)`.

Importantly, it works irrespective of the kind of grouping syntax used for the macro arguments, simplifying macros which need to *capture* callbacks.
*/
#[macro_export]
macro_rules! macro_attr_callback {
    (
        $cb:ident ! { $($cb_fixed:tt)* },
        $($args:tt)*
    ) => {
        $cb! { $($cb_fixed)* $($args)* }
    };

    (
        $cb:ident ! [ $($cb_fixed:tt)* ],
        $($args:tt)*
    ) => {
        $cb! [ $($cb_fixed)* $($args)* ]
    };

    (
        $cb:ident ! ( $($cb_fixed:tt)* ),
        $($args:tt)*
    ) => {
        $cb! ( $($cb_fixed)* $($args)* )
    };
}

/**
This macro provides a simple way to select between two branches of code, depending on whether or not support for procedural macros is enabled or not.
*/
#[macro_export]
#[cfg(feature="unstable-macros-1-1")]
macro_rules! macro_attr_if_proc_macros {
    (
        proc_macros: { $($items:item)* }
        fallback: $_ignore:tt
    ) => {
        $($items)*
    };
}

/**
This macro provides a simple way to select between two branches of code, depending on whether or not support for procedural macros is enabled or not.
*/
#[macro_export]
#[cfg(not(feature="unstable-macros-1-1"))]
macro_rules! macro_attr_if_proc_macros {
    (
        proc_macros: $_ignore:tt
        fallback: { $($items:item)* }
    ) => {
        $($items)*
    };
}
