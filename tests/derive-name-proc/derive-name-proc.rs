/*
Copyright ⓒ 2016 macro-attr contributors.

Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
#![feature(proc_macro, proc_macro_lib)]
#![crate_type="proc-macro"]

#[macro_use] extern crate quote;

extern crate proc_macro;
extern crate syn;

use self::proc_macro::TokenStream;

#[proc_macro_derive(Name)]
pub fn name(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let ast = syn::parse_macro_input(&input).unwrap();
    let name = &ast.ident;
    let name_str = name.to_string();

    let out = quote! {
        impl #name {
            pub fn name() -> &'static str {
                #name_str
            }

            pub fn derived_by() -> &'static str {
                "procedural macro"
            }
        }
    };

    out.to_string().parse().unwrap()
}
