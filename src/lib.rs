// src/lib.rs

//! Macros for printing stack-indented trace-like print statements.
//!
//! Library users should use macros provided in [`printers`] (which are
//! also listed here).
//!
//! [`printers`]: crate::printers

#![allow(uncommon_codepoints)]

pub mod function_name;
pub mod printers;
pub mod stack;
