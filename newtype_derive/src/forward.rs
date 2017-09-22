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
    #[derive(NewtypeDeref!)]
    struct $name($inner_ty);
}
```

Derives an implementation of the `Deref` trait by deferring to the implementation of the inner type.
*/
// ntf Deref
#[macro_export]
macro_rules! NewtypeDeref {
    (() $(pub)* struct $name:ident(pub $t0:ty);) => {
        impl ::std::ops::Deref for $name {
            type Target = $t0;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };

    (() $(pub)* struct $name:ident($t0:ty);) => {
        impl ::std::ops::Deref for $name {
            type Target = $t0;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeDerefMut!)]
    struct $name($inner_ty);
}
```

Derives an implementation of the `DerefMut` trait by deferring to the implementation of the inner type.
*/
// ntf DerefMut
#[macro_export]
macro_rules! NewtypeDerefMut {
    (() $(pub)* struct $name:ident(pub $t0:ty);) => {
        impl ::std::ops::DerefMut for $name {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };

    (() $(pub)* struct $name:ident($t0:ty);) => {
        impl ::std::ops::DerefMut for $name {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeIndex!($index_ty))]
    struct $name($inner_ty);
}
```

Derives an implementation of the `Index<$index_ty>` trait by deferring to the implementation of the inner type.
*/
// nti Index
#[macro_export]
macro_rules! NewtypeIndex {
    (($index_ty:ty) $(pub)* struct $name:ident(pub $t0:ty);) => {
        impl ::std::ops::Index<$index_ty> for $name {
            type Output = <$t0 as ::std::ops::Index<$index_ty>>::Output;

            #[inline]
            fn index(&self, index: $index_ty) -> &Self::Output {
                (&self.0).index(index)
            }
        }
    };

    (($index_ty:ty) $(pub)* struct $name:ident($t0:ty);) => {
        impl ::std::ops::Index<$index_ty> for $name {
            type Output = <$t0 as ::std::ops::Index<$index_ty>>::Output;

            #[inline]
            fn index(&self, index: $index_ty) -> &Self::Output {
                (&self.0).index(index)
            }
        }
    };
}

/**
```ignore
macro_attr! {
    #[derive(NewtypeIndexMut!($index_ty))]
    struct $name($inner_ty);
}
```

Derives an implementation of the `IndexMut<$index_ty>` trait by deferring to the implementation of the inner type.
*/
// nti IndexMut
#[macro_export]
macro_rules! NewtypeIndexMut {
    (($index_ty:ty) $(pub)* struct $name:ident(pub $t0:ty);) => {
        impl ::std::ops::IndexMut<$index_ty> for $name {
            #[inline]
            fn index_mut(&mut self, index: $index_ty) -> &mut Self::Output {
                (&mut self.0).index_mut(index)
            }
        }
    };

    (($index_ty:ty) $(pub)* struct $name:ident($t0:ty);) => {
        impl ::std::ops::IndexMut<$index_ty> for $name {
            #[inline]
            fn index_mut(&mut self, index: $index_ty) -> &mut Self::Output {
                (&mut self.0).index_mut(index)
            }
        }
    };
}
