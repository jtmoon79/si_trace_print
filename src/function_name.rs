// src/function_name.rs

//! Macros to derive the current function name.
//!
//! These macros are exported in case they are useful outside this crate.
//! Library users will probably want to use macros provided in [`printers`].
//!
//! [`printers`]: crate::printers

/// Return the current the current function name as a `&'static str`,
/// e.g. `"my_func"`.
///
/// `function_name` must be a macro (and not a function) to reliably use
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
        // return a slice of `name`
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

/// Return the current function name plus preceding namespaces as a
/// `&'static str`.
///
/// For example,
/// - `function_name_plus!(1)` returns `"my_struct::my_func"`
/// - `function_name_plus!(2)` returns `"my_mod::my_struct::my_func"`
///
/// `function_name_plus!(0)` is equivalent to [`function_name!()`].
///
/// `function_name_plus` must be a macro (and not a function) to reliably use
/// `std::any::type_name::<T>()` introspection.
///
/// [`function_name!()`]: function_name
#[macro_export]
macro_rules! function_name_plus {
    ($plus:literal) => {{
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

        let mut plus_: usize = $plus + 1;
        let mut at = name.len();
        while plus_ > 0 {
            let rfind: Option<usize> = name[..at].rfind(SPLIT);
            let len = name[..at].len();
            // get the index offset of the first rfind
            at = match rfind {
                Some(index) => {
                    if index < len {
                        index
                    } else {
                        // this `else` should never happen but do not panic
                        len
                    }
                }
                None => {
                    // this `None` should never happen but fallback to full name
                    len
                }
            };
            plus_ -= 1;
        }
        if at + SPLIT_LEN < name.len() {
            &name[at + SPLIT_LEN..]
        } else if at < name.len() {
            // this should not happen but do not panic
            &name[at..]
        } else {
            // this should not happen but do not panic
            &name
        }
    }};
}
pub use function_name_plus;

/// Return the current current function name full path as a `&'static str`,
/// e.g. `"my_lib::my_mod::my_func"`.
///
/// `function_name_full` must be a macro (and not a function) to reliably use
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
    use super::{function_name, function_name_full, function_name_plus};

    #[test]
    fn test_function_name() {
        eprintln!("function_name! \"{}\"", function_name!());
        fn func1() {
            eprintln!("function_name! \"{}\"", function_name!());
            assert_eq!("func1", function_name!());
        }
        func1();
        assert_eq!("test_function_name", function_name!());
    }

    #[test]
    fn test_function_name_function_name_plus0() {
        assert_eq!(function_name!(), function_name_plus!(0));
    }

    #[test]
    fn test_function_name_plus0() {
        eprintln!("function_name_plus!(0) \"{}\"", function_name_plus!(0));
        fn func1() {
            eprintln!("function_name_plus!(0) \"{}\"", function_name_plus!(0));
            assert_eq!("func1", function_name_plus!(0));
        }
        func1();
        assert_eq!("test_function_name_plus0", function_name_plus!(0));
    }

    #[test]
    fn test_function_name_plus1() {
        eprintln!("function_name_plus!(1) \"{}\"", function_name_plus!(1));
        fn func1() {
            eprintln!("function_name_plus!(1) \"{}\"", function_name_plus!(1));
            assert_eq!("test_function_name_plus1::func1", function_name_plus!(1));
        }
        func1();
        assert_eq!("tests::test_function_name_plus1", function_name_plus!(1));
    }

    #[test]
    fn test_function_name_plus2() {
        eprintln!("function_name_plus!(2) \"{}\"", function_name_plus!(2));
        fn func1() {
            eprintln!("function_name_plus!(2) \"{}\"", function_name_plus!(2));
            assert_eq!("tests::test_function_name_plus2::func1", function_name_plus!(2));
        }
        func1();
        assert_eq!("function_name::tests::test_function_name_plus2", function_name_plus!(2));
    }

    // test `function_name_plus!` within another namespace layer
    mod more_tests {

        #[test]
        fn test_function_name_plus0() {
            eprintln!("function_name_plus!(0) \"{}\"", function_name_plus!(0));
            fn func1() {
                eprintln!("function_name_plus!(0) \"{}\"", function_name_plus!(0));
                assert_eq!("func1", function_name_plus!(0));
            }
            func1();
            assert_eq!("test_function_name_plus0", function_name_plus!(0));
        }

        #[test]
        fn test_function_name_plus1() {
            eprintln!("function_name_plus!(1) \"{}\"", function_name_plus!(1));
            fn func1() {
                eprintln!("function_name_plus!(1) \"{}\"", function_name_plus!(1));
                assert_eq!("test_function_name_plus1::func1", function_name_plus!(1));
            }
            func1();
            assert_eq!("more_tests::test_function_name_plus1", function_name_plus!(1));
        }

        #[test]
        fn test_function_name_plus2() {
            eprintln!("function_name_plus!(2) \"{}\"", function_name_plus!(2));
            fn func1() {
                eprintln!("function_name_plus!(2) \"{}\"", function_name_plus!(2));
                assert_eq!("more_tests::test_function_name_plus2::func1", function_name_plus!(2));
            }
            func1();
            assert_eq!("tests::more_tests::test_function_name_plus2", function_name_plus!(2));
        }
    }

    #[test]
    fn test_function_name_full() {
        eprintln!("function_name_full! \"{}\"", function_name_full!());
        fn func1() {
            eprintln!("function_name_full! \"{}\"", function_name_full!());
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
