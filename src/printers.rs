// src/printers.rs

//! Macros for trace printing with printing stack offset indentation.

//
// `p`rintln
//

/// `p`rintln!
#[macro_export]
macro_rules! p {
    (
        $($args:tt)*
    ) => {
        println!($($args)*)
    }
}
pub use p;

/// `p`rintln! using stack offset indent [`so()`].
/// Use this to `p`rintln! within a function.
///
/// [`so()`]: crate::stack::so
#[macro_export]
macro_rules! po {
    (
        $($args:tt)*
    ) => {
        print!("{}", $crate::stack::so());
        println!($($args)*)
    }
}
pub use po;

/// `p`rintln! using stack offset indent [`sn()`].
/// Use this as the first `p`rintln! in a function.
///
/// [`sn()`]: crate::stack::sn
#[macro_export]
macro_rules! pn {
    (
        $($args:tt)*
    ) => {
        print!("{}", $crate::stack::sn());
        println!($($args)*)
    }
}
pub use pn;

/// `p`rintln! using stack offset indent [`sx()`].
/// To signify e`x`iting a function.
/// Use this as the last `p`rintln! in a function.
///
/// [`sx()`]: crate::stack::sx
#[macro_export]
macro_rules! px {
    (
        $($args:tt)*
    ) => {
        print!("{}", $crate::stack::sx());
        println!($($args)*)
    }
}
pub use px;

/// `p`rintln! using stack offset indent [`sn͓()`].
/// To signify e`n`tering and e`x`iting a function.
/// Use this as the first+last println! in a function.
/// (useful for short functions).
///
/// [`sn͓()`]: crate::stack::sn͓
#[macro_export]
macro_rules! pn͓ {
    (
        $($args:tt)*
    ) => {
        print!("{}", $crate::stack::sn͓());
        println!($($args)*)
    }
}
pub use pn͓;

/// `p`rintln! using stack `o`ffset indent [`so()`] and current
/// [`f`unction name].
/// Use this to println! within a function.
///
/// [`so()`]: crate::stack::so
/// [`f`unction name]: crate::function_name::function_name
#[macro_export]
macro_rules! pof {
    (
        $($args:tt)*
    ) => {
        print!("{}{}: ", $crate::stack::so(), $crate::function_name::function_name!());
        println!($($args)*)
    }
}
pub use pof;

/// `p`rintln! using stack offset indent [`sn()`] and current
/// [function name].
/// To signify e`n`tering a `f`unction.
/// Use this as the first println! in a function.
///
/// [`sn()`]: crate::stack::sn
/// [function name]: crate::function_name::function_name
#[macro_export]
macro_rules! pnf {
    (
        $($args:tt)*
    ) => {
        print!("{}{}: ", $crate::stack::sn(), $crate::function_name::function_name!());
        println!($($args)*)
    }
}
pub use pnf;

/// `p`rintln! using stack offset indent [`sx()`] and current
/// [function name].
/// To signify e`x`iting a `f`unction.
/// Use this as the last `p`rintln! in a function.
///
/// [`sx()`]: crate::stack::sx
/// [function name]: crate::function_name::function_name
#[macro_export]
macro_rules! pxf {
    (
        $($args:tt)*
    ) => {
        print!("{}{}: ", $crate::stack::sx(), $crate::function_name::function_name!());
        println!($($args)*)
    }
}
pub use pxf;

/// `p`rintln! only in debug builds, using stack offset indent [`sn͓()`]
/// and current [function name].
/// To signify e`n`tering and e`x`iting a `f`unction.
/// Use this as the first+last println! in a function.
///
/// [`sn͓()`]: crate::stack::sn͓
/// [function name]: crate::function_name::function_name
#[macro_export]
macro_rules! pn͓f {
    (
        $($args:tt)*
    ) => {
        print!("{}{}: ", $crate::stack::sn͓(), $crate::function_name::function_name!());
        println!($($args)*)
    }
}
pub use pn͓f;

//
// `e`println
//

/// `e`println!
#[macro_export]
macro_rules! e {
    (
        $($args:tt)*
    ) => {
        eprintln!($($args)*)
    }
}
pub use e;

/// `e`println! using stack offset indent [`so()`].
/// Use this to eprintln! within a function.
///
/// [`so()`]: crate::stack::so
#[macro_export]
macro_rules! eo {
    (
        $($args:tt)*
    ) => {
        eprint!("{}", $crate::stack::so());
        eprintln!($($args)*)
    }
}
pub use eo;

