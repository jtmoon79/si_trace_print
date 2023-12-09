// src/lib.rs

//! Macros for printing stack-indented trace-like print statements.
//!
//! Library users should use macros provided in [`printers`] (which are
//! also listed here).
//!
//! ```rust
//! use si_trace_print::{
//!     den, deo, dex, defn, defo, defx,
//! };
//!
//! fn main() {
//!     den!("hello from main");
//!     deo!("main will be doing stuff...");
//!     func1(3);
//!     deo!("main is done doing stuff.");
//!     dex!("goodbye from main");
//! }
//!
//! fn func1(_var: usize) {
//!     defn!("({:?})", _var);
//!     defo!("doing even more stuff...");
//!     defx!();
//! }
//! ```
//!
//! this should print to *stderr*
//!
//! ```text,ignore
//! $ cargo run
//! →hello from main
//!  main will be doing stuff...
//!     →func1: (3)
//!      func1: doing even more stuff...
//!     ←func1:
//!  main is done doing stuff.
//! ←goodbye from main
//! ```
//!
//! An example using a variety of the available ***p***rintln macros. These compile
//! into debug *and release* builds and print to *stdout*.
//!
//! ```rust
//! extern crate si_trace_print;
//! use si_trace_print::{pf1n, pf2n, pfn, pn, po, px};
//!
//! fn main() {
//!     pn!("hello from main");
//!     pfn!("hello again from main");
//!     pf1n!("hello again from main!");
//!     pf2n!("HELLO AGAIN FROM MAIN!!!");
//!     po!("main will be doing stuff...");
//!     mod1::mod2::func1(3);
//!     po!("main is done doing stuff...");
//!     px!("goodbye from main");
//! }
//!
//! mod mod1 {
//!     pub mod mod2 {
//!         use si_trace_print::{
//!             pf1n, pf1o, pf1x, pf1ñ, pf2n, pf2o, pf2x, pf2ñ, pfn, pfo, pfx, pfñ, pñ,
//!         };
//!         pub fn func1(var: usize) {
//!             pf1n!("({:?})", var);
//!             pf1o!("func1 calling func2...");
//!             func2(var + 1);
//!             pf1x!("({:?})", var);
//!         }
//!         fn func2(var: usize) {
//!             pf2n!("({:?})", var);
//!             pf2o!("calling func3...");
//!             func3();
//!             pf2x!("({:?})", var);
//!         }
//!         fn func3() {
//!             pfn!();
//!             func4();
//!             pfo!("almost complete...");
//!             pfx!();
//!         }
//!         fn func4() {
//!             pñ!("func4 is a short function.");
//!             pfñ!("func4 is a short function.");
//!             pf1ñ!("func4 is a short function.");
//!             pf2ñ!("func4 is a short function.");
//!         }
//!     }
//! }
//! ```
//!
//! should print to *stdout*
//!
//! ```text,ignore
//! →hello from main
//! →main: hello again from main
//! →main: hello again from main!
//! →main: HELLO AGAIN FROM MAIN!!!
//!  main will be doing stuff...
//!     →mod2::func1: (3)
//!      mod2::func1: func1 calling func2...
//!         →mod1::mod2::func2: (4)
//!          mod1::mod2::func2: calling func3...
//!             →func3:
//!                 ↔func4 is a short function.
//!                 ↔func4: func4 is a short function.
//!                 ↔mod2::func4: func4 is a short function.
//!                 ↔mod1::mod2::func4: func4 is a short function.
//!              func3: almost complete...
//!             ←func3:
//!         ←mod1::mod2::func2: (4)
//!     ←mod2::func1: (3)
//!  main is done doing stuff...
//! ←goodbye from main
//! ```
//!
//! ## Multi-threaded printing and the global lock
//!
//! To avoid interleaved printing among threads, surround your `println!` or
//! `eprintln!` statements in the global lock guard [`GLOBAL_LOCK_PRINTER`].
//! Helper functions [`print_guard`] or [`debug_print_guard`] are provided.
//!
//! ```rust
//! use ::si_trace_print::{efn, efx};
//! use ::si_trace_print::printers::print_guard;
//!
//! let mut handles: Vec<std::thread::JoinHandle<()>> = vec![];
//! for n in 0..10 {
//!     let h = std::thread::spawn(move || {
//!         efn!("time to do some work...");
//!         // ...do some work...
//!         let result = n;
//!         // ...a few moments later...
//!         // get the guard before printing
//!         let guard = print_guard();
//!         println!("{}", result);
//!         // drop the guard before calling another si_trace_print macro
//!         drop(guard);
//!         efx!("work complete");
//!     });
//!     handles.push(h);
//! }
//! for h in handles {
//!     h.join().unwrap();
//! }
//! ```
//!
//! If using `si_trace_print` ***d***ebug macros (the recommended way to use
//! this module) then call the [`debug_print_guard`] helper function.
//!
//! In this code example, `guard` is a `MutexGuard` in debug builds and
//! a `()` in non-debug release builds (which is likely optimized away).
//!
//! ```rust
//! use ::si_trace_print::{defn, defx};
//! use ::si_trace_print::printers::debug_print_guard;
//!
//! let mut handles: Vec<std::thread::JoinHandle<()>> = vec![];
//! for n in 0..10 {
//!     let h = std::thread::spawn(move || {
//!         defn!("time to do some work...");
//!         // ...do some work...
//!         let result = n;
//!         // ...a little while later...
//!         // get the guard before printing
//!         let guard = debug_print_guard();
//!         println!("{}", result);
//!         // explicitly drop the guard before calling another si_trace_print macro
//!         drop(guard);
//!         defx!("work complete");
//!     });
//!     handles.push(h);
//! }
//! for h in handles {
//!     h.join().unwrap();
//! }
//! ```
//!
//! [`printers`]: crate::printers
//! [`GLOBAL_LOCK_PRINTER`]: struct@crate::printers::GLOBAL_LOCK_PRINTER
//! [`print_guard`]: crate::printers::print_guard
//! [`debug_print_guard`]: crate::printers::debug_print_guard

#![allow(uncommon_codepoints)]

pub mod function_name;
pub mod printers;
pub mod stack;
