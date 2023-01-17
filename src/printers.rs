// src/printers.rs

//! Macros to print trace statements with stack offset indentation.
//!
//! Library users should use these macros.
//!
//! These macros call [`sn`], [`so`], [`sx`], [`sñ`], for the preprinted
//! indentation and signifier symbol.
//!
//! [`sn`]: crate::stack::sn
//! [`so`]: crate::stack::so
//! [`sx`]: crate::stack::sx
//! [`sñ`]: crate::stack::sñ

//
// `p`rintln
//

/// **p**rintln!
///
/// For completeness, wrap [`println!`].
///
/// ```rust
/// use si_trace_print::{pfn, pfx};
/// use si_trace_print::p;
/// fn func1() {
///     p!("hello");
/// }
/// fn main() {
///     pfn!();
///     func1();
///     pfx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
/// hello
/// ←main:
/// ```
///
/// [`println!`]: println!
#[macro_export]
macro_rules! p {
    (
        $($args:tt)*
    ) => {
        // for consistency with other macros, invoke setting the
        // "original" stack depth via `so`
        $crate::stack::so();
        println!($($args)*)
    }
}
pub use p;

/// **p**rintln! with **o**ffset.
///
/// To signify printing within a function.
/// Use this to [`println!`] within a function.
///
/// ```rust
/// use si_trace_print::{pfn, pfx};
/// use si_trace_print::po;
/// fn func1() {
///     po!("hello");
/// }
/// fn main() {
///     pfn!();
///     func1();
///     pfx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///     hello
/// ←main:
/// ```
///
/// Uses [`so()`].
///
/// [`println!`]: println!
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

/// **p**rintln! when e**n**tering.
///
/// To signify entering a function.
/// Use this as the first [`println!`] in a function.
///
/// ```rust
/// use si_trace_print::{pfn, pfx};
/// use si_trace_print::pn;
/// fn func1() {
///     pn!("hello");
/// }
/// fn main() {
///     pfn!();
///     func1();
///     pfx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///    →hello
/// ←main:
/// ```
///
/// Uses [`sn()`].
///
/// [`println!`]: println!
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

/// **p**rintln! when e**x**iting.
///
/// To signify exiting a function.
/// Use this as the last [`println!`] in a function.
///
/// ```rust
/// use si_trace_print::{pfn, pfx};
/// use si_trace_print::px;
/// fn func1() {
///     px!("hello");
/// }
/// fn main() {
///     pfn!();
///     func1();
///     pfx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///    ←hello
/// ←main:
/// ```
///
/// Uses [`sx()`].
///
/// [`println!`]: println!
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

/// **p**rintln! when e**n**tering and e**x**iting.
///
/// To signify entering and exiting a function.
/// Use this as the only [`println!`] in a function.
/// Most suited for short functions.
///
/// ```rust
/// use si_trace_print::{pfn, pfx};
/// use si_trace_print::pñ;
/// fn func1() {
///     pñ!("hello");
/// }
/// fn main() {
///     pfn!();
///     func1();
///     pfx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///    ↔hello
/// ←main:
/// ```
///
/// Uses [`sñ()`].
///
/// [`println!`]: println!
/// [`sñ()`]: crate::stack::sñ
#[macro_export]
macro_rules! pñ {
    (
        $($args:tt)*
    ) => {
        print!("{}", $crate::stack::sñ());
        println!($($args)*)
    }
}
pub use pñ;

/// **p**rintln! in a **f**unction with **o**ffset.
///
/// To signify printing within a function.
/// Use to [`println!`] in a [function].
///
/// ```rust
/// use si_trace_print::{pfn, pfx};
/// use si_trace_print::pfo;
/// fn func1() {
///     pfo!("hello");
/// }
/// fn main() {
///     pfn!();
///     func1();
///     pfx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///     func1: hello
/// ←main:
/// ```
///
/// Uses [`so()`].
///
/// [`println!`]: println!
/// [function]: crate::function_name::function_name
/// [`so()`]: crate::stack::so
#[macro_export]
macro_rules! pfo {
    (
        $($args:tt)*
    ) => {
        print!("{}{}: ", $crate::stack::so(), $crate::function_name::function_name!());
        println!($($args)*)
    }
}
pub use pfo;

/// **p**rintln! in a **f**unction when e**n**tering.
///
/// To signify entering a function.
/// Use this as the first [`println!`] in a [function].
///
/// ```rust
/// use si_trace_print::{pfn, pfx};
/// fn func1() {
///     pfn!("hello");
/// }
/// fn main() {
///     pfn!();
///     func1();
///     pfx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///    →func1: hello
/// ←main:
/// ```
///
/// Uses [`sn()`].
///
/// [`println!`]: println!
/// [function]: crate::function_name::function_name
/// [`sn()`]: crate::stack::sn
#[macro_export]
macro_rules! pfn {
    (
        $($args:tt)*
    ) => {
        print!("{}{}: ", $crate::stack::sn(), $crate::function_name::function_name!());
        println!($($args)*)
    }
}
pub use pfn;

/// **p**rintln! in a **f**unction when e**x**iting.
///
/// To signify exiting a function.
/// Use this as the last [`println!`] in a [function].
///
/// ```rust
/// use si_trace_print::{pfn, pfx};
/// fn func1() {
///     pfn!("hello");
/// }
/// fn main() {
///     pfn!();
///     func1();
///     pfx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///    ←func1: hello
/// ←main:
/// ```
///
/// Uses [`sx()`].
///
/// [`println!`]: println!
/// [function]: crate::function_name::function_name
/// [`sx()`]: crate::stack::sx
#[macro_export]
macro_rules! pfx {
    (
        $($args:tt)*
    ) => {
        print!("{}{}: ", $crate::stack::sx(), $crate::function_name::function_name!());
        println!($($args)*)
    }
}
pub use pfx;

/// **p**rintln! in a **f**unction when e**n**tering and e**x**iting.
///
/// To signify entering and exiting a function.
/// Use this as the only [`println!`] in a [function].
/// Most suited for short functions.
///
/// ```rust
/// use si_trace_print::{pfn, pfx};
/// use si_trace_print::pfñ;
/// fn func1() {
///     pfñ!("hello");
/// }
/// fn main() {
///     pfn!();
///     func1();
///     pfx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///    ↔func1: hello
/// ←main:
/// ```
///
/// Uses [`sñ()`].
///
/// [`println!`]: println!
/// [function]: crate::function_name::function_name
/// [`sñ()`]: crate::stack::sñ
#[macro_export]
macro_rules! pfñ {
    (
        $($args:tt)*
    ) => {
        print!("{}{}: ", $crate::stack::sñ(), $crate::function_name::function_name!());
        println!($($args)*)
    }
}
pub use pfñ;

//
// print with **one** namespace level
//

/// **p**rintln! in a **f**unction plus **one** namespace level with
/// **o**ffset.
///
/// To signify printing within a function.
/// Use to [`println!`] in a [function].
///
/// ```rust
/// use si_trace_print::{pfn, pfx};
/// use si_trace_print::pf1o;
/// fn func1() {
///     pf1o!("hello");
/// }
/// fn main() {
///     pfn!();
///     func1();
///     pfx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///      main::func1: hello
/// ←main:
/// ```
///
/// Uses [`so()`].
///
/// [`println!`]: println!
/// [function]: crate::function_name::function_name_plus
/// [`so()`]: crate::stack::so
#[macro_export]
macro_rules! pf1o {
    (
        $($args:tt)*
    ) => {
        print!("{}{}: ", $crate::stack::so(), $crate::function_name::function_name_plus!(1));
        println!($($args)*)
    }
}
pub use pf1o;