/// `e`println! using stack offset indent [`sn()`].
/// To signify e`n`tering a function.
/// Use this as the first eprintln! in a function.
///
/// [`sn()`]: crate::stack::sn
#[macro_export]
macro_rules! en {
    (
        $($args:tt)*
    ) => {
        eprint!("{}", $crate::stack::sn());
        eprintln!($($args)*)
    }
}
pub use en;

/// `e`println! using stack offset indent [`sx()`].
/// To signify e`x`iting a function.
/// Use this as the last `e`println! in a function.
///
/// [`sx()`]: crate::stack::sx
#[macro_export]
macro_rules! ex {
    (
        $($args:tt)*
    ) => {
        eprint!("{}", $crate::stack::sx());
        eprintln!($($args)*)
    }
}
pub use ex;

/// `e`println! using stack offset indent [`sn͓()`].
/// To signify e`n`tering and e`x`iting a `f`unction.
/// Use this as the first+last eprintln! in a function
/// (useful for short functions).
///
/// [`sn͓()`]: crate::stack::sn͓
#[macro_export]
macro_rules! en͓ {
    (
        $($args:tt)*
    ) => {
        eprint!("{}", $crate::stack::sn͓());
        eprintln!($($args)*)
    }
}
pub use en͓;

/// `e`println! using stack `o`ffset indent [`so()`] and current
/// [function name].
/// Use this to `e`println! within a function.
///
/// [`so()`]: crate::stack::so
/// [function name]: crate::function_name::function_name
#[macro_export]
macro_rules! eof {
    (
        $($args:tt)*
    ) => {
        eprint!("{}{}: ", $crate::stack::so(), $crate::function_name::function_name!());
        eprintln!($($args)*)
    }
}
pub use eof;

/// `e`println! using stack offset indent [`sn()`] and current
/// [function name].
/// To signify e`n`tering a `f`unction.
/// Use this as the first eprintln! in a function.
///
/// [`sn()`]: crate::stack::sn
/// [function name]: crate::function_name::function_name
#[macro_export]
macro_rules! enf {
    (
        $($args:tt)*
    ) => {
        eprint!("{}{}: ", $crate::stack::sn(), $crate::function_name::function_name!());
        eprintln!($($args)*)
    }
}
pub use enf;

/// `e`println! using stack offset indent [`sx()`] and current
/// [function name].
/// To signify e`x`iting a `f`unction.
/// Use this as the last `e`println! in a function.
///
/// [`sx()`]: crate::stack::sx
/// [function name]: crate::function_name::function_name
#[macro_export]
macro_rules! exf {
    (
        $($args:tt)*
    ) => {
        eprint!("{}{}: ", $crate::stack::sx(), $crate::function_name::function_name!());
        eprintln!($($args)*)
    }
}
pub use exf;

/// `e`println! only in debug builds, using stack offset indent [`sn͓()`]
/// and current [function name].
/// To signify e`n`tering and e`x`iting a `f`unction.
///
/// [`sn͓()`]: crate::stack::sn͓
/// [function name]: crate::function_name::function_name
#[macro_export]
macro_rules! en͓f {
    (
        $($args:tt)*
    ) => {
        eprint!("{}{}: ", $crate::stack::sn͓(), $crate::function_name::function_name!());
        eprintln!($($args)*)
    }
}
pub use en͓f;

//
// `d`ebug `p`rintln
//

/// `d`ebug `p`rintln!
#[macro_export]
macro_rules! dp {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        eprintln!($($args)*)
    }
}
pub use dp;

/// `d`ebug `p`rintln! using stack `o`ffset indent [`so()`].
/// Use this within a function.
///
/// [`so()`]: crate::stack::so
#[macro_export]
macro_rules! dpo {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        eprint!("{}", $crate::stack::so());
        #[cfg(any(debug_assertions,test))]
        eprintln!($($args)*)
    }
}
pub use dpo;

/// `d`ebug `p`rintln! using stack offset indent [`sn()`].
/// To signify e`n`tering a function.
/// Use this in the first `d`ebug `p`rintln! of a function.
///
/// [`sn()`]: crate::stack::sn
#[macro_export]
macro_rules! dpn {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        eprint!("{}", $crate::stack::sn());
        #[cfg(any(debug_assertions,test))]
        eprintln!($($args)*)
    }
}
pub use dpn;

