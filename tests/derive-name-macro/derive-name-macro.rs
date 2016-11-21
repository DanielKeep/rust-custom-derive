/*
Copyright ⓒ 2016 macro-attr contributors.

Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
#[macro_export]
macro_rules! Name {
    (
        ()
        struct $name:ident $($_tail:tt)*
    ) => {
        impl $name {
            pub fn name() -> &'static str {
                stringify!($name)
            }

            pub fn derived_by() -> &'static str {
                "macro-by-example"
            }
        }
    };
}
