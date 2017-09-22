/*
Copyright ⓒ 2017 macro-attr contributors.

Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
/*!
# `Newtype$unop` Template

Given `/\/\/\s*(ntuop\s+([A-Za-z0-9]+),\s*([a-z_]+))\n(^#\[.+?\]$\n)*^macro_rules!.*?\{$\n(^ +.*?$\n)*^\}$/`,

```ignore
// \1
#[macro_export]
macro_rules! Newtype\2 {
    ((*) $($tts:tt)*) => {
        Newtype\2! { () $($tts)* }
        Newtype\2! { (&self) $($tts)* }
    };
    (() $($tts:tt)*) => {
        newtype_wrap_un_op! { trait: (::std::ops::\2)::\3, kind: simple, item: $($tts)* }
    };
    ((&self) $($tts:tt)*) => {
        newtype_wrap_un_op! { trait: (::std::ops::\2)::\3, kind: simple_ref, item: $($tts)* }
    };
}
```
*/

#[doc(hidden)]
#[macro_export]
macro_rules! newtype_wrap_un_op {
    (
        trait: ($($tr:tt)*)::$meth:ident,
        kind: simple,
        item: $(pub)* struct $name:ident(pub $t:ty);
    ) => {
        newtype_as_item! {
            impl $($tr)* for $name {
                type Output = $name;

                #[inline]
                fn $meth(self) -> $name {
                    $name((self.0).$meth())
                }
            }
        }
    };

    (
        trait: ($($tr:tt)*)::$meth:ident,
        kind: simple,
        item: $(pub)* struct $name:ident($t:ty);
    ) => {
        newtype_as_item! {
            impl $($tr)* for $name {
                type Output = $name;

                #[inline]
                fn $meth(self) -> $name {
                    $name((self.0).$meth())
                }
            }
        }
    };

    (
        trait: ($($tr:tt)*)::$meth:ident,
        kind: simple_ref,
        item: $(pub)* struct $name:ident(pub $t:ty);
    ) => {
        newtype_as_item! {
            impl<'a> $($tr)* for &'a $name {
                type Output = $name;

                #[inline]
                fn $meth(self) -> $name {
                    $name((self.0).$meth())
                }
            }
        }
    };

    (
        trait: ($($tr:tt)*)::$meth:ident,
        kind: simple_ref,
        item: $(pub)* struct $name:ident($t:ty);
    ) => {
        newtype_as_item! {
            impl<'a> $($tr)* for &'a $name {
                type Output = $name;

                #[inline]
                fn $meth(self) -> $name {
                    $name((self.0).$meth())
                }
            }
        }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeNeg!)]
    #[derive(NewtypeNeg!(&self))]
    struct $name($inner_ty);
}
// or:
macro_attr! {
    #[derive(NewtypeNeg!(*))]
    struct $name($inner_ty);
}
```

Derives an implementation of the `Neg` trait by deferring to the implementation of the inner type.  The first two invocation forms shown above correspond to the following implementations:

- `impl Neg<Output=$name> for $name`
- `impl<'a> Neg<Output=$name> for &'a $name`

The last form expands to the first two forms.
*/
// ntuop Neg,      neg
#[macro_export]
macro_rules! NewtypeNeg {
    ((*) $($tts:tt)*) => {
        NewtypeNeg! { () $($tts)* }
        NewtypeNeg! { (&self) $($tts)* }
    };
    (() $($tts:tt)*) => {
        newtype_wrap_un_op! { trait: (::std::ops::Neg)::neg, kind: simple, item: $($tts)* }
    };
    ((&self) $($tts:tt)*) => {
        newtype_wrap_un_op! { trait: (::std::ops::Neg)::neg, kind: simple_ref, item: $($tts)* }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeNot!)]
    #[derive(NewtypeNot!(&self))]
    struct $name($inner_ty);
}
// or:
macro_attr! {
    #[derive(NewtypeNot!(*))]
    struct $name($inner_ty);
}
```

Derives an implementation of the `Not` trait by deferring to the implementation of the inner type.  The first two invocation forms shown above correspond to the following implementations:

- `impl Not<Output=$name> for $name`
- `impl<'a> Not<Output=$name> for &'a $name`

The last form expands to the first two forms.
*/
// ntuop Not,      not
#[macro_export]
macro_rules! NewtypeNot {
    ((*) $($tts:tt)*) => {
        NewtypeNot! { () $($tts)* }
        NewtypeNot! { (&self) $($tts)* }
    };
    (() $($tts:tt)*) => {
        newtype_wrap_un_op! { trait: (::std::ops::Not)::not, kind: simple, item: $($tts)* }
    };
    ((&self) $($tts:tt)*) => {
        newtype_wrap_un_op! { trait: (::std::ops::Not)::not, kind: simple_ref, item: $($tts)* }
    };
}
