/*
Copyright ⓒ 2016 macro-attr contributors.

Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
/*!
This crate provides the `macro_attr!` macro that enables the use of custom, macro-based attributes and derivations.  Supercedes the `custom_derive` crate.

<style type="text/css">
.link-block { font-family: "Fira Sans"; }
.link-block > p { display: inline-block; }
.link-block > p > strong { font-weight: 500; margin-right: 1em; }
.link-block > ul { display: inline-block; padding: 0; list-style: none; }
.link-block > ul > li {
  font-size: 0.8em;
  background-color: #eee;
  border: 1px solid #ccc;
  padding: 0.3em;
  display: inline-block;
}
</style>
<span></span><div class="link-block">

**Links**

* [Latest Release](https://crates.io/crates/macro-attr/)
* [Latest Docs](https://docs.rs/crate/macro-attr/)
* [Repository](https://github.com/DanielKeep/rust-custom-derive)

<span></span></div>

## Compatibility

`macro-attr` is compatible with Rust 1.2 and higher.

## Quick Example

To use it, make sure you link to the crate like so:

```rust
#[macro_use] extern crate macro_attr;
# macro_rules! Dummy { (() struct $name:ident;) => {}; }
# macro_attr! { #[derive(Clone, Dummy!)] struct Foo; }
# fn main() { let _ = Foo; }
```

The `macro_attr!` macro should be used to wrap an entire *single* item (`enum`, `struct`, *etc.*) declaration, including its attributes (both `derive` and others).  All attributes and derivations which whose names end with `!` will be assumed to be implemented by macros, and treated accordingly.

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

macro_rules! ReprType {
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
        then $cb:tt,
        $(#[$($attrs:tt)*])*
        enum $_old_name:ident $($tail:tt)*
    ) => {
        macro_attr_callback! {
            $cb,
            $(#[$($attrs)*])*
            enum $new_name $($tail)*
        }
    };
}

macro_attr! {
    #[allow(dead_code)]
    #[derive(Clone, Copy, Debug, ReprType!(u8), TypeName!)]
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
When given an item definition, including its attributes, this macro parses said attributes and dispatches any attributes or derivations suffixed with `!` to user-defined macros.  This allows multiple macros to process the same item.

This is similar to, but distinct from, the function of "procedural" macros and compiler plugins.

# Supported Forms

In particular, this macro looks for two kinds of syntax:

- Derivations such as the `Name` in `#[derive(Name!)]` or `#[derive(Name!(...))]`.
- Top-level attributes written as `#[name!]` or `#![name!(...)]`.

Unlike "real" attributes, optional parenthesised arguments after the `!` are allowed to be entirely arbitrary token trees, meaning they can effectively contain any token sequence.  These are supported to allow custom attribute macros to easily take arguments.

Derivations parse the item and emit whatever additional definitions needed.  They *cannot* change the item itself, and do not receive any other attributes attached to the item.

Attributes receive *everything* lexically *after* themselves, and must re-emit the item.  This allows attributes to make changes to the item, drop or alter other attributes, *etc.*.  This power makes writing attribute macros more difficult, however.

# Macro Derivations

Given the following input:

```ignore
#[derive(Copy, Name!(args...), Clone, Another!, Debug)]
struct Foo;
```

`macro_attr!` will expand to the equivalent of:

```ignore
#[derive(Copy, Clone, Debug)]
struct Foo;

Name!((args...) struct Foo;);
Another!(() struct Foo;);
```

Note that macro derives may be mixed with regular derives, or put in their own `#[derive(...)]` attribute.  Also note that macro derive invocations are *not* passed the other attributes on the item; input will consist of the arguments provided to the derivation (*i.e.* `(args...)` in this example), the item's visibility (if any), and the item definition itself.

A macro derivation invoked *without* arguments will be treated as though it was invoked with empty parentheses.  *i.e.* `#[derive(Name!)]` is equivalent to `#[derive(Name!())]`.

A derivation macro may expand to any number of new items derived from the provided input.  There is no way for a derivation macro to alter the item itself (for that, use a macro attribute).

# Macro Attributes

When `macro_attr!` encounters an attribute suffixed with a `!` (*e.g.* `#[name!(args...)]`), it invokes the macro `name!` with everything lexically *after* that attribute.  A macro attribute is free to add to, remove from, or alter the provided input as it sees fit, before instructing `macro_attr!` to resume parsing.

For example, given the following input:

```ignore
#[make_unitary!]
#[repr(C)]
#[rename_to!(Quux)]
#[doc="Test."]
struct Bar { field: i32 }
```

`macro_attr!` will expand to:

```ignore
make_unitary! {
    (), then $resume,
    #[repr(C)]
    #[rename_to!(Quux)]
    #[doc="Test."]
    struct Bar { field: i32 };
}
```

Note that `$resume` is **not** literal.  When implementing an attribute macro, you should accept this part as `$resume:tt`, and not attempt to inspect or deconstruct the contents.

Assuming `make_unitary!` removes the body of the `struct` it is attached to, `macro_attr!` *requires* that it expand to:

```ignore
macro_attr_callback! {
    $resume,
    #[repr(C)]
    #[rename_to!(Quxx)]
    #[doc="Test."]
    struct Bar;
}
```

`macro_attr!` will then resume parsing, and expand to:

```ignore
rename_to! {
    (Quxx), then $resume,
    #[doc="Test."]
    struct Bar;
}
```

Assuming `rename_to!` does the obvious thing and changes the name of the item it is attached to, it should expand to:

```ignore
macro_attr_callback! {
    $resume,
    #[doc="Test."]
    struct Quxx;
}
```

Once more, `macro_attr!` will resume, and produce the final expansion of:

```ignore
#[repr(C)]
#[doc="Test."]
struct Quxx;
```

Note that normal attributes are automatically carried through and re-attached to the item.

Macro attributes should be used as sparingly as possible: due to the way Rust macros work, they must expand recursively in sequence, which can quickly consume the available macro recursion limit.  This limit can be raised, but it makes for a less-than-ideal user experience if you are authoring macros to be used by others.
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
            then (macro_attr_impl! {
                @split_attrs_resume
                $non_derives,
                $derives,
            }),
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
            then (macro_attr_impl! {
                @split_attrs_resume
                $non_derives,
                $derives,
            }),
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
                    then (macro_attr_impl! {
                        @split_attrs_resume
                        ($($non_derives)*),
                        $derives,
                    }),
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
                    then (macro_attr_impl! {
                        @split_attrs_resume
                        ($($non_derives)*),
                        $derives,
                    }),
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

It takes an arbitrary macro call `(name!(args...))`, plus some sequence of `new_args...`, and expands `name!(args... new_args...)`.

Importantly, it works irrespective of the kind of grouping syntax used for the macro arguments, simplifying macros which need to *capture* callbacks.
*/
#[macro_export]
macro_rules! macro_attr_callback {
    (
        ($cb:ident ! { $($cb_fixed:tt)* }),
        $($args:tt)*
    ) => {
        $cb! { $($cb_fixed)* $($args)* }
    };

    (
        ($cb:ident ! [ $($cb_fixed:tt)* ]),
        $($args:tt)*
    ) => {
        $cb! [ $($cb_fixed)* $($args)* ]
    };

    (
        ($cb:ident ! ( $($cb_fixed:tt)* )),
        $($args:tt)*
    ) => {
        $cb! ( $($cb_fixed)* $($args)* )
    };
}

/**
This macro provides a simple way to select between two branches of code, depending on whether or not support for procedural macros is enabled or not.
*/
#[doc(hidden)]
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
#[doc(hidden)]
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
