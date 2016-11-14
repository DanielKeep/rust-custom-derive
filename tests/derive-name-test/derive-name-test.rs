/*
Copyright ⓒ 2016 macro-attr contributors.

Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
#![cfg_attr(feature="unstable-macros-1-1", feature(proc_macro))]

#[macro_use] extern crate macro_attr;

#[cfg(not(feature="unstable-macros-1-1"))]
#[macro_use] extern crate derive_name_macro;

#[cfg(feature="unstable-macros-1-1")]
#[macro_use] extern crate derive_name_proc;

macro_rules! remove_body {
    (
        (),
        then $cb:ident!$cb_arg:tt,
        $(#[$($attrs:tt)*])*
        struct $name:ident $($_tail:tt)*
    ) => {
        macro_attr_callback! {
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
        macro_attr_callback! {
            $cb!$cb_arg,
            $(#[$($attrs)*])*
            struct $name $($tail)*
        }
    };
}

macro_attr! {
    #[derive(Debug, Name~!)]
    #[remove_body!]
    #[use_secret_alias!(Alucard)]
    struct Alucard;
}

fn main() {
    assert_eq!(format!("{:?}", Alucard), "Alucard");
    assert_eq!(Alucard::name(), "Alucard");
    println!("derive-name-test: Ok ({}).", Alucard::derived_by());
}
