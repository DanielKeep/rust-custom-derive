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
