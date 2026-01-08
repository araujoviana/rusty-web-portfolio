// These are some macros I haven't decided where to put.

/// Exports a component to be visible to other files
#[macro_export]
macro_rules! export_comp {
    ($mod:ident,$ty:ident) => {
        pub mod $mod;
        pub use $mod::$ty;
    };
}