/// `d`ebug `p`rintln! using stack offset indent [`sx()`].
/// To signify e`x`iting a function.
/// Use this as the last debug println! of a function.
///
/// [`sx()`]: crate::stack::sx
#[macro_export]
macro_rules! dpx {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        eprint!("{}", $crate::stack::sx());
        #[cfg(any(debug_assertions,test))]
        eprintln!($($args)*)
    }
}
pub use dpx;

/// `d`ebug `p`rintln! using stack offset indent [`sn͓()`].
/// To signify e`n`tering and e`x`iting a function.
/// Use this as the first+last debug println! of a function
/// (useful for short functions).
///
/// [`sn͓()`]: crate::stack::sn͓
#[macro_export]
macro_rules! dpn͓ {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        eprint!("{}", $crate::stack::sn͓());
        #[cfg(any(debug_assertions,test))]
        eprintln!($($args)*)
    }
}
pub use dpn͓;

/// `d`ebug `p`rintln! using stack `o`ffset indent [`so()`] and
/// current [`f`unction name].
/// Use this to `d`ebug `p`rintln! within a function.
///
/// [`so()`]: crate::stack::so
/// [`f`unction name]: crate::function_name::function_name
#[macro_export]
macro_rules! dpof {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        eprint!("{}{}: ", $crate::stack::so(), $crate::function_name::function_name!());
        #[cfg(any(debug_assertions,test))]
        eprintln!($($args)*)
    }
}
pub use dpof;

/// `d`ebug `p`rintln! using stack offset indent [`sn()`] and current
/// [function name].
/// To signify e`n`tering a `f`unction.
/// Use this as the first debug println! of a function.
///
/// [`sn()`]: crate::stack::sn
/// [function name]: crate::function_name::function_name
#[macro_export]
macro_rules! dpnf {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        eprint!("{}{}: ", $crate::stack::sn(), $crate::function_name::function_name!());
        #[cfg(any(debug_assertions,test))]
        eprintln!($($args)*)
    }
}
pub use dpnf;

/// `d`ebug `p`rintln! using stack offset indent [`sx()`] and current
/// [function name].
/// To signify e`x`iting a `f`unction.
/// Use this as the last debug println! of a function.
///
/// [`sx()`]: crate::stack::sx
/// [function name]: crate::function_name::function_name
#[macro_export]
macro_rules! dpxf {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        eprint!("{}{}: ", $crate::stack::sx(), $crate::function_name::function_name!());
        #[cfg(any(debug_assertions,test))]
        eprintln!($($args)*)
    }
}
pub use dpxf;

/// `d`ebug `p`rintln! only in debug builds, using stack offset indent
/// [`sn͓()`] and current [function name].
/// To signify e`n`tering and e`x`iting a `f`unction.
/// Use this as the first+last debug println! of a function
/// (useful for short functions).
///
/// [`sn͓()`]: crate::stack::sn͓
/// [function name]: crate::function_name::function_name
#[macro_export]
macro_rules! dpn͓f {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        eprint!("{}{}: ", $crate::stack::sn͓(), $crate::function_name::function_name!());
        #[cfg(any(debug_assertions,test))]
        eprintln!($($args)*)
    }
}
pub use dpn͓f;

//
// `d`ebug `e`println
//

/// `d`ebug `e`println!
#[macro_export]
macro_rules! de {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        eprintln!($($args)*)
    }
}
pub use de;

/// `d`ebug `e`println! using stack `o`ffset indent [`so()`].
/// Use this within a function.
///
/// [`so()`]: crate::stack::so
#[macro_export]
macro_rules! deo {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        eprint!("{}", $crate::stack::so());
        #[cfg(any(debug_assertions,test))]
        eprintln!($($args)*)
    }
}
pub use deo;

/// `d`ebug `e`println! using stack offset indent [`sn()`].
/// To signify e`n`tering a function.
/// Use this in the first debug eprintln! of a function.
///
/// [`sn()`]: crate::stack::sn
#[macro_export]
macro_rules! den {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        eprint!("{}", $crate::stack::sn());
        #[cfg(any(debug_assertions,test))]
        eprintln!($($args)*)
    }
}
pub use den;

/// `d`ebug `e`println! using stack offset indent [`sx()`].
/// To signify e`x`iting a function.
/// Use this as the last debug eprintln! of a function.
///
/// [`sx()`]: crate::stack::sx
#[macro_export]
macro_rules! dex {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        eprint!("{}", $crate::stack::sx());
        #[cfg(any(debug_assertions,test))]
        eprintln!($($args)*)
    }
}
pub use dex;

