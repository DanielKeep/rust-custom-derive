#[macro_use] extern crate custom_derive;
extern crate rustc_serialize;

macro_rules! StableEncodable {
    (
        () $(pub)* struct $name:ident { $($body:tt)* }
    ) => {
        StableEncodable! {
            @impl $name,
            bounds(),
            ty_clss(),
            { $($body)* }
        }
    };

    (
        () $(pub)* struct $name:ident < $($tail:tt)*
    ) => {
        StableEncodable! {
            @extract_gen_args $name,
            ($($tail)*)
            -> bounds(), ty_clss(where)
        }
    };

    (
        @impl $name:ident,
        bounds($($bounds:tt)*),
        ty_clss($($ty_clss:tt)*),
        { $($fnames:ident: $_ftys:ty),* $(,)* }
    ) => {
        StableEncodable! {
            @as_item
            impl<$($bounds)*> rustc_serialize::Encodable for $name<$($bounds)*>
            $($ty_clss)* {
                fn encode<S>(&self, s: &mut S) -> Result<(), S::Error>
                where S: rustc_serialize::Encoder {
                    const NUM_FIELDS: usize = StableEncodable!(@count_tts $($fnames)*);
                    try!(s.emit_struct(stringify!($name), NUM_FIELDS, |s| {
                        // Poor man's enumerate!($($fnames)):
                        let mut idx = 0;
                        $(
                            try!(s.emit_struct_field(stringify!($fnames), idx, |s| {
                                self.$fnames.encode(s)
                            }));
                            idx += 1;
                        )*
                        let _ = idx;
                        Ok(())
                    }));
                    Ok(())
                }
            }
        }
    };

    (@as_item $i:item) => {$i};

    (
        @extract_gen_args $name:ident,
        (> { $($tail:tt)* })
        -> bounds($($bounds:tt)*), ty_clss($($ty_clss:tt)*)
    ) => {
        StableEncodable! {
            @impl $name,
            bounds($($bounds)*),
            ty_clss($($ty_clss)*),
            { $($tail)* }
        }
    };

    (
        @extract_gen_args $name:ident,
        ($ty_name:ident: $($tail)*)
        -> bounds($($bounds:tt)*), ty_clss($($ty_clss:tt)*)
    ) => {
        StableEncodable! {
            @skip_inline_bound $name,
            ($($tail)*)
            -> bounds($($bounds)* $ty_name:),
               ty_clss($($ty_clss)* $ty_name: ::rustc_serialize::Encodable,)
        }
    };

    (
        @extract_gen_args $name:ident,
        ($ty_name:ident $($tail:tt)*)
        -> bounds($($bounds:tt)*), ty_clss($($ty_clss:tt)*)
    ) => {
        StableEncodable! {
            @extract_gen_args $name,
            ($($tail)*)
            -> bounds($($bounds)* $ty_name),
               ty_clss($($ty_clss)* $ty_name: ::rustc_serialize::Encodable,)
        }
    };

    (
        @extract_gen_args $name:ident,
        (, $($tail:tt)*)
        -> bounds($($bounds:tt)*), ty_clss($($ty_clss:tt)*)
    ) => {
        StableEncodable! {
            @extract_gen_args $name,
            ($($tail)*)
            -> bounds($($bounds)* ,), ty_clss($($ty_clss)*)
        }
    };

    (
        @extract_gen_args $name:ident,
        ($lt:tt $($tail:tt)*)
        -> bounds($($bounds:tt)*), ty_clss($($ty_clss:tt)*)
    ) => {
        StableEncodable! {
            @extract_gen_args $name,
            ($($tail)*)
            -> bounds($($bounds)* $lt), ty_clss($($ty_clss)*)
        }
    };

    (
        @skip_inline_bound $name:ident,
        (, $($tail:tt)*)
        -> bounds($($bounds:tt)*), ty_clss($($ty_clss:tt)*)
    ) => {
        StableEncodable! {
            @extract_gen_args $name,
            ($($tail)*)
            -> bounds($($bounds)* ,), ty_clss($($ty_clss)*)
        }
    };

    (
        @skip_inline_bound $name:ident,
        (> { $($tail:tt)* })
        -> bounds($($bounds:tt)*), ty_clss($($ty_clss:tt)*)
    ) => {
        StableEncodable! {
            @impl $name,
            bounds($($bounds)*),
            ty_clss($($ty_clss)*),
            { $($tail)* }
        }
    };

    (@count_tts) => {0usize};
    (@count_tts $_tt:tt $($tail:tt)*) => {1usize + StableEncodable!(@count_tts $($tail)*)};
}

custom_derive! {
    #[derive(Debug, StableEncodable)]
    struct LazyEg<A> { a: A, b: i32, c: (u8, u8, u8) }
}

#[test]
fn test_stable_encodable() {
    let lazy_eg = LazyEg {
        a: String::from("Oh hai!"),
        b: 42,
        c: (1, 3, 0),
    };
    let lazy_eg_s = rustc_serialize::json::encode(&lazy_eg).unwrap();
    assert_eq!(&*lazy_eg_s, r#"{"a":"Oh hai!","b":42,"c":[1,3,0]}"#);
}