/// **p**rintln! in a **f**unction plus **one** namespace level when
/// e**n**tering.
///
/// To signify entering a function.
/// Use this as the first [`println!`] in a [function].
///
/// ```rust
/// use si_trace_print::{pfn, pfx};
/// use si_trace_print::pf1n;
/// fn func1() {
///     pf1n!("hello");
/// }
/// fn main() {
///     pfn!();
///     func1();
///     pfx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///     →main::func1: hello
/// ←main:
/// ```
///
/// Uses [`sn()`].
///
/// [`println!`]: println!
/// [function]: crate::function_name::function_name_plus
/// [`sn()`]: crate::stack::sn
#[macro_export]
macro_rules! pf1n {
    (
        $($args:tt)*
    ) => {
        print!("{}{}: ", $crate::stack::sn(), $crate::function_name::function_name_plus!(1));
        println!($($args)*)
    }
}
pub use pf1n;

/// **p**rintln! in a **f**unction plus **one** namespace level when
/// e**x**iting.
///
/// To signify exiting a function.
/// Use this as the last [`println!`] in a [function].
///
/// ```rust
/// use si_trace_print::{pfn, pfx};
/// use si_trace_print::pf1x;
/// fn func1() {
///     pf1x!("hello");
/// }
/// fn main() {
///     pfn!();
///     func1();
///     pfx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///     ←si_trace_print::main::func1: hello
/// ←main:
/// ```
///
/// Uses [`sx()`].
///
/// [`println!`]: println!
/// [function]: crate::function_name::function_name
/// [`sx()`]: crate::stack::sx
#[macro_export]
macro_rules! pf1x {
    (
        $($args:tt)*
    ) => {
        print!("{}{}: ", $crate::stack::sx(), $crate::function_name::function_name_plus!(1));
        println!($($args)*)
    }
}
pub use pf1x;

/// **p**rintln! in a **f**unction plus **one** namespace level when
/// e**n**tering and e**x**iting.
///
/// To signify entering and exiting a function.
/// Use this as the only [`println!`] in a [function].
/// Most suited for short functions.
///
/// ```rust
/// use si_trace_print::{pfn, pfx};
/// use si_trace_print::pf1ñ;
/// fn func1() {
///     pf1ñ!("hello");
/// }
/// fn main() {
///     pfn!();
///     func1();
///     pfx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///     ↔main::func1: hello
/// ←main:
/// ```
///
/// Uses [`sñ()`].
///
/// [`println!`]: println!
/// [function]: crate::function_name::function_name_plus
/// [`sñ()`]: crate::stack::sñ
#[macro_export]
macro_rules! pf1ñ {
    (
        $($args:tt)*
    ) => {
        print!("{}{}: ", $crate::stack::sñ(), $crate::function_name::function_name_plus!(1));
        println!($($args)*)
    }
}
pub use pf1ñ;

//
// print with **two** namespace levels
//

/// **p**rintln! in a **f**unction plus **two** namespace levels with
/// **o**ffset.
///
/// To signify printing within a function.
/// Use to [`println!`] in a [function].
///
/// ```rust
/// use si_trace_print::{pfn, pfx};
/// use si_trace_print::pf2o;
/// fn func1() {
///     pf2o!("hello");
/// }
/// fn main() {
///     pfn!();
///     func1();
///     pfx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///      si_trace_print::main::func1: hello
/// ←main:
/// ```
///
/// Uses [`so()`].
///
/// [`println!`]: println!
/// [function]: crate::function_name::function_name_plus
/// [`so()`]: crate::stack::so
#[macro_export]
macro_rules! pf2o {
    (
        $($args:tt)*
    ) => {
        print!("{}{}: ", $crate::stack::so(), $crate::function_name::function_name_plus!(2));
        println!($($args)*)
    }
}
pub use pf2o;

/// **p**rintln! in a **f**unction plus **two** namespace levels when
/// e**n**tering.
///
/// To signify entering a function.
/// Use this as the first [`println!`] in a [function].
///
/// ```rust
/// use si_trace_print::{pfn, pfx};
/// use si_trace_print::pf2n;
/// fn func1() {
///     pf2n!("hello");
/// }
/// fn main() {
///     pfn!();
///     func1();
///     pfx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///     →si_trace_print::main::func1: hello
/// ←main:
/// ```
///
/// Uses [`sn()`].
///
/// [`println!`]: println!
/// [function]: crate::function_name::function_name_plus
/// [`sn()`]: crate::stack::sn
#[macro_export]
macro_rules! pf2n {
    (
        $($args:tt)*
    ) => {
        print!("{}{}: ", $crate::stack::sn(), $crate::function_name::function_name_plus!(2));
        println!($($args)*)
    }
}
pub use pf2n;

/// **p**rintln! in a **f**unction plus **two** namespace levels when
/// e**x**iting.
///
/// To signify exiting a function.
/// Use this as the last [`println!`] in a [function].
///
/// ```rust
/// use si_trace_print::{pfn, pfx};
/// use si_trace_print::pf2x;
/// fn func1() {
///     pf2x!("hello");
/// }
/// fn main() {
///     pfn!();
///     func1();
///     pfx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///     ←si_trace_print::main::func1: hello
/// ←main:
/// ```
///
/// Uses [`sx()`].
///
/// [`println!`]: println!
/// [function]: crate::function_name::function_name
/// [`sx()`]: crate::stack::sx
#[macro_export]
macro_rules! pf2x {
    (
        $($args:tt)*
    ) => {
        print!("{}{}: ", $crate::stack::sx(), $crate::function_name::function_name_plus!(2));
        println!($($args)*)
    }
}
pub use pf2x;

/// **p**rintln! in a **f**unction plus **two** namespace levels when
/// e**n**tering and e**x**iting.
///
/// To signify entering and exiting a function.
/// Use this as the only [`println!`] in a [function].
/// Most suited for short functions.
///
/// ```rust
/// use si_trace_print::{pfn, pfx};
/// use si_trace_print::pf2ñ;
/// fn func1() {
///     pf2ñ!("hello");
/// }
/// fn main() {
///     pfn!();
///     func1();
///     pfx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///     ↔si_trace_print::main::func1: hello
/// ←main:
/// ```
///
/// Uses [`sñ()`].
///
/// [`println!`]: println!
/// [function]: crate::function_name::function_name_plus
/// [`sñ()`]: crate::stack::sñ
#[macro_export]
macro_rules! pf2ñ {
    (
        $($args:tt)*
    ) => {
        print!("{}{}: ", $crate::stack::sñ(), $crate::function_name::function_name_plus!(2));
        println!($($args)*)
    }
}
pub use pf2ñ;

//
// `e`println
//

/// **e**println!
///
/// For completeness, wrap [`eprintln!`].
///
/// ```rust
/// use si_trace_print::{efn, efx};
/// use si_trace_print::e;
/// fn func1() {
///     e!("hello");
/// }
/// fn main() {
///     efn!();
///     func1();
///     efx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
/// hello
/// ←main:
/// ```
///
/// [`eprintln!`]: eprintln!
#[macro_export]
macro_rules! e {
    (
        $($args:tt)*
    ) => {
        // for consistency with other macros, invoke setting the
        // "original" stack depth via `so`
        $crate::stack::so();
        eprintln!($($args)*)
    }
}
pub use e;

/// **e**println! with **o**ffset.
///
/// To signify printing within a function.
/// Use this to [`eprintln!`] within a function.
///
/// ```rust
/// use si_trace_print::{efn, efx};
/// use si_trace_print::eo;
/// fn func1() {
///     eo!("hello");
/// }
/// fn main() {
///     efn!();
///     func1();
///     efx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///      hello
/// ←main:
/// ```
///
/// Uses [`so()`].
///
/// [`eprintln!`]: eprintln!
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

//
// `e`println with offset in a function
//

/// **e**println! when e**n**tering.
///
/// To signify entering a function.
/// Use this as the first [`eprintln!`] in a function.
///
/// ```rust
/// use si_trace_print::{efn, efx};
/// use si_trace_print::en;
/// fn func1() {
///     en!("hello");
/// }
/// fn main() {
///     efn!();
///     func1();
///     efx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///     →hello
/// ←main:
/// ```
///
/// Uses [`sn()`].
///
/// [`eprintln!`]: eprintln!
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

