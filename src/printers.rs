// src/printers.rs

//! Macros for trace printing with printing stack offset indentation.

//
// `p`rintln
//

/// **p**rintln!
///
/// For completeness, wrap [`println!`].
///
/// ```rust
/// use si_trace_print::{efn, efx};
/// use si_trace_print::p;
/// fn func1() {
///     p!("hello");
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
/// [`println!`]: println!
#[macro_export]
macro_rules! p {
    (
        $($args:tt)*
    ) => {
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
/// use si_trace_print::{efn, efx};
/// use si_trace_print::po;
/// fn func1() {
///     po!("hello");
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
/// use si_trace_print::{efn, efx};
/// use si_trace_print::pn;
/// fn func1() {
///     pn!("hello");
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
/// use si_trace_print::{efn, efx};
/// use si_trace_print::px;
/// fn func1() {
///     px!("hello");
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
/// use si_trace_print::{efn, efx};
/// use si_trace_print::pñ;
/// fn func1() {
///     pñ!("hello");
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
/// To signify printing wihtin a function.
/// Use to [`println!`] in a [function].
///
/// ```rust
/// use si_trace_print::{efn, efx};
/// use si_trace_print::pfo;
/// fn func1() {
///     pfo!("hello");
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
/// use si_trace_print::{efn, efx};
/// use si_trace_print::pfn;
/// fn func1() {
///     pfn!("hello");
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
/// use si_trace_print::{efn, efx};
/// use si_trace_print::pfn;
/// fn func1() {
///     pfn!("hello");
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
/// use si_trace_print::{efn, efx};
/// use si_trace_print::pfn;
/// fn func1() {
///     pfn!("hello");
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

/// **e**println! in a **f**unction with **o**ffset.
///
/// To signify printing wihtin a function.
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
// **d**ebug `p`rintln
//

/// **d**ebug **p**rintln!
///
/// ```rust
/// use si_trace_print::{efn, efx};
/// use si_trace_print::dp;
/// fn func1() {
///     dp!("hello");
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
/// use si_trace_print::{efn, efx};
/// use si_trace_print::dpo;
/// fn func1() {
///     dpo!("hello");
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
        eprint!("{}", $crate::stack::so());
        #[cfg(any(debug_assertions,test))]
        eprintln!($($args)*)
    }
}
pub use dpo;

/// **d**ebug **p**rintln! when e**n**tering.
///
/// To signify entering a function.
/// Use this as the first [`println!`] in a function.
///
/// ```rust
/// use si_trace_print::{efn, efx};
/// use si_trace_print::dpn;
/// fn func1() {
///     dpn!("hello");
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
        eprint!("{}", $crate::stack::sn());
        #[cfg(any(debug_assertions,test))]
        eprintln!($($args)*)
    }
}
pub use dpn;

/// **d**ebug **p**rintln! when e**x**iting.
///
/// To signify exiting a function.
/// Use this as the last [`println!`] in a function.
///
/// ```rust
/// use si_trace_print::{efn, efx};
/// use si_trace_print::dpx;
/// fn func1() {
///     dpx!("hello");
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
        eprint!("{}", $crate::stack::sx());
        #[cfg(any(debug_assertions,test))]
        eprintln!($($args)*)
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
/// use si_trace_print::{efn, efx};
/// use si_trace_print::dpñ;
/// fn func1() {
///     dpñ!("hello");
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
        eprint!("{}", $crate::stack::sñ());
        #[cfg(any(debug_assertions,test))]
        eprintln!($($args)*)
    }
}
pub use dpñ;

/// **d**ebug **p**rintln! in a **f**unction with **o**ffset.
///
/// To signify printing wihtin a function.
/// Use to [`println!`] in a [function].
///
/// ```rust
/// use si_trace_print::{efn, efx};
/// use si_trace_print::dpfo;
/// fn func1() {
///     dpfo!("hello");
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
        eprint!("{}{}: ", $crate::stack::so(), $crate::function_name::function_name!());
        #[cfg(any(debug_assertions,test))]
        eprintln!($($args)*)
    }
}
pub use dpfo;

/// **d**ebug **p**rintln! in a **f**unction when e**n**tering.
///
/// To signify entering a function.
/// Use this as the first [`println!`] in a [function].
///
/// ```rust
/// use si_trace_print::{efn, efx};
/// use si_trace_print::dpfn;
/// fn func1() {
///     dpfn!("hello");
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
        eprint!("{}{}: ", $crate::stack::sn(), $crate::function_name::function_name!());
        #[cfg(any(debug_assertions,test))]
        eprintln!($($args)*)
    }
}
pub use dpfn;

