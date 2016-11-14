/*
Copyright ⓒ 2016 macro-attr contributors.

Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
#[macro_use] extern crate macro_attr;

macro_rules! Name {
    (
        ()
        struct $name:ident $($_tail:tt)*
    ) => {
        impl $name {
            pub fn name() -> &'static str {
                stringify!($name)
            }
        }
    };
}

macro_rules! remove_body {
    (
        (),
        then $cb:tt,
        $(#[$($attrs:tt)*])*
        struct $name:ident $($_tail:tt)*
    ) => {
        macro_attr_callback! {
            $cb,
            $(#[$($attrs)*])*
            struct $name;
        }
    };
}

macro_rules! use_secret_alias {
    (
        ($name:ident),
        then $cb:tt,
        $(#[$($attrs:tt)*])*
        struct $_old_name:ident $($tail:tt)*
    ) => {
        macro_attr_callback! {
            $cb,
            $(#[$($attrs)*])*
            struct $name $($tail)*
        }
    };
}

macro_attr! {
    #[derive(Debug, Name!)]
    #[remove_body!]
    #[use_secret_alias!(Alucard)]
    struct Dracula {
        pub vulnerabilities: Vec<Vulnerability>,
    }
}

#[test]
fn test_mac_attrs() {
    assert_eq!(format!("{:?}", Alucard), "Alucard");
    assert_eq!(Alucard::name(), "Alucard");
}
