/*
Copyright ⓒ 2017 macro-attr contributors.

Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
/**
```ignore
macro_attr! {
    #[derive(NewtypeProduct!)]
    #[derive(NewtypeProduct!(&Self))]
    struct $name($inner_ty);
}
```

Derives an implementation of the `Product` trait by deferring to the implementation of the inner type.  The two invocation forms shown above correspond to the following implementations:

- `impl Product<$name> for $name`
- `impl<'a> Product<&'a $name> for $name`
*/
// ntiter (w+)
#[cfg(iter_sum_product)]
#[macro_export]
macro_rules! NewtypeProduct {
    ($arg:tt $(pub)* struct $name:ident(pub $t0:ty);) => {
        NewtypeProduct! { $arg struct $name($t0); }
    };

    (() $(pub)* struct $name:ident($t0:ty);) => {
        impl ::std::iter::Product<$name> for $name {
            #[inline]
            fn product<I>(iter: I) -> Self
            where I: Iterator<Item=$name> {
                $name(iter.map(|e| e.0).product::<$t0>())
            }
        }
    };

    ((&Self) $(pub)* struct $name:ident($t0:ty);) => {
        impl<'a> ::std::iter::Product<&'a $name> for $name {
            #[inline]
            fn product<I>(iter: I) -> Self
            where I: Iterator<Item=&'a $name> {
                $name(iter.map(|e| &e.0).product::<$t0>())
            }
        }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeSum!)]
    #[derive(NewtypeSum!(&Self))]
    struct $name($inner_ty);
}
```

Derives an implementation of the `Sum` trait by deferring to the implementation of the inner type.  The two invocation forms shown above correspond to the following implementations:

- `impl Sum<$name> for $name`
- `impl<'a> Sum<&'a $name> for $name`
*/
// ntiter (w+)
#[cfg(iter_sum_product)]
#[macro_export]
macro_rules! NewtypeSum {
    ($arg:tt $(pub)* struct $name:ident(pub $t0:ty);) => {
        NewtypeSum! { $arg struct $name($t0); }
    };

    (() $(pub)* struct $name:ident($t0:ty);) => {
        impl ::std::iter::Sum<$name> for $name {
            #[inline]
            fn sum<I>(iter: I) -> Self
            where I: Iterator<Item=$name> {
                $name(iter.map(|e| e.0).sum::<$t0>())
            }
        }
    };

    ((&Self) $(pub)* struct $name:ident($t0:ty);) => {
        impl<'a> ::std::iter::Sum<&'a $name> for $name {
            #[inline]
            fn sum<I>(iter: I) -> Self
            where I: Iterator<Item=&'a $name> {
                $name(iter.map(|e| &e.0).sum::<$t0>())
            }
        }
    };
}