/// **d**ebug **p**rintln! in a **f**unction when e**x**iting.
///
/// To signify exiting a function.
/// Use this as the last [`println!`] in a [function].
///
/// ```rust
/// use si_trace_print::{efn, efx};
/// use si_trace_print::dpfx;
/// fn func1() {
///     dpfx!("hello");
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
        eprint!("{}{}: ", $crate::stack::sx(), $crate::function_name::function_name!());
        #[cfg(any(debug_assertions,test))]
        eprintln!($($args)*)
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
/// use si_trace_print::{efn, efx};
/// use si_trace_print::dpfñ;
/// fn func1() {
///     dpfñ!("hello");
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
        eprint!("{}{}: ", $crate::stack::sñ(), $crate::function_name::function_name!());
        #[cfg(any(debug_assertions,test))]
        eprintln!($($args)*)
    }
}
pub use dpfñ;

//
// **d**ebug `e`println
//

/// **d**ebug **e**println!
///
/// For completeness, wrap [`eprintln!`].
///
/// ```rust
/// use si_trace_print::{efn, efx};
/// use si_trace_print::de;
/// fn func1() {
///     de!("hello");
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
/// use si_trace_print::{efn, efx};
/// use si_trace_print::deo;
/// fn func1() {
///     deo!("hello");
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
/// use si_trace_print::{efn, efx};
/// use si_trace_print::den;
/// fn func1() {
///     den!("hello");
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
/// use si_trace_print::{efn, efx};
/// use si_trace_print::dex;
/// fn func1() {
///     dex!("hello");
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
/// use si_trace_print::{efn, efx};
/// use si_trace_print::deñ;
/// fn func1() {
///     deñ!("hello");
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