/// **e**println! when e**x**iting.
///
/// To signify exiting a function.
/// Use this as the last [`eprintln!`] in a function.
///
/// ```rust
/// use si_trace_print::{efn, efx};
/// use si_trace_print::ex;
/// fn func1() {
///     ex!("hello");
/// }
/// fn main() {
///     efn!();
///     func1();
///     efx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///     ←hello
/// ←main:
/// ```
///
/// Uses [`sx()`].
///
/// [`eprintln!`]: eprintln!
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

/// **e**println! when e**n**tering and e**x**iting.
///
/// To signify entering and exiting a function.
/// Use this as the only [`eprintln!`] in a function.
/// Most suited for short functions.
///
/// ```rust
/// use si_trace_print::{efn, efx};
/// use si_trace_print::eñ;
/// fn func1() {
///     eñ!("hello");
/// }
/// fn main() {
///     efn!();
///     func1();
///     efx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///     ↔hello
/// ←main:
/// ```
///
/// Uses [`sñ()`].
///
/// [`eprintln!`]: eprintln!
/// [`sñ()`]: crate::stack::sñ
#[macro_export]
macro_rules! eñ {
    (
        $($args:tt)*
    ) => {
        eprint!("{}", $crate::stack::sñ());
        eprintln!($($args)*)
    }
}
pub use eñ;

//
// `e`println with offset with a `f`unction name
//

/// **e**println! in a **f**unction with **o**ffset.
///
/// To signify printing within a function.
/// Use to [`eprintln!`] in a [function].
///
/// ```rust
/// use si_trace_print::{efn, efx};
/// use si_trace_print::efo;
/// fn func1() {
///     efo!("hello");
/// }
/// fn main() {
///     efn!();
///     func1();
///     efx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///      func1: hello
/// ←main:
/// ```
///
/// Uses [`so()`].
///
/// [`eprintln!`]: eprintln!
/// [function]: crate::function_name::function_name
/// [`so()`]: crate::stack::so
#[macro_export]
macro_rules! efo {
    (
        $($args:tt)*
    ) => {
        eprint!("{}{}: ", $crate::stack::so(), $crate::function_name::function_name!());
        eprintln!($($args)*)
    }
}
pub use efo;

/// **e**println! in a **f**unction when e**n**tering.
///
/// To signify entering a function.
/// Use this as the first [`eprintln!`] in a [function].
///
/// ```rust
/// use si_trace_print::{efn, efx};
/// fn func1() {
///     efn!("hello");
/// }
/// fn main() {
///     efn!();
///     func1();
///     efx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///     →func1: hello
/// ←main:
/// ```
///
/// Uses [`sn()`].
///
/// [`eprintln!`]: eprintln!
/// [function]: crate::function_name::function_name
/// [`sn()`]: crate::stack::sn
#[macro_export]
macro_rules! efn {
    (
        $($args:tt)*
    ) => {
        eprint!("{}{}: ", $crate::stack::sn(), $crate::function_name::function_name!());
        eprintln!($($args)*)
    }
}
pub use efn;

/// **e**println! in a **f**unction when e**x**iting.
///
/// To signify exiting a function.
/// Use this as the last [`eprintln!`] in a [function].
///
/// ```rust
/// use si_trace_print::{efn, efx};
/// fn func1() {
///     efx!("hello");
/// }
/// fn main() {
///     efn!();
///     func1();
///     efx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///     ←func1: hello
/// ←main:
/// ```
///
/// Uses [`sx()`].
///
/// [`eprintln!`]: eprintln!
/// [function]: crate::function_name::function_name
/// [`sx()`]: crate::stack::sx
#[macro_export]
macro_rules! efx {
    (
        $($args:tt)*
    ) => {
        eprint!("{}{}: ", $crate::stack::sx(), $crate::function_name::function_name!());
        eprintln!($($args)*)
    }
}
pub use efx;

/// **e**println! in a **f**unction when e**n**tering and e**x**iting.
///
/// To signify entering and exiting a function.
/// Use this as the only [`eprintln!`] in a [function].
/// Most suited for short functions.
///
/// ```rust
/// use si_trace_print::{efn, efx};
/// use si_trace_print::efñ;
/// fn func1() {
///     efñ!("hello");
/// }
/// fn main() {
///     efn!();
///     func1();
///     efx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///     ↔func1: hello
/// ←main:
/// ```
///
/// Uses [`sñ()`].
///
/// [`eprintln!`]: eprintln!
/// [function]: crate::function_name::function_name
/// [`sñ()`]: crate::stack::sñ
#[macro_export]
macro_rules! efñ {
    (
        $($args:tt)*
    ) => {
        eprint!("{}{}: ", $crate::stack::sñ(), $crate::function_name::function_name!());
        eprintln!($($args)*)
    }
}
pub use efñ;

//
// `e`println with offset with a `f`unction name with `one` namespace levels
//

/// **e**println! in a **f**unction with **one** namespace levels and
/// **o**ffset.
///
/// To signify printing within a function.
/// Use to [`eprintln!`] in a [function].
///
/// ```rust
/// use si_trace_print::{efn, efx};
/// use si_trace_print::ef1o;
/// fn func1() {
///     ef1o!("hello");
/// }
/// fn main() {
///     efn!();
///     func1();
///     efx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///      main::func1: hello
/// ←main:
/// ```
///
/// Uses [`so()`].
///
/// [`eprintln!`]: eprintln!
/// [function]: crate::function_name::function_name_plus
/// [`so()`]: crate::stack::so
#[macro_export]
macro_rules! ef1o {
    (
        $($args:tt)*
    ) => {
        eprint!("{}{}: ", $crate::stack::so(), $crate::function_name::function_name_plus!(1));
        eprintln!($($args)*)
    }
}
pub use ef1o;

/// **e**println! in a **f**unction plus **one** namespace level
/// when e**n**tering.
///
/// To signify entering a function.
/// Use this as the first [`eprintln!`] in a [function].
///
/// ```rust
/// use si_trace_print::{efn, efx};
/// use si_trace_print::ef1n;
/// fn func1() {
///     ef1n!("hello");
/// }
/// fn main() {
///     efn!();
///     func1();
///     efx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///     →main::func1: hello
/// ←main:
/// ```
///
/// Uses [`sn()`].
///
/// [`eprintln!`]: eprintln!
/// [function]: crate::function_name::function_name_plus
/// [`sn()`]: crate::stack::sn
#[macro_export]
macro_rules! ef1n {
    (
        $($args:tt)*
    ) => {
        eprint!("{}{}: ", $crate::stack::sn(), $crate::function_name::function_name_plus!(1));
        eprintln!($($args)*)
    }
}
pub use ef1n;

/// **e**println! in a **f**unction plus **one** namespace level
/// when e**x**iting.
///
/// To signify exiting a function.
/// Use this as the last [`eprintln!`] in a [function].
///
/// ```rust
/// use si_trace_print::{efn, efx};
/// use si_trace_print::ef1x;
/// fn func1() {
///     ef1x!("hello");
/// }
/// fn main() {
///     efn!();
///     func1();
///     efx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///     ←si_trace_print::main::func1: hello
/// ←main:
/// ```
///
/// Uses [`sx()`].
///
/// [`eprintln!`]: eprintln!
/// [function]: crate::function_name::function_name_plus
/// [`sx()`]: crate::stack::sx
#[macro_export]
macro_rules! ef1x {
    (
        $($args:tt)*
    ) => {
        eprint!("{}{}: ", $crate::stack::sx(), $crate::function_name::function_name_plus!(1));
        eprintln!($($args)*)
    }
}
pub use ef1x;

