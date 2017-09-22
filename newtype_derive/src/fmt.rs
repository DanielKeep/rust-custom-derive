/*
Copyright ⓒ 2017 macro-attr contributors.

Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/

#[doc(hidden)]
#[macro_export]
macro_rules! newtype_fmt {
    ($fmt_trait:ident, $name:ident) => {
        impl ::std::fmt::$fmt_trait for $name {
            fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::$fmt_trait::fmt(&self.0, fmt)
            }
        }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeBinary!)]
    struct $name($inner_ty);
}
```

Derives an implementation of the `Binary` trait by deferring to the implementation of the inner type.
*/
// ntf Binary
#[macro_export]
macro_rules! NewtypeBinary {
    (() $(pub)* struct $name:ident $_field:tt;) => {
        newtype_fmt! { Binary, $name }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeDebug!)]
    struct $name($inner_ty);
}
```

Derives an implementation of the `Debug` trait by deferring to the implementation of the inner type.
*/
// ntf Debug
#[macro_export]
macro_rules! NewtypeDebug {
    (() $(pub)* struct $name:ident $_field:tt;) => {
        newtype_fmt! { Debug, $name }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeDisplay!)]
    struct $name($inner_ty);
}
```

Derives an implementation of the `Display` trait by deferring to the implementation of the inner type.
*/
// ntf Display
#[macro_export]
macro_rules! NewtypeDisplay {
    (() $(pub)* struct $name:ident $_field:tt;) => {
        newtype_fmt! { Display, $name }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeLowerExp!)]
    struct $name($inner_ty);
}
```

Derives an implementation of the `LowerExp` trait by deferring to the implementation of the inner type.
*/
// ntf LowerExp
#[macro_export]
macro_rules! NewtypeLowerExp {
    (() $(pub)* struct $name:ident $_field:tt;) => {
        newtype_fmt! { LowerExp, $name }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeLowerHex!)]
    struct $name($inner_ty);
}
```

Derives an implementation of the `LowerHex` trait by deferring to the implementation of the inner type.
*/
// ntf LowerHex
#[macro_export]
macro_rules! NewtypeLowerHex {
    (() $(pub)* struct $name:ident $_field:tt;) => {
        newtype_fmt! { LowerHex, $name }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeOctal!)]
    struct $name($inner_ty);
}
```

Derives an implementation of the `Octal` trait by deferring to the implementation of the inner type.
*/
// ntf Octal
#[macro_export]
macro_rules! NewtypeOctal {
    (() $(pub)* struct $name:ident $_field:tt;) => {
        newtype_fmt! { Octal, $name }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypePointer!)]
    struct $name($inner_ty);
}
```

Derives an implementation of the `Pointer` trait by deferring to the implementation of the inner type.
*/
// ntf Pointer
#[macro_export]
macro_rules! NewtypePointer {
    (() $(pub)* struct $name:ident $_field:tt;) => {
        newtype_fmt! { Pointer, $name }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeUpperExp!)]
    struct $name($inner_ty);
}
```

Derives an implementation of the `UpperExp` trait by deferring to the implementation of the inner type.
*/
// ntf UpperExp
#[macro_export]
macro_rules! NewtypeUpperExp {
    (() $(pub)* struct $name:ident $_field:tt;) => {
        newtype_fmt! { UpperExp, $name }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeUpperHex!)]
    struct $name($inner_ty);
}
```

Derives an implementation of the `UpperHex` trait by deferring to the implementation of the inner type.
*/
// ntf UpperHex
#[macro_export]
macro_rules! NewtypeUpperHex {
    (() $(pub)* struct $name:ident $_field:tt;) => {
        newtype_fmt! { UpperHex, $name }
    };
}