/// `d`ebug `e`println! using stack offset indent [`sn͓()`].
/// To signify e`n`tering and e`x`iting a function.
/// Use this as the first+last debug eprintln! of a function
/// (useful for short functions).
///
/// [`sn͓()`]: crate::stack::sn͓
#[macro_export]
macro_rules! den͓ {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        eprint!("{}", $crate::stack::sn͓());
        #[cfg(any(debug_assertions,test))]
        eprintln!($($args)*)
    }
}
pub use den͓;

/// `d`ebug `e`println! using stack `o`ffset indent [`so()`] and
/// current [`f`unction name].
/// Use this to debug eprintln! within a function.
///
/// [`so()`]: crate::stack::so
/// [`f`unction name]: crate::function_name::function_name
#[macro_export]
macro_rules! deof {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        eprint!("{}{}: ", $crate::stack::so(), $crate::function_name::function_name!());
        #[cfg(any(debug_assertions,test))]
        eprintln!($($args)*)
    }
}
pub use deof;

/// `d`ebug `e`println! using stack offset indent [`sn()`] and current
/// [function name].
/// To signify e`n`tering a `f`unction.
/// Use this as the first debug eprintln! of a function.
///
/// [`sn()`]: crate::stack::sn
/// [function name]: crate::function_name::function_name
#[macro_export]
macro_rules! denf {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        eprint!("{}{}: ", $crate::stack::sn(), $crate::function_name::function_name!());
        #[cfg(any(debug_assertions,test))]
        eprintln!($($args)*)
    }
}
pub use denf;

/// `d`ebug `e`println! using stack offset indent [`sx()`] and current
/// [function name].
/// To signify e`x`iting a `f`unction.
/// Use this as the last debug eprintln! of a function.
///
/// [`sx()`]: crate::stack::sx
/// [function name]: crate::function_name::function_name
#[macro_export]
macro_rules! dexf {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        eprint!("{}{}: ", $crate::stack::sx(), $crate::function_name::function_name!());
        #[cfg(any(debug_assertions,test))]
        eprintln!($($args)*)
    }
}
pub use dexf;

/// `d`ebug `e`println! only in debug builds, using stack offset indent
/// [`sn͓()`] and current [function name].
/// To signify e`n`tering and e`x`iting a `f`unction.
/// Use this as the first+last debug eprintln! of a function
/// (useful for short functions).
///
/// [`sn͓()`]: crate::stack::sn͓
/// [function name]: crate::function_name::function_name
#[macro_export]
macro_rules! den͓f {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        eprint!("{}{}: ", $crate::stack::sn͓(), $crate::function_name::function_name!());
        #[cfg(any(debug_assertions,test))]
        eprintln!($($args)*)
    }
}
pub use den͓f;

//
// tests
//

#[cfg(test)]
mod tests {
    use crate::function_name::function_name_full;
    use crate::stack::stack_offset_set;

    // `p`rintln tests

    #[test]
    fn test_p() {
        println!();
        p!("this printed line, with arg \"{}\"", function_name_full!());
        p!();
        println!();
    }

    #[test]
    fn test_po() {
        stack_offset_set(Some(2));
        println!();
        po!(
            "this printed line should be indented with arg \"{}\"",
            function_name_full!()
        );
        po!();
        println!();
    }

    #[test]
    fn test_pn() {
        stack_offset_set(Some(2));
        println!();
        pn!(
            "this printed line should be indented with arg \"{}\"",
            function_name_full!()
        );
        pn!();
        println!();
    }

    #[test]
    fn test_px() {
        stack_offset_set(Some(2));
        println!();
        px!(
            "this printed line should be indented with arg \"{}\"",
            function_name_full!()
        );
        px!();
        println!();
    }

    #[test]
    fn test_pn͓() {
        stack_offset_set(Some(2));
        println!();
        pn͓!(
            "this printed line should be indented with arg \"{}\"",
            function_name_full!()
        );
        pn͓!();
        println!();
    }

    #[test]
    fn test_pof() {
        stack_offset_set(Some(2));
        println!();
        pof!(
            "this printed line should be indented and preceded with function name 'test_pof', with arg \"{}\"",
            function_name_full!()
        );
        pof!();
        println!();
    }

    #[test]
    fn test_pnf() {
        stack_offset_set(Some(2));
        println!();
        pnf!(
            "this printed line should be indented and preceded with function name 'test_pnf', with arg \"{}\"",
            function_name_full!()
        );
        pnf!();
        println!();
    }