/// **e**println! in a **f**unction plus **one** namespace level
/// when e**n**tering and e**x**iting.
///
/// To signify entering and exiting a function.
/// Use this as the only [`eprintln!`] in a [function].
/// Most suited for short functions.
///
/// ```rust
/// use si_trace_print::{efn, efx};
/// use si_trace_print::ef1ñ;
/// fn func1() {
///     ef1ñ!("hello");
/// }
/// fn main() {
///     efn!();
///     func1();
///     efx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///     ↔main::func1: hello
/// ←main:
/// ```
///
/// Uses [`sñ()`].
///
/// [`eprintln!`]: eprintln!
/// [function]: crate::function_name::function_name_plus
/// [`sñ()`]: crate::stack::sñ
#[macro_export]
macro_rules! ef1ñ {
    (
        $($args:tt)*
    ) => {
        eprint!("{}{}: ", $crate::stack::sñ(), $crate::function_name::function_name_plus!(1));
        eprintln!($($args)*)
    }
}
pub use ef1ñ;

//
// `e`println with offset with a `f`unction name with `two` namespace levels
//

/// **e**println! in a **f**unction with **two** namespace levels and
/// **o**ffset.
///
/// To signify printing within a function.
/// Use to [`eprintln!`] in a [function].
///
/// ```rust
/// use si_trace_print::{efn, efx};
/// use si_trace_print::ef2o;
/// fn func1() {
///     ef2o!("hello");
/// }
/// fn main() {
///     efn!();
///     func1();
///     efx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///      si_trace_print::main::func1: hello
/// ←main:
/// ```
///
/// Uses [`so()`].
///
/// [`eprintln!`]: eprintln!
/// [function]: crate::function_name::function_name_plus
/// [`so()`]: crate::stack::so
#[macro_export]
macro_rules! ef2o {
    (
        $($args:tt)*
    ) => {
        eprint!("{}{}: ", $crate::stack::so(), $crate::function_name::function_name_plus!(2));
        eprintln!($($args)*)
    }
}
pub use ef2o;

/// **e**println! in a **f**unction plus **two** namespace levels
/// when e**n**tering.
///
/// To signify entering a function.
/// Use this as the first [`eprintln!`] in a [function].
///
/// ```rust
/// use si_trace_print::{efn, efx};
/// use si_trace_print::ef2n;
/// fn func1() {
///     ef2n!("hello");
/// }
/// fn main() {
///     efn!();
///     func1();
///     efx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///     →si_trace_print::main::func1: hello
/// ←main:
/// ```
///
/// Uses [`sn()`].
///
/// [`eprintln!`]: eprintln!
/// [function]: crate::function_name::function_name_plus
/// [`sn()`]: crate::stack::sn
#[macro_export]
macro_rules! ef2n {
    (
        $($args:tt)*
    ) => {
        eprint!("{}{}: ", $crate::stack::sn(), $crate::function_name::function_name_plus!(2));
        eprintln!($($args)*)
    }
}
pub use ef2n;

/// **e**println! in a **f**unction plus **two** namespace levels
/// when e**x**iting.
///
/// To signify exiting a function.
/// Use this as the last [`eprintln!`] in a [function].
///
/// ```rust
/// use si_trace_print::{efn, efx};
/// use si_trace_print::ef2x;
/// fn func1() {
///     ef2x!("hello");
/// }
/// fn main() {
///     efn!();
///     func1();
///     efx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///     ←si_trace_print::main::func1: hello
/// ←main:
/// ```
///
/// Uses [`sx()`].
///
/// [`eprintln!`]: eprintln!
/// [function]: crate::function_name::function_name_plus
/// [`sx()`]: crate::stack::sx
#[macro_export]
macro_rules! ef2x {
    (
        $($args:tt)*
    ) => {
        eprint!("{}{}: ", $crate::stack::sx(), $crate::function_name::function_name_plus!(2));
        eprintln!($($args)*)
    }
}
pub use ef2x;

/// **e**println! in a **f**unction plus **two** namespace levels
/// when e**n**tering and e**x**iting.
///
/// To signify entering and exiting a function.
/// Use this as the only [`eprintln!`] in a [function].
/// Most suited for short functions.
///
/// ```rust
/// use si_trace_print::{efn, efx};
/// use si_trace_print::ef2ñ;
/// fn func1() {
///     ef2ñ!("hello");
/// }
/// fn main() {
///     efn!();
///     func1();
///     efx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///     ↔si_trace_print::main::func1: hello
/// ←main:
/// ```
///
/// Uses [`sñ()`].
///
/// [`eprintln!`]: eprintln!
/// [function]: crate::function_name::function_name_plus
/// [`sñ()`]: crate::stack::sñ
#[macro_export]
macro_rules! ef2ñ {
    (
        $($args:tt)*
    ) => {
        eprint!("{}{}: ", $crate::stack::sñ(), $crate::function_name::function_name_plus!(2));
        eprintln!($($args)*)
    }
}
pub use ef2ñ;

//
// **d**ebug `p`rintln
//

/// **d**ebug **p**rintln!
///
/// ```rust
/// use si_trace_print::{dpfn, dpfx};
/// use si_trace_print::dp;
/// fn func1() {
///     dp!("hello");
/// }
/// fn main() {
///     dpfn!();
///     func1();
///     dpfx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
/// hello
/// ←main:
/// ```
///
/// For completeness, wraps [`println!`] for debug builds.
///
/// For debug builds.
///
/// [`println!`]: println!
#[macro_export]
macro_rules! dp {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        println!($($args)*)
    }
}
pub use dp;

/// **d**ebug **p**rintln! with **o**ffset.
///
/// To signify printing within a function.
/// Use this to [`println!`] within a function.
///
/// ```rust
/// use si_trace_print::{dpfn, dpfx};
/// use si_trace_print::dpo;
/// fn func1() {
///     dpo!("hello");
/// }
/// fn main() {
///     dpfn!();
///     func1();
///     dpfx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///      hello
/// ←main:
/// ```
///
/// Uses [`so()`].
///
/// For debug builds.
///
/// [`println!`]: println!
/// [`so()`]: crate::stack::so
#[macro_export]
macro_rules! dpo {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        print!("{}", $crate::stack::so());
        #[cfg(any(debug_assertions,test))]
        println!($($args)*)
    }
}
pub use dpo;

/// **d**ebug **p**rintln! when e**n**tering.
///
/// To signify entering a function.
/// Use this as the first [`println!`] in a function.
///
/// ```rust
/// use si_trace_print::{dpfn, dpfx};
/// use si_trace_print::dpn;
/// fn func1() {
///     dpn!("hello");
/// }
/// fn main() {
///     dpfn!();
///     func1();
///     dpfx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///     →hello
/// ←main:
/// ```
///
/// Uses [`sn()`].
///
/// For debug builds.
///
/// [`println!`]: println!
/// [`sn()`]: crate::stack::sn
#[macro_export]
macro_rules! dpn {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        print!("{}", $crate::stack::sn());
        #[cfg(any(debug_assertions,test))]
        println!($($args)*)
    }
}
pub use dpn;

/// **d**ebug **p**rintln! when e**x**iting.
///
/// To signify exiting a function.
/// Use this as the last [`println!`] in a function.
///
/// ```rust
/// use si_trace_print::{dpfn, dpfx};
/// use si_trace_print::dpx;
/// fn func1() {
///     dpx!("hello");
/// }
/// fn main() {
///     dpfn!();
///     func1();
///     dpfx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///     ←hello
/// ←main:
/// ```
///
/// Uses [`sx()`].
///
/// For debug builds.
///
/// [`println!`]: println!
/// [`sx()`]: crate::stack::sx
#[macro_export]
macro_rules! dpx {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        print!("{}", $crate::stack::sx());
        #[cfg(any(debug_assertions,test))]
        println!($($args)*)
    }
}
pub use dpx;

/// **d**ebug **p**rintln! when e**n**tering and e**x**iting.
///
/// To signify entering and exiting a function.
/// Use this as the only [`println!`] in a function.
/// Most suited for short functions.
///
/// ```rust
/// use si_trace_print::{dpfn, dpfx};
/// use si_trace_print::dpñ;
/// fn func1() {
///     dpñ!("hello");
/// }
/// fn main() {
///     dpfn!();
///     func1();
///     dpfx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///     ↔hello
/// ←main:
/// ```
///
/// Uses [`sñ()`].
///
/// For debug builds.
///
/// [`println!`]: println!
/// [`sñ()`]: crate::stack::sñ
#[macro_export]
macro_rules! dpñ {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        print!("{}", $crate::stack::sñ());
        #[cfg(any(debug_assertions,test))]
        println!($($args)*)
    }
}
pub use dpñ;