/// **d**ebug **e**println! in a **f**unction with **o**ffset.
///
/// To signify printing wihtin a function.
/// Use to [`eprintln!`] in a [function].
///
/// ```rust
/// use si_trace_print::{efn, efx};
/// use si_trace_print::defo;
/// fn func1() {
///     defo!("hello");
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
/// use si_trace_print::{efn, efx};
/// use si_trace_print::defn;
/// fn func1() {
///     defn!("hello");
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
/// use si_trace_print::{efn, efx};
/// use si_trace_print::defx;
/// fn func1() {
///     defx!("hello");
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
/// use si_trace_print::{efn, efx};
/// use si_trace_print::defñ;
/// fn func1() {
///     defñ!("hello");
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
    fn test_pñ() {
        stack_offset_set(Some(2));
        println!();
        pñ!(
            "this printed line should be indented with arg \"{}\"",
            function_name_full!()
        );
        pñ!();
        println!();
    }

    #[test]
    fn test_pfo() {
        stack_offset_set(Some(2));
        println!();
        pfo!(
            "this printed line should be indented and preceded with function name 'test_pfo', with arg \"{}\"",
            function_name_full!()
        );
        pfo!();
        println!();
    }

    #[test]
    fn test_pfn() {
        stack_offset_set(Some(2));
        println!();
        pfn!(
            "this printed line should be indented and preceded with function name 'test_pfn', with arg \"{}\"",
            function_name_full!()
        );
        pfn!();
        println!();
    }

    #[test]
    fn test_pfx() {
        stack_offset_set(Some(2));
        println!();
        pfx!(
            "this printed line should be indented and preceded with function name 'test_pfx', with arg \"{}\"",
            function_name_full!()
        );
        pfx!();
        println!();
    }

    #[test]
    fn test_pfñ() {
        stack_offset_set(Some(2));
        println!();
        pfñ!(
            "this printed line should be indented and preceded with function name 'test_pfñ', with arg \"{}\"",
            function_name_full!()
        );
        pfñ!();
        println!();
    }

    #[test]
    fn test_ponxñf() {
        stack_offset_set(Some(2));
        println!();
        pfn!("pfn!");
        pfo!("pfo!");
        pfñ!("pfñ!");
        pfx!("pfx!");
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
    fn test_eñ() {
        stack_offset_set(Some(2));
        eprintln!();
        eñ!(
            "this printed line should be indented with arg \"{}\"",
            function_name_full!()
        );
        eñ!();
        eprintln!();
    }

    #[test]
    fn test_eonxñ() {
        stack_offset_set(Some(2));
        eprintln!();
        en!("en!");
        eo!("eo!");
        eñ!("eñ!");
        ex!("ex!");
        eprintln!();
    }

    #[test]
    fn test_efo() {
        stack_offset_set(Some(2));
        eprintln!();
        efo!(
            "this printed line should be indented and preceded with function name 'test_efo', with arg \"{}\"",
            function_name_full!()
        );
        efo!();
        eprintln!();
    }

    #[test]
    fn test_efn() {
        stack_offset_set(Some(2));
        eprintln!();
        efn!(
            "this printed line should be indented and preceded with function name 'test_efn', with arg \"{}\"",
            function_name_full!()
        );
        efn!();
        eprintln!();
    }

    #[test]
    fn test_efx() {
        stack_offset_set(Some(2));
        eprintln!();
        efx!(
            "this printed line should be indented and preceded with function name 'test_efx', with arg \"{}\"",
            function_name_full!()
        );
        efx!();
        eprintln!();
    }

    #[test]
    fn test_efñ() {
        stack_offset_set(Some(2));
        eprintln!();
        efñ!(
            "this printed line should be indented and preceded with function name 'test_efñ', with arg \"{}\"",
            function_name_full!()
        );
        efñ!();
        eprintln!();
    }

    #[test]
    fn test_eonxñf() {
        stack_offset_set(Some(2));
        eprintln!();
        efn!("efn!");
        efo!("efo!");
        efñ!("efñ!");
        efx!("efx!");
        eprintln!();
    }

    // **d**ebug `p`rintln tests

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
    fn test_dpñ() {
        stack_offset_set(Some(2));
        println!();
        dpñ!(
            "this printed line should be indented with arg \"{}\"",
            function_name_full!()
        );
        dpñ!();
        println!();
    }

    #[test]
    fn test_dponxñ() {
        stack_offset_set(Some(2));
        println!();
        dpn!("dpn!");
        dpo!("dpo!");
        dpñ!("dpñ!");
        dpx!("dpx!");
        println!();
    }

    #[test]
    fn test_dpfo() {
        stack_offset_set(Some(2));
        println!();
        dpfo!(
            "this printed line should be indented and preceded with function name 'test_dpfo', with arg \"{}\"",
            function_name_full!()
        );
        dpfo!();
        println!();
    }

    #[test]
    fn test_dpfn() {
        stack_offset_set(Some(2));
        println!();
        dpfn!(
            "this printed line should be indented and preceded with function name 'test_dpfn', with arg \"{}\"",
            function_name_full!()
        );
        dpfn!();
        println!();
    }

    #[test]
    fn test_dpfx() {
        stack_offset_set(Some(2));
        println!();
        dpfx!(
            "this printed line should be indented and preceded with function name 'test_dpfx', with arg \"{}\"",
            function_name_full!()
        );
        dpfx!();
        println!();
    }

    #[test]
    fn test_dpfñ() {
        stack_offset_set(Some(2));
        println!();
        dpfñ!(
            "this printed line should be indented and preceded with function name 'test_dpfñ', with arg \"{}\"",
            function_name_full!()
        );
        dpfñ!();
        println!();
    }

    #[test]
    fn test_dponxñf() {
        stack_offset_set(Some(2));
        println!();
        dpfn!("dpfn!");
        dpfo!("dpfo!");
        dpfñ!("dpfñ!");
        dpfx!("dpfx!");
        println!();
    }

    // **d**ebug `e`println tests

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
    fn test_deñ() {
        stack_offset_set(Some(2));
        eprintln!();
        deñ!(
            "this printed line should be indented with arg \"{}\"",
            function_name_full!()
        );
        deñ!();
        eprintln!();
    }

    #[test]
    fn test_deonxñ() {
        stack_offset_set(Some(2));
        eprintln!();
        den!("den!");
        deo!("deo!");
        deñ!("deñ!");
        dex!("dex!");
        eprintln!();
    }

    #[test]
    fn test_defo() {
        stack_offset_set(Some(2));
        eprintln!();
        defo!(
            "this printed line should be indented and preceded with function name 'test_defo', with arg \"{}\"",
            function_name_full!()
        );
        defo!();
        eprintln!();
    }

    #[test]
    fn test_defn() {
        stack_offset_set(Some(2));
        eprintln!();
        defn!(
            "this printed line should be indented and preceded with function name 'test_defn', with arg \"{}\"",
            function_name_full!()
        );
        defn!();
        eprintln!();
    }

    #[test]
    fn test_defx() {
        stack_offset_set(Some(2));
        eprintln!();
        defx!(
            "this printed line should be indented and preceded with function name 'test_defx', with arg \"{}\"",
            function_name_full!()
        );
        defx!();
        eprintln!();
    }

    #[test]
    fn test_defñ() {
        stack_offset_set(Some(2));
        eprintln!();
        defñ!(
            "this printed line should be indented and preceded with function name 'test_defñ', with arg \"{}\"",
            function_name_full!()
        );
        defñ!();
        eprintln!();
    }

    #[test]
    fn test_deonxñf() {
        stack_offset_set(Some(2));
        eprintln!();
        defn!("defn!");
        defo!("defo!");
        defñ!("defñ!");
        defx!("defx!");
        eprintln!();
    }
}
