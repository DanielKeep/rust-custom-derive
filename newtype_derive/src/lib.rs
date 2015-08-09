/*
# `Newtype$binop` Template

Given `/\/\/\s*(ntbop\s+([A-Za-z0-9]+),\s*([a-z_]+))/`,

```
// \1
#[macro_export]
macro_rules! Newtype\2 {
    (() $(pub)* struct $name:ident($(pub)* $t0:ty);) => {
        impl ::std::ops::\2 for $name {
            type Output = $name;
            fn \3(self, rhs: Self) -> $name {
                $name(self.0.\3(rhs.0))
            }
        }
    };
    ((ref) $(pub)* struct $name:ident($(pub)* $t0:ty);) => {
        impl<'a> ::std::ops::\2 for &'a $name {
            type Output = $name;
            fn \3(self, rhs: Self) -> $name {
                $name(self.0.\3(rhs.0))
            }
        }
    };
}
```
*/

// ntbop Add,      add
#[macro_export]
macro_rules! NewtypeAdd {
    (() $(pub)* struct $name:ident($(pub)* $t0:ty);) => {
        impl ::std::ops::Add for $name {
            type Output = $name;
            fn add(self, rhs: Self) -> $name {
                $name(self.0.add(rhs.0))
            }
        }
    };
    ((ref) $(pub)* struct $name:ident($(pub)* $t0:ty);) => {
        impl<'a> ::std::ops::Add for &'a $name {
            type Output = $name;
            fn add(self, rhs: Self) -> $name {
                $name(self.0.add(rhs.0))
            }
        }
    };
}

// ntbop BitAnd,   bitand
#[macro_export]
macro_rules! NewtypeBitAnd {
    (() $(pub)* struct $name:ident($(pub)* $t0:ty);) => {
        impl ::std::ops::BitAnd for $name {
            type Output = $name;
            fn bitand(self, rhs: Self) -> $name {
                $name(self.0.bitand(rhs.0))
            }
        }
    };
    ((ref) $(pub)* struct $name:ident($(pub)* $t0:ty);) => {
        impl<'a> ::std::ops::BitAnd for &'a $name {
            type Output = $name;
            fn bitand(self, rhs: Self) -> $name {
                $name(self.0.bitand(rhs.0))
            }
        }
    };
}

// ntbop BitOr,    bitor
#[macro_export]
macro_rules! NewtypeBitOr {
    (() $(pub)* struct $name:ident($(pub)* $t0:ty);) => {
        impl ::std::ops::BitOr for $name {
            type Output = $name;
            fn bitor(self, rhs: Self) -> $name {
                $name(self.0.bitor(rhs.0))
            }
        }
    };
    ((ref) $(pub)* struct $name:ident($(pub)* $t0:ty);) => {
        impl<'a> ::std::ops::BitOr for &'a $name {
            type Output = $name;
            fn bitor(self, rhs: Self) -> $name {
                $name(self.0.bitor(rhs.0))
            }
        }
    };
}

// ntbop BitXor,   bitxor
#[macro_export]
macro_rules! NewtypeBitXor {
    (() $(pub)* struct $name:ident($(pub)* $t0:ty);) => {
        impl ::std::ops::BitXor for $name {
            type Output = $name;
            fn bitxor(self, rhs: Self) -> $name {
                $name(self.0.bitxor(rhs.0))
            }
        }
    };
    ((ref) $(pub)* struct $name:ident($(pub)* $t0:ty);) => {
        impl<'a> ::std::ops::BitXor for &'a $name {
            type Output = $name;
            fn bitxor(self, rhs: Self) -> $name {
                $name(self.0.bitxor(rhs.0))
            }
        }
    };
}

// ntbop Div,      div
#[macro_export]
macro_rules! NewtypeDiv {
    (() $(pub)* struct $name:ident($(pub)* $t0:ty);) => {
        impl ::std::ops::Div for $name {
            type Output = $name;
            fn div(self, rhs: Self) -> $name {
                $name(self.0.div(rhs.0))
            }
        }
    };
    ((ref) $(pub)* struct $name:ident($(pub)* $t0:ty);) => {
        impl<'a> ::std::ops::Div for &'a $name {
            type Output = $name;
            fn div(self, rhs: Self) -> $name {
                $name(self.0.div(rhs.0))
            }
        }
    };
}

// ntbop Mul,      mul
#[macro_export]
macro_rules! NewtypeMul {
    (() $(pub)* struct $name:ident($(pub)* $t0:ty);) => {
        impl ::std::ops::Mul for $name {
            type Output = $name;
            fn mul(self, rhs: Self) -> $name {
                $name(self.0.mul(rhs.0))
            }
        }
    };
    ((ref) $(pub)* struct $name:ident($(pub)* $t0:ty);) => {
        impl<'a> ::std::ops::Mul for &'a $name {
            type Output = $name;
            fn mul(self, rhs: Self) -> $name {
                $name(self.0.mul(rhs.0))
            }
        }
    };
}