/// **d**ebug **p**rintln! in a **f**unction with **o**ffset.
///
/// To signify printing within a function.
/// Use to [`println!`] in a [function].
///
/// ```rust
/// use si_trace_print::{dpfn, dpfx};
/// use si_trace_print::dpfo;
/// fn func1() {
///     dpfo!("hello");
/// }
/// fn main() {
///     dpfn!();
///     func1();
///     dpfx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///      func1: hello
/// ←main:
/// ```
///
/// Uses [`so()`].
///
/// For debug builds.
///
/// [`println!`]: println!
/// [function]: crate::function_name::function_name
/// [`so()`]: crate::stack::so
#[macro_export]
macro_rules! dpfo {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        print!("{}{}: ", $crate::stack::so(), $crate::function_name::function_name!());
        #[cfg(any(debug_assertions,test))]
        println!($($args)*)
    }
}
pub use dpfo;

//
// `d`ebug `p`rintln! with `f`unction name
//

/// **d**ebug **p**rintln! in a **f**unction when e**n**tering.
///
/// To signify entering a function.
/// Use this as the first [`println!`] in a [function].
///
/// ```rust
/// use si_trace_print::{dpfn, dpfx};
/// fn func1() {
///     dpfn!("hello");
/// }
/// fn main() {
///     dpfn!();
///     func1();
///     dpfx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///     →func1: hello
/// ←main:
/// ```
///
/// Uses [`sn()`].
///
/// For debug builds.
///
/// [`println!`]: println!
/// [function]: crate::function_name::function_name
/// [`sn()`]: crate::stack::sn
#[macro_export]
macro_rules! dpfn {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        print!("{}{}: ", $crate::stack::sn(), $crate::function_name::function_name!());
        #[cfg(any(debug_assertions,test))]
        println!($($args)*)
    }
}
pub use dpfn;

/// **d**ebug **p**rintln! in a **f**unction when e**x**iting.
///
/// To signify exiting a function.
/// Use this as the last [`println!`] in a [function].
///
/// ```rust
/// use si_trace_print::{dpfn, dpfx};
/// fn func1() {
///     dpfx!("hello");
/// }
/// fn main() {
///     dpfn!();
///     func1();
///     dpfx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///     ←func1: hello
/// ←main:
/// ```
///
/// Uses [`sx()`].
///
/// For debug builds.
///
/// [`println!`]: println!
/// [function]: crate::function_name::function_name
/// [`sx()`]: crate::stack::sx
#[macro_export]
macro_rules! dpfx {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        print!("{}{}: ", $crate::stack::sx(), $crate::function_name::function_name!());
        #[cfg(any(debug_assertions,test))]
        println!($($args)*)
    }
}
pub use dpfx;

/// **d**ebug **p**rintln! in a **f**unction when e**n**tering and e**x**iting.
///
/// To signify entering and exiting a function.
/// Use this as the only [`println!`] in a [function].
/// Most suited for short functions.
///
/// ```rust
/// use si_trace_print::{dpfn, dpfx};
/// use si_trace_print::dpfñ;
/// fn func1() {
///     dpfñ!("hello");
/// }
/// fn main() {
///     dpfn!();
///     func1();
///     dpfx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///     ↔func1: hello
/// ←main:
/// ```
///
/// Uses [`sñ()`].
///
/// For debug builds.
///
/// [`println!`]: println!
/// [function]: crate::function_name::function_name
/// [`sñ()`]: crate::stack::sñ
#[macro_export]
macro_rules! dpfñ {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        print!("{}{}: ", $crate::stack::sñ(), $crate::function_name::function_name!());
        #[cfg(any(debug_assertions,test))]
        println!($($args)*)
    }
}
pub use dpfñ;

//
// `d`ebug `p`rintln! with `f`unction name and `one` namespace levels
//

/// **d**ebug **p**rintln! in a **f**unction plus **one** namespace level with
/// **o**ffset.
///
/// To signify printing within a function.
/// Use to [`println!`] in a [function].
///
/// ```rust
/// use si_trace_print::{dpfn, dpfx};
/// use si_trace_print::dpf1o;
/// fn func1() {
///     dpf1o!("hello");
/// }
/// fn main() {
///     dpfn!();
///     func1();
///     dpfx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///      main::func1: hello
/// ←main:
/// ```
///
/// Uses [`so()`].
///
/// For debug builds.
///
/// [`println!`]: println!
/// [function]: crate::function_name::function_name_plus
/// [`so()`]: crate::stack::so
#[macro_export]
macro_rules! dpf1o {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        print!("{}{}: ", $crate::stack::so(), $crate::function_name::function_name_plus!(1));
        #[cfg(any(debug_assertions,test))]
        println!($($args)*)
    }
}
pub use dpf1o;

/// **d**ebug **p**rintln! in a **f**unction plus **one** namespace level when
/// e**n**tering.
///
/// To signify entering a function.
/// Use this as the first [`println!`] in a [function].
///
/// ```rust
/// use si_trace_print::{dpfn, dpfx};
/// use si_trace_print::dpf1n;
/// fn func1() {
///     dpf1n!("hello");
/// }
/// fn main() {
///     dpfn!();
///     func1();
///     dpfx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///     →si_trace_print::main::func1: hello
/// ←main:
/// ```
///
/// Uses [`sn()`].
///
/// For debug builds.
///
/// [`println!`]: println!
/// [function]: crate::function_name::function_name_plus
/// [`sn()`]: crate::stack::sn
#[macro_export]
macro_rules! dpf1n {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        print!("{}{}: ", $crate::stack::sn(), $crate::function_name::function_name_plus!(1));
        #[cfg(any(debug_assertions,test))]
        println!($($args)*)
    }
}
pub use dpf1n;

/// **d**ebug **p**rintln! in a **f**unction plus **one** namespace level when
/// e**x**iting.
///
/// To signify exiting a function.
/// Use this as the last [`println!`] in a [function].
///
/// ```rust
/// use si_trace_print::{dpfn, dpfx};
/// use si_trace_print::dpf1x;
/// fn func1() {
///     dpf1x!("hello");
/// }
/// fn main() {
///     dpfn!();
///     func1();
///     dpfx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///     ←main::func1: hello
/// ←main:
/// ```
///
/// Uses [`sx()`].
///
/// For debug builds.
///
/// [`println!`]: println!
/// [function]: crate::function_name::function_name_plus
/// [`sx()`]: crate::stack::sx
#[macro_export]
macro_rules! dpf1x {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        print!("{}{}: ", $crate::stack::sx(), $crate::function_name::function_name_plus!(1));
        #[cfg(any(debug_assertions,test))]
        println!($($args)*)
    }
}
pub use dpf1x;

/// **d**ebug **p**rintln! in a **f**unction plus **one** namespace level when
/// e**n**tering and e**x**iting.
///
/// To signify entering and exiting a function.
/// Use this as the only [`println!`] in a [function].
/// Most suited for short functions.
///
/// ```rust
/// use si_trace_print::{dpfn, dpfx};
/// use si_trace_print::dpf1ñ;
/// fn func1() {
///     dpf1ñ!("hello");
/// }
/// fn main() {
///     dpfn!();
///     func1();
///     dpfx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///     ↔main::func1: hello
/// ←main:
/// ```
///
/// Uses [`sñ()`].
///
/// For debug builds.
///
/// [`println!`]: println!
/// [function]: crate::function_name::function_name_plus
/// [`sñ()`]: crate::stack::sñ
#[macro_export]
macro_rules! dpf1ñ {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        print!("{}{}: ", $crate::stack::sñ(), $crate::function_name::function_name_plus!(1));
        #[cfg(any(debug_assertions,test))]
        println!($($args)*)
    }
}
pub use dpf1ñ;

