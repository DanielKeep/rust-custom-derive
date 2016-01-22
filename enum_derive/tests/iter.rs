/*
Copyright ⓒ 2015 rust-custom-derive contributors.

Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
#[macro_use] extern crate custom_derive;
#[macro_use] extern crate enum_derive;

custom_derive! {
    #[derive(Debug, PartialEq, IterVariants(GetVariants), IterVariantNames(GetVariantNames))]
    pub enum Get {
        Up,
        /// And
        Down,
        /** And */
        AllAround
    }
}

custom_derive! {
    #[derive(IterVariants(NadaVariants), IterVariantNames(NadaVariantNames))]
    enum Nada {}
}

#[test]
fn test_enum_iterator() {
    let vs: Vec<_> = Get::iter_variant_names().zip(Get::iter_variants()).collect();
    assert_eq!(&*vs, &[("Up", Get::Up), ("Down", Get::Down), ("AllAround", Get::AllAround)]);

    assert_eq!(Nada::iter_variants().collect::<Vec<_>>().len(), 0);
    assert_eq!(Nada::iter_variant_names().collect::<Vec<_>>().len(), 0);
}