    #[test]
    fn test_pxf() {
        stack_offset_set(Some(2));
        println!();
        pxf!(
            "this printed line should be indented and preceded with function name 'test_pxf', with arg \"{}\"",
            function_name_full!()
        );
        pxf!();
        println!();
    }

    #[test]
    fn test_pn͓f() {
        stack_offset_set(Some(2));
        println!();
        pn͓f!(
            "this printed line should be indented and preceded with function name 'test_pn͓f', with arg \"{}\"",
            function_name_full!()
        );
        pn͓f!();
        println!();
    }

    #[test]
    fn test_ponxn͓f() {
        stack_offset_set(Some(2));
        println!();
        pnf!("pnf!");
        pof!("pof!");
        pn͓f!("pn͓f!");
        pxf!("pxf!");
        println!();
    }

    // `e`println tests

    #[test]
    fn test_e() {
        eprintln!();
        e!("this printed line, with arg \"{}\"", function_name_full!());
        e!();
        eprintln!();
    }

    #[test]
    fn test_eo() {
        stack_offset_set(Some(2));
        eprintln!();
        eo!(
            "this printed line should be indented with arg \"{}\"",
            function_name_full!()
        );
        eo!();
        eprintln!();
    }

    #[test]
    fn test_en() {
        stack_offset_set(Some(2));
        eprintln!();
        en!(
            "this printed line should be indented with arg \"{}\"",
            function_name_full!()
        );
        en!();
        eprintln!();
    }

    #[test]
    fn test_ex() {
        stack_offset_set(Some(2));
        eprintln!();
        ex!(
            "this printed line should be indented with arg \"{}\"",
            function_name_full!()
        );
        ex!();
        eprintln!();
    }

    #[test]
    fn test_en͓() {
        stack_offset_set(Some(2));
        eprintln!();
        en͓!(
            "this printed line should be indented with arg \"{}\"",
            function_name_full!()
        );
        en͓!();
        eprintln!();
    }

    #[test]
    fn test_eonxn͓() {
        stack_offset_set(Some(2));
        eprintln!();
        en!("en!");
        eo!("eo!");
        en͓!("en͓!");
        ex!("ex!");
        eprintln!();
    }

    #[test]
    fn test_eof() {
        stack_offset_set(Some(2));
        eprintln!();
        eof!(
            "this printed line should be indented and preceded with function name 'test_eof', with arg \"{}\"",
            function_name_full!()
        );
        eof!();
        eprintln!();
    }

    #[test]
    fn test_enf() {
        stack_offset_set(Some(2));
        eprintln!();
        enf!(
            "this printed line should be indented and preceded with function name 'test_enf', with arg \"{}\"",
            function_name_full!()
        );
        enf!();
        eprintln!();
    }

    #[test]
    fn test_exf() {
        stack_offset_set(Some(2));
        eprintln!();
        exf!(
            "this printed line should be indented and preceded with function name 'test_exf', with arg \"{}\"",
            function_name_full!()
        );
        exf!();
        eprintln!();
    }

    #[test]
    fn test_en͓f() {
        stack_offset_set(Some(2));
        eprintln!();
        en͓f!(
            "this printed line should be indented and preceded with function name 'test_en͓f', with arg \"{}\"",
            function_name_full!()
        );
        en͓f!();
        eprintln!();
    }

    #[test]
    fn test_eonxn͓f() {
        stack_offset_set(Some(2));
        eprintln!();
        enf!("enf!");
        eof!("eof!");
        en͓f!("en͓f!");
        exf!("exf!");
        eprintln!();
    }

    // `d`ebug `p`rintln tests

    #[test]
    fn test_dp() {
        println!();
        dp!("this printed line, with arg \"{}\"", function_name_full!());
        dp!();
        println!();
    }

    #[test]
    fn test_dpo() {
        stack_offset_set(Some(2));
        println!();
        dpo!(
            "this printed line should be indented with arg \"{}\"",
            function_name_full!()
        );
        dpo!();
        println!();
    }

    #[test]
    fn test_dpn() {
        stack_offset_set(Some(2));
        println!();
        dpn!(
            "this printed line should be indented with arg \"{}\"",
            function_name_full!()
        );
        dpn!();
        println!();
    }

    #[test]
    fn test_dpx() {
        stack_offset_set(Some(2));
        println!();
        dpx!(
            "this printed line should be indented with arg \"{}\"",
            function_name_full!()
        );
        dpx!();
        println!();
    }

    #[test]
    fn test_dpn͓() {
        stack_offset_set(Some(2));
        println!();
        dpn͓!(
            "this printed line should be indented with arg \"{}\"",
            function_name_full!()
        );
        dpn͓!();
        println!();
    }