//
// `d`ebug `p`rintln! with `f`unction name and `two` namespace levels
//

/// **d**ebug **p**rintln! in a **f**unction plus **two** namespace levels with
/// **o**ffset
///
/// To signify printing within a function.
/// Use to [`println!`] in a [function].
///
/// ```rust
/// use si_trace_print::{dpfn, dpfx};
/// use si_trace_print::dpf2o;
/// fn func1() {
///     dpf2o!("hello");
/// }
/// fn main() {
///     dpfn!();
///     func1();
///     dpfx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///      si_trace_print::main::func1: hello
/// ←main:
/// ```
///
/// Uses [`so()`].
///
/// For debug builds.
///
/// [`println!`]: println!
/// [function]: crate::function_name::function_name_plus
/// [`so()`]: crate::stack::so
#[macro_export]
macro_rules! dpf2o {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        print!("{}{}: ", $crate::stack::so(), $crate::function_name::function_name_plus!(2));
        #[cfg(any(debug_assertions,test))]
        println!($($args)*)
    }
}
pub use dpf2o;

/// **d**ebug **p**rintln! in a **f**unction plus **two** namespace levels when
/// e**n**tering.
///
/// To signify entering a function.
/// Use this as the first [`println!`] in a [function].
///
/// ```rust
/// use si_trace_print::{dpfn, dpfx};
/// use si_trace_print::dpf2n;
/// fn func1() {
///     dpf2n!("hello");
/// }
/// fn main() {
///     dpfn!();
///     func1();
///     dpfx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///     →si_trace_print::main::func1: hello
/// ←main:
/// ```
///
/// Uses [`sn()`].
///
/// For debug builds.
///
/// [`println!`]: println!
/// [function]: crate::function_name::function_name_plus
/// [`sn()`]: crate::stack::sn
#[macro_export]
macro_rules! dpf2n {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        print!("{}{}: ", $crate::stack::sn(), $crate::function_name::function_name_plus!(2));
        #[cfg(any(debug_assertions,test))]
        println!($($args)*)
    }
}
pub use dpf2n;

/// **d**ebug **p**rintln! in a **f**unction plus **two** namespace levels when
/// e**x**iting.
///
/// To signify exiting a function.
/// Use this as the last [`println!`] in a [function].
///
/// ```rust
/// use si_trace_print::{dpfn, dpfx};
/// use si_trace_print::dpf2x;
/// fn func1() {
///     dpf2x!("hello");
/// }
/// fn main() {
///     dpfn!();
///     func1();
///     dpfx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///     ←si_trace_print::main::func1: hello
/// ←main:
/// ```
///
/// Uses [`sx()`].
///
/// For debug builds.
///
/// [`println!`]: println!
/// [function]: crate::function_name::function_name_plus
/// [`sx()`]: crate::stack::sx
#[macro_export]
macro_rules! dpf2x {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        print!("{}{}: ", $crate::stack::sx(), $crate::function_name::function_name_plus!(2));
        #[cfg(any(debug_assertions,test))]
        println!($($args)*)
    }
}
pub use dpf2x;

/// **d**ebug **p**rintln! in a **f**unction plus **two** namespace levels when
/// e**n**tering and e**x**iting.
///
/// To signify entering and exiting a function.
/// Use this as the only [`println!`] in a [function].
/// Most suited for short functions.
///
/// ```rust
/// use si_trace_print::{dpfn, dpfx};
/// use si_trace_print::dpf2ñ;
/// fn func1() {
///     dpf2ñ!("hello");
/// }
/// fn main() {
///     dpfn!();
///     func1();
///     dpfx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///     ↔si_trace_print::main::func1: hello
/// ←main:
/// ```
///
/// Uses [`sñ()`].
///
/// For debug builds.
///
/// [`println!`]: println!
/// [function]: crate::function_name::function_name_plus
/// [`sñ()`]: crate::stack::sñ
#[macro_export]
macro_rules! dpf2ñ {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        print!("{}{}: ", $crate::stack::sñ(), $crate::function_name::function_name_plus!(2));
        #[cfg(any(debug_assertions,test))]
        println!($($args)*)
    }
}
pub use dpf2ñ;

//
// **d**ebug `e`println
//

/// **d**ebug **e**println!
///
/// For completeness, wrap [`eprintln!`].
///
/// ```rust
/// use si_trace_print::{defn, defx};
/// use si_trace_print::de;
/// fn func1() {
///     de!("hello");
/// }
/// fn main() {
///     defn!();
///     func1();
///     defx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
/// hello
/// ←main:
/// ```
///
/// For debug builds.
///
/// [`eprintln!`]: eprintln!
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

/// **d**ebug **e**println! with **o**ffset.
///
/// To signify printing within a function.
/// Use this to [`eprintln!`] within a function.
///
/// ```rust
/// use si_trace_print::{defn, defx};
/// use si_trace_print::deo;
/// fn func1() {
///     deo!("hello");
/// }
/// fn main() {
///     defn!();
///     func1();
///     defx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///      hello
/// ←main:
/// ```
///
/// Uses [`so()`].
///
/// For debug builds.
///
/// [`eprintln!`]: eprintln!
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

/// **d**ebug **e**println! when e**n**tering.
///
/// To signify entering a function.
/// Use this as the first [`eprintln!`] in a function.
///
/// ```rust
/// use si_trace_print::{defn, defx};
/// use si_trace_print::den;
/// fn func1() {
///     den!("hello");
/// }
/// fn main() {
///     defn!();
///     func1();
///     defx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///     →hello
/// ←main:
/// ```
///
/// Uses [`sn()`].
///
/// For debug builds.
///
/// [`eprintln!`]: eprintln!
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

/// **d**ebug **e**println! when e**x**iting.
///
/// To signify exiting a function.
/// Use this as the last [`eprintln!`] in a function.
///
/// ```rust
/// use si_trace_print::{defn, defx};
/// use si_trace_print::dex;
/// fn func1() {
///     dex!("hello");
/// }
/// fn main() {
///     defn!();
///     func1();
///     defx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///     ←hello
/// ←main:
/// ```
///
/// Uses [`sx()`].
///
/// For debug builds.
///
/// [`eprintln!`]: eprintln!
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

/// **d**ebug **e**println! when e**n**tering and e**x**iting.
///
/// To signify entering and exiting a function.
/// Use this as the only [`eprintln!`] in a function.
/// Most suited for short functions.
///
/// ```rust
/// use si_trace_print::{defn, defx};
/// use si_trace_print::deñ;
/// fn func1() {
///     deñ!("hello");
/// }
/// fn main() {
///     defn!();
///     func1();
///     defx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///     ↔hello
/// ←main:
/// ```
///
/// Uses [`sñ()`].
///
/// For debug builds.
///
/// [`eprintln!`]: eprintln!
/// [`sñ()`]: crate::stack::sñ
#[macro_export]
macro_rules! deñ {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        eprint!("{}", $crate::stack::sñ());
        #[cfg(any(debug_assertions,test))]
        eprintln!($($args)*)
    }
}
pub use deñ;

//
// `d`ebug `e`println with `f`unction name
//

/// **d**ebug **e**println! in a **f**unction with **o**ffset.
///
/// To signify printing within a function.
/// Use to [`eprintln!`] in a [function].
///
/// ```rust
/// use si_trace_print::{defn, defx};
/// use si_trace_print::defo;
/// fn func1() {
///     defo!("hello");
/// }
/// fn main() {
///     defn!();
///     func1();
///     defx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///      func1: hello
/// ←main:
/// ```
///
/// Uses [`so()`].
///
/// For debug builds.
///
/// [`eprintln!`]: eprintln!
/// [function]: crate::function_name::function_name
/// [`so()`]: crate::stack::so
#[macro_export]
macro_rules! defo {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        eprint!("{}{}: ", $crate::stack::so(), $crate::function_name::function_name!());
        #[cfg(any(debug_assertions,test))]
        eprintln!($($args)*)
    }
}
pub use defo;

