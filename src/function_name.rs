// src/function_name.rs

//! Macros to derive the current function name.

/// Return the current the current function name as a `&'static str`,
/// e.g. `"my_func"`.
///
/// `function_name` must be a macro (and not a function) to use
/// `std::any::type_name::<T>()` introspection.
#[macro_export]
macro_rules! function_name {
    () => {{
        const fn f() {}
        fn type_name_of<T>(_: &T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name: &'static str = type_name_of(&f);
        // name will be `"my_lib::my_mod::my_func::f"`
        // slice off the trailing `"::f"`
        let name: &'static str = &name[..name.len() - 3];
        const SPLIT: &str = "::";
        const SPLIT_LEN: usize = SPLIT.len();
        let rfind: Option<usize> = name.rfind(SPLIT);
        let len = name.len();
        match rfind {
            Some(index) => {
                if index + SPLIT_LEN < len {
                    &name[index + SPLIT_LEN..]
                } else {
                    // this `else` should never happen... but if it does then
                    // do not panic
                    &name[index..]
                }
            }
            None => {
                // this `None` should never happen... but if it does then
                // fallback to full name
                name
            }
        }
    }};
}
pub use function_name;

/// Return the current current function name full path as a `&'static str`,
/// e.g. `"my_lib::my_mod::my_func"`.
///
/// `function_name_full` must be a macro (and not a function) to use
/// `std::any::type_name::<T>()` introspection.
///
/// Credit to <https://github.com/popzxc/stdext-rs/blob/2179f94475f925a2eacdc2f2408d7ab352d0052c/src/macros.rs#L44-L74>
#[macro_export]
macro_rules! function_name_full {
    () => {{
        const fn f() {}
        fn type_name_of<T>(_: &T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name: &'static str = type_name_of(&f);
        // name will be `"my_lib::my_mod::my_func::f"`
        // slice off the trailing `"::f"`
        let name: &'static str = &name[..name.len() - 3];

        name
    }};
}
pub use function_name_full;

#[cfg(test)]
mod tests {
    use super::{function_name, function_name_full};

    #[test]
    fn test_function_name() {
        eprintln!("function_name \"{}\"", function_name!());
        fn func1() {
            eprintln!("function_name \"{}\"", function_name!());
            assert_eq!("func1", function_name!());
        }
        func1();
        assert_eq!("test_function_name", function_name!());
    }

    #[test]
    fn test_function_name_full() {
        eprintln!("function_name_full \"{}\"", function_name_full!());
        fn func1() {
            eprintln!("function_name_full \"{}\"", function_name_full!());
            assert_eq!(
                "si_trace_print::function_name::tests::test_function_name_full::func1",
                function_name_full!()
            );
        }
        func1();
        assert_eq!(
            "si_trace_print::function_name::tests::test_function_name_full",
            function_name_full!()
        );
    }
}