    #[test]
    fn test_dponxn͓() {
        stack_offset_set(Some(2));
        println!();
        dpn!("dpn!");
        dpo!("dpo!");
        dpn͓!("dpn͓!");
        dpx!("dpx!");
        println!();
    }

    #[test]
    fn test_dpof() {
        stack_offset_set(Some(2));
        println!();
        dpof!(
            "this printed line should be indented and preceded with function name 'test_dpof', with arg \"{}\"",
            function_name_full!()
        );
        dpof!();
        println!();
    }

    #[test]
    fn test_dpnf() {
        stack_offset_set(Some(2));
        println!();
        dpnf!(
            "this printed line should be indented and preceded with function name 'test_dpnf', with arg \"{}\"",
            function_name_full!()
        );
        dpnf!();
        println!();
    }

    #[test]
    fn test_dpxf() {
        stack_offset_set(Some(2));
        println!();
        dpxf!(
            "this printed line should be indented and preceded with function name 'test_dpxf', with arg \"{}\"",
            function_name_full!()
        );
        dpxf!();
        println!();
    }

    #[test]
    fn test_dpn͓f() {
        stack_offset_set(Some(2));
        println!();
        dpn͓f!(
            "this printed line should be indented and preceded with function name 'test_dpn͓f', with arg \"{}\"",
            function_name_full!()
        );
        dpn͓f!();
        println!();
    }

    #[test]
    fn test_dponxn͓f() {
        stack_offset_set(Some(2));
        println!();
        dpnf!("dpnf!");
        dpof!("dpof!");
        dpn͓f!("dpn͓f!");
        dpxf!("dpxf!");
        println!();
    }

    // `d`ebug `e`println tests

    #[test]
    fn test_de() {
        eprintln!();
        de!("this printed line, with arg \"{}\"", function_name_full!());
        de!();
        eprintln!();
    }

    #[test]
    fn test_deo() {
        stack_offset_set(Some(2));
        eprintln!();
        deo!(
            "this printed line should be indented with arg \"{}\"",
            function_name_full!()
        );
        deo!();
        eprintln!();
    }

    #[test]
    fn test_den() {
        stack_offset_set(Some(2));
        eprintln!();
        den!(
            "this printed line should be indented with arg \"{}\"",
            function_name_full!()
        );
        den!();
        eprintln!();
    }

    #[test]
    fn test_dex() {
        stack_offset_set(Some(2));
        eprintln!();
        dex!(
            "this printed line should be indented with arg \"{}\"",
            function_name_full!()
        );
        dex!();
        eprintln!();
    }

    #[test]
    fn test_den͓() {
        stack_offset_set(Some(2));
        eprintln!();
        den͓!(
            "this printed line should be indented with arg \"{}\"",
            function_name_full!()
        );
        den͓!();
        eprintln!();
    }

    #[test]
    fn test_deonxn͓() {
        stack_offset_set(Some(2));
        eprintln!();
        den!("den!");
        deo!("deo!");
        den͓!("den͓!");
        dex!("dex!");
        eprintln!();
    }

    #[test]
    fn test_deof() {
        stack_offset_set(Some(2));
        eprintln!();
        deof!(
            "this printed line should be indented and preceded with function name 'test_deof', with arg \"{}\"",
            function_name_full!()
        );
        deof!();
        eprintln!();
    }

    #[test]
    fn test_denf() {
        stack_offset_set(Some(2));
        eprintln!();
        denf!(
            "this printed line should be indented and preceded with function name 'test_denf', with arg \"{}\"",
            function_name_full!()
        );
        denf!();
        eprintln!();
    }

    #[test]
    fn test_dexf() {
        stack_offset_set(Some(2));
        eprintln!();
        dexf!(
            "this printed line should be indented and preceded with function name 'test_dexf', with arg \"{}\"",
            function_name_full!()
        );
        dexf!();
        eprintln!();
    }

    #[test]
    fn test_den͓f() {
        stack_offset_set(Some(2));
        eprintln!();
        den͓f!(
            "this printed line should be indented and preceded with function name 'test_den͓f', with arg \"{}\"",
            function_name_full!()
        );
        den͓f!();
        eprintln!();
    }

    #[test]
    fn test_deonxn͓f() {
        stack_offset_set(Some(2));
        eprintln!();
        denf!("denf!");
        deof!("deof!");
        den͓f!("den͓f!");
        dexf!("dexf!");
        eprintln!();
    }
}