/// **d**ebug **e**println! in a **f**unction when e**n**tering.
///
/// To signify entering a function.
/// Use this as the first [`eprintln!`] in a [function].
///
/// ```rust
/// use si_trace_print::{defn, defx};
/// fn func1() {
///     defn!("hello");
/// }
/// fn main() {
///     defn!();
///     func1();
///     defx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///     →func1: hello
/// ←main:
/// ```
///
/// Uses [`sn()`].
///
/// For debug builds.
///
/// [`eprintln!`]: eprintln!
/// [function]: crate::function_name::function_name
/// [`sn()`]: crate::stack::sn
#[macro_export]
macro_rules! defn {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        eprint!("{}{}: ", $crate::stack::sn(), $crate::function_name::function_name!());
        #[cfg(any(debug_assertions,test))]
        eprintln!($($args)*)
    }
}
pub use defn;

/// **d**ebug **e**println! in a **f**unction when e**x**iting.
///
/// To signify exiting a function.
/// Use this as the last [`eprintln!`] in a [function].
///
/// ```rust
/// use si_trace_print::{defn, defx};
/// fn func1() {
///     defx!("hello");
/// }
/// fn main() {
///     defn!();
///     func1();
///     defx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///     ←func1: hello
/// ←main:
/// ```
///
/// Uses [`sx()`].
///
/// For debug builds.
///
/// [`eprintln!`]: eprintln!
/// [function]: crate::function_name::function_name
/// [`sx()`]: crate::stack::sx
#[macro_export]
macro_rules! defx {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        eprint!("{}{}: ", $crate::stack::sx(), $crate::function_name::function_name!());
        #[cfg(any(debug_assertions,test))]
        eprintln!($($args)*)
    }
}
pub use defx;

/// **d**ebug **e**println! in a **f**unction when e**n**tering and e**x**iting.
///
/// To signify entering and exiting a function.
/// Use this as the only [`eprintln!`] in a [function].
/// Most suited for short functions.
///
/// ```rust
/// use si_trace_print::{defn, defx};
/// use si_trace_print::defñ;
/// fn func1() {
///     defñ!("hello");
/// }
/// fn main() {
///     defn!();
///     func1();
///     defx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///     ↔func1: hello
/// ←main:
/// ```
///
/// Uses [`sñ()`].
///
/// For debug builds.
///
/// [`eprintln!`]: eprintln!
/// [function]: crate::function_name::function_name
/// [`sñ()`]: crate::stack::sñ
#[macro_export]
macro_rules! defñ {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        eprint!("{}{}: ", $crate::stack::sñ(), $crate::function_name::function_name!());
        #[cfg(any(debug_assertions,test))]
        eprintln!($($args)*)
    }
}
pub use defñ;

//
// `d`ebug `e`println with `f`unction name plus `one` namespace levels
//

/// **d**ebug **e**println! in a **f**unction plus **one** namespace level
/// with **o**ffset.
///
/// To signify printing within a function.
/// Use to [`eprintln!`] in a [function].
///
/// ```rust
/// use si_trace_print::{defn, defx};
/// use si_trace_print::def1o;
/// fn func1() {
///     def1o!("hello");
/// }
/// fn main() {
///     defn!();
///     func1();
///     defx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///      main::func1: hello
/// ←main:
/// ```
///
/// Uses [`so()`].
///
/// For debug builds.
///
/// [`eprintln!`]: eprintln!
/// [function]: crate::function_name::function_name_plus
/// [`so()`]: crate::stack::so
#[macro_export]
macro_rules! def1o {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        eprint!("{}{}: ", $crate::stack::so(), $crate::function_name::function_name_plus!(1));
        #[cfg(any(debug_assertions,test))]
        eprintln!($($args)*)
    }
}
pub use def1o;

/// **d**ebug **e**println! in a **f**unction plus **one** namespace level when
/// e**n**tering.
///
/// To signify entering a function.
/// Use this as the first [`eprintln!`] in a [function].
///
/// ```rust
/// use si_trace_print::{defn, defx};
/// use si_trace_print::def1n;
/// fn func1() {
///     def1n!("hello");
/// }
/// fn main() {
///     defn!();
///     func1();
///     defx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///     →main::func1: hello
/// ←main:
/// ```
///
/// Uses [`sn()`].
///
/// For debug builds.
///
/// [`eprintln!`]: eprintln!
/// [function]: crate::function_name::function_name_plus
/// [`sn()`]: crate::stack::sn
#[macro_export]
macro_rules! def1n {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        eprint!("{}{}: ", $crate::stack::sn(), $crate::function_name::function_name_plus!(1));
        #[cfg(any(debug_assertions,test))]
        eprintln!($($args)*)
    }
}
pub use def1n;

/// **d**ebug **e**println! in a **f**unction plus **one** namespace level when
/// e**x**iting.
///
/// To signify exiting a function.
/// Use this as the last [`eprintln!`] in a [function].
///
/// ```rust
/// use si_trace_print::{defn, defx};
/// use si_trace_print::def1x;
/// fn func1() {
///     def1x!("hello");
/// }
/// fn main() {
///     defn!();
///     func1();
///     defx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///     ←main::func1: hello
/// ←main:
/// ```
///
/// Uses [`sx()`].
///
/// For debug builds.
///
/// [`eprintln!`]: eprintln!
/// [function]: crate::function_name::function_name_plus
/// [`sx()`]: crate::stack::sx
#[macro_export]
macro_rules! def1x {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        eprint!("{}{}: ", $crate::stack::sx(), $crate::function_name::function_name_plus!(1));
        #[cfg(any(debug_assertions,test))]
        eprintln!($($args)*)
    }
}
pub use def1x;

/// **d**ebug **e**println! in a **f**unction with **one** namespace levels when
/// e**n**tering and e**x**iting.
///
/// To signify entering and exiting a function.
/// Use this as the only [`eprintln!`] in a [function].
/// Most suited for short functions.
///
/// ```rust
/// use si_trace_print::{defn, defx};
/// use si_trace_print::def1ñ;
/// fn func1() {
///     def1ñ!("hello");
/// }
/// fn main() {
///     defn!();
///     func1();
///     defx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///     ↔main::func1: hello
/// ←main:
/// ```
///
/// Uses [`sñ()`].
///
/// For debug builds.
///
/// [`eprintln!`]: eprintln!
/// [function]: crate::function_name::function_name_plus
/// [`sñ()`]: crate::stack::sñ
#[macro_export]
macro_rules! def1ñ {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        eprint!("{}{}: ", $crate::stack::sñ(), $crate::function_name::function_name_plus!(1));
        #[cfg(any(debug_assertions,test))]
        eprintln!($($args)*)
    }
}
pub use def1ñ;

//
// `d`ebug `e`println with `f`unction name plus `two` namespace levels
//

/// **d**ebug **e**println! in a **f**unction plus **two** namespace levels
/// with **o**ffset.
///
/// To signify printing within a function.
/// Use to [`eprintln!`] in a [function].
///
/// ```rust
/// use si_trace_print::{defn, defx};
/// use si_trace_print::def2o;
/// fn func1() {
///     def2o!("hello");
/// }
/// fn main() {
///     defn!();
///     func1();
///     defx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///      si_trace_print::main::func1: hello
/// ←main:
/// ```
///
/// Uses [`so()`].
///
/// For debug builds.
///
/// [`eprintln!`]: eprintln!
/// [function]: crate::function_name::function_name_plus
/// [`so()`]: crate::stack::so
#[macro_export]
macro_rules! def2o {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        eprint!("{}{}: ", $crate::stack::so(), $crate::function_name::function_name_plus!(2));
        #[cfg(any(debug_assertions,test))]
        eprintln!($($args)*)
    }
}
pub use def2o;