// ntbop Rem,      rem
#[macro_export]
macro_rules! NewtypeRem {
    (() $(pub)* struct $name:ident($(pub)* $t0:ty);) => {
        impl ::std::ops::Rem for $name {
            type Output = $name;
            fn rem(self, rhs: Self) -> $name {
                $name(self.0.rem(rhs.0))
            }
        }
    };
    ((ref) $(pub)* struct $name:ident($(pub)* $t0:ty);) => {
        impl<'a> ::std::ops::Rem for &'a $name {
            type Output = $name;
            fn rem(self, rhs: Self) -> $name {
                $name(self.0.rem(rhs.0))
            }
        }
    };
}

// ntbop Sub,      sub
#[macro_export]
macro_rules! NewtypeSub {
    (() $(pub)* struct $name:ident($(pub)* $t0:ty);) => {
        impl ::std::ops::Sub for $name {
            type Output = $name;
            fn sub(self, rhs: Self) -> $name {
                $name(self.0.sub(rhs.0))
            }
        }
    };
    ((ref) $(pub)* struct $name:ident($(pub)* $t0:ty);) => {
        impl<'a> ::std::ops::Sub for &'a $name {
            type Output = $name;
            fn sub(self, rhs: Self) -> $name {
                $name(self.0.sub(rhs.0))
            }
        }
    };
}

// ntsop Shl,      shl
#[macro_export]
macro_rules! NewtypeShl {
    (() $(pub)* struct $name:ident($(pub)* $t0:ty);) => {
        impl ::std::ops::Shl<$name> for $name {
            type Output = $name;
            fn shl(self, rhs: Self) -> $name {
                $name(self.0.shl(rhs.0))
            }
        }
    };
    ((ref) $(pub)* struct $name:ident($(pub)* $t0:ty);) => {
        impl<'a> ::std::ops::Shl<&'a $name> for &'a $name {
            type Output = $name;
            fn shl(self, rhs: Self) -> $name {
                $name(self.0.shl(&rhs.0))
            }
        }
    };
    ((<_>) $(pub)* struct $name:ident($(pub)* $t0:ty);) => {
        impl<T> ::std::ops::Shl<T> for $name
        where $t0: ::std::ops::Shl<T, Output=$t0> {
            type Output = $name;
            fn shl(self, rhs: T) -> $name {
                $name(self.0.shl(rhs))
            }
        }
    };
    ((ref <_>) $(pub)* struct $name:ident($(pub)* $t0:ty);) => {
        impl<'a, T> ::std::ops::Shl<T> for &'a $name
        where &'a $t0: ::std::ops::Shl<T, Output=$t0> {
            type Output = $name;
            fn shl(self, rhs: T) -> $name {
                $name((&self.0).shl(rhs))
            }
        }
    };
}

// ntsop Shr,      shr
#[macro_export]
macro_rules! NewtypeShr {
    (() $(pub)* struct $name:ident($(pub)* $t0:ty);) => {
        impl ::std::ops::Shr<$name> for $name {
            type Output = $name;
            fn shr(self, rhs: Self) -> $name {
                $name(self.0.shr(rhs.0))
            }
        }
    };
    ((ref) $(pub)* struct $name:ident($(pub)* $t0:ty);) => {
        impl<'a> ::std::ops::Shr<&'a $name> for &'a $name {
            type Output = $name;
            fn shr(self, rhs: Self) -> $name {
                $name(self.0.shr(&rhs.0))
            }
        }
    };
    ((<_>) $(pub)* struct $name:ident($(pub)* $t0:ty);) => {
        impl<T> ::std::ops::Shr<T> for $name
        where $t0: ::std::ops::Shr<T, Output=$t0> {
            type Output = $name;
            fn shr(self, rhs: T) -> $name {
                $name(self.0.shr(rhs))
            }
        }
    };
    ((ref <_>) $(pub)* struct $name:ident($(pub)* $t0:ty);) => {
        impl<'a, T> ::std::ops::Shr<T> for &'a $name
        where &'a $t0: ::std::ops::Shr<T, Output=$t0> {
            type Output = $name;
            fn shr(self, rhs: T) -> $name {
                $name((&self.0).shr(rhs))
            }
        }
    };
}

// ntuop Neg,      neg
#[macro_export]
macro_rules! NewtypeNeg {
    (() $(pub)* struct $name:ident($(pub)* $t0:ty);) => {
        impl ::std::ops::Neg for $name {
            type Output = $name;
            fn neg(self) -> $name {
                $name(self.0.neg())
            }
        }
    };
    ((ref) $(pub)* struct $name:ident($(pub)* $t0:ty);) => {
        impl<'a> ::std::ops::Neg for &'a $name {
            type Output = $name;
            fn neg(self) -> $name {
                $name(self.0.neg())
            }
        }
    };
}

// ntuop Not,      not
#[macro_export]
macro_rules! NewtypeNot {
    (() $(pub)* struct $name:ident($(pub)* $t0:ty);) => {
        impl ::std::ops::Not for $name {
            type Output = $name;
            fn not(self) -> $name {
                $name(self.0.not())
            }
        }
    };
    ((ref) $(pub)* struct $name:ident($(pub)* $t0:ty);) => {
        impl<'a> ::std::ops::Not for &'a $name {
            type Output = $name;
            fn not(self) -> $name {
                $name(self.0.not())
            }
        }
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
