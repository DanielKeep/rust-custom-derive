#![cfg(feature = "std-unstable")]

#[macro_export]
macro_rules! NewtypeOne {
    (() $(pub)* struct $name:ident(pub $_t0:ty);) => {
        impl ::std::num::One for $name {
            fn one() -> Self {
                $name(::std::num::One::one())
            }
        }
    };

    (() $(pub)* struct $name:ident($_t0:ty);) => {
        impl ::std::num::One for $name {
            fn one() -> Self {
                $name(::std::num::One::one())
            }
        }
    };
}

#[macro_export]
macro_rules! NewtypeZero {
    (() $(pub)* struct $name:ident(pub $_t0:ty);) => {
        impl ::std::num::Zero for $name {
            fn zero() -> Self {
                $name(::std::num::Zero::zero())
            }
        }
    };

    (() $(pub)* struct $name:ident($_t0:ty);) => {
        impl ::std::num::Zero for $name {
            fn zero() -> Self {
                $name(::std::num::Zero::zero())
            }
        }
    };
}