/// **d**ebug **e**println! in a **f**unction plus **two** namespace levels
/// when e**n**tering.
///
/// To signify entering a function.
/// Use this as the first [`eprintln!`] in a [function].
///
/// ```rust
/// use si_trace_print::{defn, defx};
/// use si_trace_print::def2n;
/// fn func1() {
///     def2n!("hello");
/// }
/// fn main() {
///     defn!();
///     func1();
///     defx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///     →si_trace_print::main::func1: hello
/// ←main:
/// ```
///
/// Uses [`sn()`].
///
/// For debug builds.
///
/// [`eprintln!`]: eprintln!
/// [function]: crate::function_name::function_name_plus
/// [`sn()`]: crate::stack::sn
#[macro_export]
macro_rules! def2n {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        eprint!("{}{}: ", $crate::stack::sn(), $crate::function_name::function_name_plus!(2));
        #[cfg(any(debug_assertions,test))]
        eprintln!($($args)*)
    }
}
pub use def2n;

/// **d**ebug **e**println! in a **f**unction plus **two** namespace levels
/// when e**x**iting.
///
/// To signify exiting a function.
/// Use this as the last [`eprintln!`] in a [function].
///
/// ```rust
/// use si_trace_print::{defn, defx};
/// use si_trace_print::def2x;
/// fn func1() {
///     def2x!("hello");
/// }
/// fn main() {
///     defn!();
///     func1();
///     defx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///     ←si_trace_print::main::func1: hello
/// ←main:
/// ```
///
/// Uses [`sx()`].
///
/// For debug builds.
///
/// [`eprintln!`]: eprintln!
/// [function]: crate::function_name::function_name_plus
/// [`sx()`]: crate::stack::sx
#[macro_export]
macro_rules! def2x {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        eprint!("{}{}: ", $crate::stack::sx(), $crate::function_name::function_name_plus!(2));
        #[cfg(any(debug_assertions,test))]
        eprintln!($($args)*)
    }
}
pub use def2x;

/// **d**ebug **e**println! in a **f**unction plus **two** namespace levels
/// when e**n**tering and e**x**iting.
///
/// To signify entering and exiting a function.
/// Use this as the only [`eprintln!`] in a [function].
/// Most suited for short functions.
///
/// ```rust
/// use si_trace_print::{defn, defx};
/// use si_trace_print::def2ñ;
/// fn func1() {
///     def2ñ!("hello");
/// }
/// fn main() {
///     defn!();
///     func1();
///     defx!();
/// }
/// ```
///
/// prints
///
/// ```text
/// →main:
///     ↔si_trace_print::main::func1: hello
/// ←main:
/// ```
///
/// Uses [`sñ()`].
///
/// For debug builds.
///
/// [`eprintln!`]: eprintln!
/// [function]: crate::function_name::function_name_plus
/// [`sñ()`]: crate::stack::sñ
#[macro_export]
macro_rules! def2ñ {
    (
        $($args:tt)*
    ) => {
        #[cfg(any(debug_assertions,test))]
        eprint!("{}{}: ", $crate::stack::sñ(), $crate::function_name::function_name_plus!(2));
        #[cfg(any(debug_assertions,test))]
        eprintln!($($args)*)
    }
}
pub use def2ñ;

//
// tests
//

#[cfg(test)]
mod tests {
    use crate::stack::stack_offset_set;

    // `p`rintln tests

    #[test]
    fn test_p() {
        println!();
        p!("this printed line");
        p!();
        println!();
    }

    #[test]
    fn test_ponxñ() {
        stack_offset_set(Some(2));
        println!();
        po!("po!");
        pn!("pn!");
        pñ!("pñ!");
        px!("px!");
        println!();
    }

    #[test]
    fn test_pfonxñ() {
        stack_offset_set(Some(2));
        println!();
        pfo!("pfo!");
        pfn!("pfn!");
        pfñ!("pfñ!");
        pfx!("pfx!");
        println!();
    }

    #[test]
    fn test_pf1onxñ() {
        stack_offset_set(Some(2));
        println!();
        pf1o!("pf1o!");
        pf1n!("pf1n!");
        pf1ñ!("pf1ñ!");
        pf1x!("pf1x!");
        println!();
    }

    #[test]
    fn test_pf2onxñ() {
        stack_offset_set(Some(2));
        println!();
        pf2o!("pf2o!");
        pf2n!("pf2n!");
        pf2ñ!("pf2ñ!");
        pf2x!("pf2x!");
        println!();
    }

    // `e`println tests

    #[test]
    fn test_e() {
        eprintln!();
        e!("this printed line should not be indented");
        e!();
        eprintln!();
    }

    #[test]
    fn test_eonxñ() {
        stack_offset_set(Some(2));
        eprintln!();
        eo!("eo!");
        en!("en!");
        eñ!("eñ!");
        ex!("ex!");
        eprintln!();
    }

    #[test]
    fn test_efonxñ() {
        stack_offset_set(Some(2));
        eprintln!();
        efo!("efo!");
        efn!("efn!");
        efñ!("efñ!");
        efx!("efx!");
        eprintln!();
    }

    #[test]
    fn test_ef1onxñ() {
        stack_offset_set(Some(2));
        eprintln!();
        ef1o!("ef1o!");
        ef1n!("ef1n!");
        ef1ñ!("ef1ñ!");
        ef1x!("ef1x!");
        eprintln!();
    }

    #[test]
    fn test_ef2onxñ() {
        stack_offset_set(Some(2));
        eprintln!();
        ef2o!("ef2o!");
        ef2n!("ef2n!");
        ef2ñ!("ef2ñ!");
        ef2x!("ef2x!");
        eprintln!();
    }

    // `d`ebug `p`rintln tests

    #[test]
    fn test_dp() {
        println!();
        dp!("this printed line should not be indented");
        dp!();
        println!();
    }

    #[test]
    fn test_dponxñ() {
        stack_offset_set(Some(2));
        println!();
        dpo!("dpo!");
        dpn!("dpn!");
        dpñ!("dpñ!");
        dpx!("dpx!");
        println!();
    }

    #[test]
    fn test_dpfonxñ() {
        stack_offset_set(Some(2));
        println!();
        dpfo!("dpfo!");
        dpfn!("dpfn!");
        dpfñ!("dpfñ!");
        dpfx!("dpfx!");
        println!();
    }

    #[test]
    fn test_dpf1onxñ() {
        stack_offset_set(Some(2));
        println!();
        dpf1o!("dpf1o!");
        dpf1n!("dpf1n!");
        dpf1ñ!("dpf1ñ!");
        dpf1x!("dpf1x!");
        println!();
    }

    #[test]
    fn test_dpf2onxñ() {
        stack_offset_set(Some(2));
        println!();
        dpf2o!("dpf2o!");
        dpf2n!("dpf2n!");
        dpf2ñ!("dpf2ñ!");
        dpf2x!("dpf2x!");
        println!();
    }

    // `d`ebug `e`println tests

    #[test]
    fn test_de() {
        eprintln!();
        de!("this printed line should not be indented");
        de!();
        eprintln!();
    }

    #[test]
    fn test_deonxñ() {
        stack_offset_set(Some(2));
        eprintln!();
        deo!("deo!");
        den!("den!");
        deñ!("deñ!");
        dex!("dex!");
        eprintln!();
    }

    #[test]
    fn test_defonxñ() {
        stack_offset_set(Some(2));
        eprintln!();
        defo!("defo!");
        defn!("defn!");
        defñ!("defñ!");
        defx!("defx!");
        eprintln!();
    }

    #[test]
    fn test_def1onxñ() {
        stack_offset_set(Some(2));
        eprintln!();
        def1o!("def1o!");
        def1n!("def1n!");
        def1ñ!("def1ñ!");
        def1x!("def1x!");
        eprintln!();
    }

    #[test]
    fn test_def2onxñ() {
        stack_offset_set(Some(2));
        eprintln!();
        def2o!("def2o!");
        def2n!("def2n!");
        def2ñ!("def2ñ!");
        def2x!("def2x!");
        eprintln!();
    }
}
