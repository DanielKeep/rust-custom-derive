/*
# `Newtype$binop` Template

Given `/\/\/\s*(ntbop\s+([A-Za-z0-9]+),\s*([a-z_]+))\n(^#\[.+?\]$\n)*^macro_rules!.*?\{$\n(^ +.*?$\n)*^\}$/`,

```
// \1
#[macro_export]
macro_rules! Newtype\2 {
    (() $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::\2)::\3, kind: simple, item: $($tts)* }
    };
    ((&self) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::\2)::\3, kind: simple_ref, item: $($tts)* }
    };
    ((&self, $($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::\2)::\3, kind: ref_rhs_rewrap($($rhs)*), item: $($tts)* }
    };
    (($($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::\2)::\3, kind: rhs_rewrap($($rhs)*), item: $($tts)* }
    };
}
```

# `Newtype$unop` Template

Given `/\/\/\s*(ntuop\s+([A-Za-z0-9]+),\s*([a-z_]+))\n(^#\[.+?\]$\n)*^macro_rules!.*?\{$\n(^ +.*?$\n)*^\}$/`,

```
// \1
#[macro_export]
macro_rules! Newtype\2 {
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
macro_rules! newtype_as_item {
    ($i:item) => {$i};
}

#[doc(hidden)]
#[macro_export]
macro_rules! newtype_wrap_bin_op {
    (
        trait: ($($tr:tt)*)::$meth:ident,
        kind: simple,
        item: $(pub)* struct $name:ident($(pub)* $t:ty);
    ) => {
        newtype_as_item! {
            impl $($tr)*<$name> for $name {
                type Output = $name;
                fn $meth(self, rhs: Self) -> $name {
                    $name((self.0).$meth(rhs.0))
                }
            }
        }
    };

    (
        trait: ($($tr:tt)*)::$meth:ident,
        kind: simple_ref,
        item: $(pub)* struct $name:ident($(pub)* $t:ty);
    ) => {
        newtype_as_item! {
            impl<'a> $($tr)*<&'a $name> for &'a $name {
                type Output = $name;
                fn $meth(self, rhs: Self) -> $name {
                    $name((self.0).$meth(rhs.0))
                }
            }
        }
    };

    (
        trait: ($($tr:tt)*)::$meth:ident,
        kind: rhs_rewrap($rhs:ty),
        item: $(pub)* struct $name:ident($(pub)* $t:ty);
    ) => {
        newtype_as_item! {
            impl $($tr)*<$rhs> for $name {
                type Output = $name;
                fn $meth(self, rhs: $rhs) -> $name {
                    $name((self.0).$meth(rhs))
                }
            }
        }
    };

    (
        trait: ($($tr:tt)*)::$meth:ident,
        kind: ref_rhs_rewrap($rhs:ty),
        item: $(pub)* struct $name:ident($(pub)* $t:ty);
    ) => {
        newtype_as_item! {
            impl<'a> $($tr)*<$rhs> for &'a $name {
                type Output = $name;
                fn $meth(self, rhs: $rhs) -> $name {
                    $name((self.0).$meth(rhs))
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! newtype_wrap_un_op {
    (
        trait: ($($tr:tt)*)::$meth:ident,
        kind: simple,
        item: $(pub)* struct $name:ident($(pub)* $t:ty);
    ) => {
        newtype_as_item! {
            impl $($tr)* for $name {
                type Output = $name;
                fn $meth(self) -> $name {
                    $name((self.0).$meth())
                }
            }
        }
    };

    (
        trait: ($($tr:tt)*)::$meth:ident,
        kind: simple_ref,
        item: $(pub)* struct $name:ident($(pub)* $t:ty);
    ) => {
        newtype_as_item! {
            impl<'a> $($tr)* for &'a $name {
                type Output = $name;
                fn $meth(self) -> $name {
                    $name((self.0).$meth())
                }
            }
        }
    };
}

// ntbop Add,      add
#[macro_export]
macro_rules! NewtypeAdd {
    (() $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Add)::add, kind: simple, item: $($tts)* }
    };
    ((&self) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Add)::add, kind: simple_ref, item: $($tts)* }
    };
    ((&self, $($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Add)::add, kind: ref_rhs_rewrap($($rhs)*), item: $($tts)* }
    };
    (($($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Add)::add, kind: rhs_rewrap($($rhs)*), item: $($tts)* }
    };
}

// ntbop BitAnd,   bitand
#[macro_export]
macro_rules! NewtypeBitAnd {
    (() $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::BitAnd)::bitand, kind: simple, item: $($tts)* }
    };
    ((&self) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::BitAnd)::bitand, kind: simple_ref, item: $($tts)* }
    };
    ((&self, $($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::BitAnd)::bitand, kind: ref_rhs_rewrap($($rhs)*), item: $($tts)* }
    };
    (($($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::BitAnd)::bitand, kind: rhs_rewrap($($rhs)*), item: $($tts)* }
    };
}

// ntbop BitOr,    bitor
#[macro_export]
macro_rules! NewtypeBitOr {
    (() $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::BitOr)::bitor, kind: simple, item: $($tts)* }
    };
    ((&self) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::BitOr)::bitor, kind: simple_ref, item: $($tts)* }
    };
    ((&self, $($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::BitOr)::bitor, kind: ref_rhs_rewrap($($rhs)*), item: $($tts)* }
    };
    (($($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::BitOr)::bitor, kind: rhs_rewrap($($rhs)*), item: $($tts)* }
    };
}

// ntbop BitXor,   bitxor
#[macro_export]
macro_rules! NewtypeBitXor {
    (() $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::BitXor)::bitxor, kind: simple, item: $($tts)* }
    };
    ((&self) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::BitXor)::bitxor, kind: simple_ref, item: $($tts)* }
    };
    ((&self, $($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::BitXor)::bitxor, kind: ref_rhs_rewrap($($rhs)*), item: $($tts)* }
    };
    (($($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::BitXor)::bitxor, kind: rhs_rewrap($($rhs)*), item: $($tts)* }
    };
}

// ntbop Div,      div
#[macro_export]
macro_rules! NewtypeDiv {
    (() $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Div)::div, kind: simple, item: $($tts)* }
    };
    ((&self) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Div)::div, kind: simple_ref, item: $($tts)* }
    };
    ((&self, $($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Div)::div, kind: ref_rhs_rewrap($($rhs)*), item: $($tts)* }
    };
    (($($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Div)::div, kind: rhs_rewrap($($rhs)*), item: $($tts)* }
    };
}

// ntbop Mul,      mul
#[macro_export]
macro_rules! NewtypeMul {
    (() $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Mul)::mul, kind: simple, item: $($tts)* }
    };
    ((&self) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Mul)::mul, kind: simple_ref, item: $($tts)* }
    };
    ((&self, $($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Mul)::mul, kind: ref_rhs_rewrap($($rhs)*), item: $($tts)* }
    };
    (($($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Mul)::mul, kind: rhs_rewrap($($rhs)*), item: $($tts)* }
    };
}

// ntbop Rem,      rem
#[macro_export]
macro_rules! NewtypeRem {
    (() $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Rem)::rem, kind: simple, item: $($tts)* }
    };
    ((&self) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Rem)::rem, kind: simple_ref, item: $($tts)* }
    };
    ((&self, $($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Rem)::rem, kind: ref_rhs_rewrap($($rhs)*), item: $($tts)* }
    };
    (($($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Rem)::rem, kind: rhs_rewrap($($rhs)*), item: $($tts)* }
    };
}

// ntbop Sub,      sub
#[macro_export]
macro_rules! NewtypeSub {
    (() $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Sub)::sub, kind: simple, item: $($tts)* }
    };
    ((&self) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Sub)::sub, kind: simple_ref, item: $($tts)* }
    };
    ((&self, $($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Sub)::sub, kind: ref_rhs_rewrap($($rhs)*), item: $($tts)* }
    };
    (($($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Sub)::sub, kind: rhs_rewrap($($rhs)*), item: $($tts)* }
    };
}

// ntbop Shl,      shl
#[macro_export]
macro_rules! NewtypeShl {
    (() $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Shl)::shl, kind: simple, item: $($tts)* }
    };
    ((&self) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Shl)::shl, kind: simple_ref, item: $($tts)* }
    };
    ((&self, $($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Shl)::shl, kind: ref_rhs_rewrap($($rhs)*), item: $($tts)* }
    };
    (($($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Shl)::shl, kind: rhs_rewrap($($rhs)*), item: $($tts)* }
    };
}

// ntbop Shr,      shr
#[macro_export]
macro_rules! NewtypeShr {
    (() $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Shr)::shr, kind: simple, item: $($tts)* }
    };
    ((&self) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Shr)::shr, kind: simple_ref, item: $($tts)* }
    };
    ((&self, $($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Shr)::shr, kind: ref_rhs_rewrap($($rhs)*), item: $($tts)* }
    };
    (($($rhs:tt)*) $($tts:tt)*) => {
        newtype_wrap_bin_op! { trait: (::std::ops::Shr)::shr, kind: rhs_rewrap($($rhs)*), item: $($tts)* }
    };
}

// ntuop Neg,      neg
#[macro_export]
macro_rules! NewtypeNeg {
    (() $($tts:tt)*) => {
        newtype_wrap_un_op! { trait: (::std::ops::Neg)::neg, kind: simple, item: $($tts)* }
    };
    ((&self) $($tts:tt)*) => {
        newtype_wrap_un_op! { trait: (::std::ops::Neg)::neg, kind: simple_ref, item: $($tts)* }
    };
}

// ntuop Not,      not
#[macro_export]
macro_rules! NewtypeNot {
    (() $($tts:tt)*) => {
        newtype_wrap_un_op! { trait: (::std::ops::Not)::not, kind: simple, item: $($tts)* }
    };
    ((&self) $($tts:tt)*) => {
        newtype_wrap_un_op! { trait: (::std::ops::Not)::not, kind: simple_ref, item: $($tts)* }
    };
}

#[macro_export]
macro_rules! NewtypeDeref {
    (() $(pub)* struct $name:ident($(pub)* $t0:ty);) => {
        impl ::std::ops::Deref for $name {
            type Target = $t0;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

#[macro_export]
macro_rules! NewtypeDerefMut {
    (() $(pub)* struct $name:ident($(pub)* $t0:ty);) => {
        impl ::std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

#[macro_export]
macro_rules! NewtypeIndex {
    (($index_ty:ty) $(pub)* struct $name:ident($(pub)* $t0:ty);) => {
        impl ::std::ops::Index<$index_ty> for $name {
            type Output = <$t0 as ::std::ops::Index<$index_ty>>::Output;
            fn index(&self, index: $index_ty) -> &Self::Output {
                (&self.0).index(index)
            }
        }
    };
}

#[macro_export]
macro_rules! NewtypeIndexMut {
    (($index_ty:ty) $(pub)* struct $name:ident($(pub)* $t0:ty);) => {
        impl ::std::ops::IndexMut<$index_ty> for $name {
            fn index_mut(&mut self, index: $index_ty) -> &mut Self::Output {
                (&mut self.0).index_mut(index)
            }
        }
    };
}

#[macro_export]
macro_rules! NewtypeFrom {
    (() $(pub)* struct $name:ident($(pub)* $t0:ty);) => {
        impl ::std::convert::From<$t0> for $name {
            fn from(v: $t0) -> Self {
                $name(v)
            }
        }
        impl ::std::convert::From<$name> for $t0 {
            fn from(v: $name) -> Self {
                v.0
            }
        }
    };
}
