/*
Copyright ⓒ 2016 rust-custom-derive contributors.

Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
#[macro_use] extern crate custom_derive;

macro_rules! remove_body {
    (
        (),
        then $cb:ident!$cb_arg:tt,
        $(#[$($attrs:tt)*])*
        struct $name:ident $($_tail:tt)*
    ) => {
        custom_derive! {
            @callback
            $cb!$cb_arg,
            $(#[$($attrs)*])*
            struct $name;
        }
    };
}

macro_rules! use_secret_alias {
    (
        ($name:ident),
        then $cb:ident!$cb_arg:tt,
        $(#[$($attrs:tt)*])*
        struct $_old_name:ident $($tail:tt)*
    ) => {
        custom_derive! {
            @callback
            $cb!$cb_arg,
            $(#[$($attrs)*])*
            struct $name $($tail)*
        }
    };
}

custom_derive! {
    #[derive(Debug)]
    #[remove_body!]
    #[use_secret_alias!(Alucard)]
    struct Dracula {
        pub vulnerabilities: Vec<Vulnerability>,
    }
}

#[test]
fn test_mac_attrs() {
    assert_eq!(format!("{:?}", Alucard), "Alucard");
}
