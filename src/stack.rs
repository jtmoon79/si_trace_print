// src/stack.rs

//! Functions to find current stack depth for indented trace prints.

use std::collections::HashMap;
use std::thread;
use std::thread::{Thread, ThreadId};

extern crate backtrace;

extern crate const_format;
use const_format::concatcp;

extern crate lazy_static;
use lazy_static::lazy_static;

extern crate mut_static;

/// A _stack depth_ counter. Also a _stack offset_ counter.
type StackDepth = usize;

/// Map a `ThreadId` to a `StackDepth`. The private global singleton
/// `STACK_OFFSET_TABLE` is this type.
type MapThreadidSd<'a> = HashMap<ThreadId, StackDepth>;

lazy_static! {
    /// Call `stack_offset_set` to set `STACK_OFFSET_TABLE` once per thread.
    /// Call `stack_offset` to get current stack offset using
    /// `STACK_OFFSET_TABLE`.
    //
    // XXX: no mutex to guard access; it's rarely written to 🤞
    //
    // XXX: a mutable static reference for "complex types" is not allowed in rust
    //      using `lazy_static` and `mut_static` to create one
    //      see https://github.com/tyleo/mut_static#quickstart
    //
    static ref STACK_OFFSET_TABLE: mut_static::MutStatic<MapThreadidSd<'static>> =
        mut_static::MutStatic::new();
}

/// Return current absolute stack depth according to [`backtrace::trace`],
/// including this function `stack_depth`.
/// Users should prefer to use [`stack_offset`].
///
/// [`stack_offset`]: stack_offset
/// [`backtrace::trace`]: https://docs.rs/backtrace/0.3.66/backtrace/fn.trace.html
#[inline(always)]
pub fn stack_depth() -> StackDepth {
    let mut sd: StackDepth = 0;
    backtrace::trace(|_| {
        sd += 1;
        true
    });

    sd
}

/// Return current stack depth _offset_ compared to "original" stack depth.
/// The "original" stack depth should have been recorded at the beginning of the
/// thread by calling [`stack_offset_set`].
///
/// [`stack_offset_set`]: stack_offset_set
pub fn stack_offset() -> StackDepth {
    if !(cfg!(debug_assertions) || cfg!(test)) {
        return 0;
    }
    let mut sd: StackDepth = stack_depth() - 1;
    let sd2: StackDepth = sd; // XXX: copy `sd` to avoid borrow error
    let tid: ThreadId = thread::current().id();
    // XXX: for tests, just set on first call
    if !STACK_OFFSET_TABLE.is_set().unwrap() {
        #[allow(clippy::single_match)]
        match STACK_OFFSET_TABLE.set(MapThreadidSd::new()) {
            Err(err) => {
                eprintln!("ERROR: stack_offset: STACK_OFFSET_TABLE.set failed {:?}", err);
            }
            _ => {}
        }
    }
    let so_table = STACK_OFFSET_TABLE.read().unwrap();
    let so: &usize = so_table.get(&tid).unwrap_or(&sd2);
    if &sd < so {
        return 0;
    }
    sd -= so;

    sd
}

/// Call `stack_offset_set` once in each thread, preferably before using
/// dependent macros (e.g. `so()`, `den()`, etc.) and before calling any
/// functions. Gets a baseline "offset" value (retrieved from [`stack_depth`])
/// and stores it in the private global `STACK_OFFSET_TABLE`.
///
/// A positive value `correction` will move the printed output to the right.
/// If the `correction` is too negative then it will print to the left-most
/// column of the terminal. Negative values are useful for when most of a
/// program runs in a function that is several calls deep.
/// Passing `None` will set `correction` to value `0`.
///
/// For example, the `main` function might
/// call function `intialize` which might call function `run`. The `run`
/// function might do the majority of work (printing). In that case,
/// from `main`, pass a negative offset of `-4` to `stack_offset_set`,
/// i.e. `stack_offset_set(Some(-4))`.
/// This way, printing from function `run` will start at the
/// left-most column (and not be indented to the right).
/// This may improve readability.
///
/// [`stack_depth`]: stack_depth
pub fn stack_offset_set(correction: Option<isize>) {
    //if ! (cfg!(debug_assertions) || cfg!(test)) {
    //    return;
    //}
    let sd_: usize = stack_depth();
    let sdi: isize = (sd_ as isize) - correction.unwrap_or(0);
    let so: usize = std::cmp::max(sdi, 0) as usize;
    let thread_cur: Thread = thread::current();
    let tid: ThreadId = thread_cur.id();
    if !STACK_OFFSET_TABLE.is_set().unwrap() {
        // BUG: multiple simlutaneous calls to `STACK_OFFSET_TABLE.is_set()`
        //      then `STACK_OFFSET_TABLE.set(…)` may cause `.set(…)` to return
        //      an error.
        //      Seen in some calls to `cargo test` with filtering where many
        //      tests call `stack_offset_set`. Needs a mutex.
        #[allow(clippy::single_match)]
        match STACK_OFFSET_TABLE.set(MapThreadidSd::new()) {
            Err(err) => {
                eprintln!("ERROR: stack_offset_set: STACK_OFFSET_TABLE.set failed {:?}", err);
            }
            _ => {}
        }
    }
    if STACK_OFFSET_TABLE.read().unwrap().contains_key(&tid) {
        return;
    }
    STACK_OFFSET_TABLE.write().unwrap().insert(tid, so);
}

const S_0: &str = "";
const S_1: &str = "    ";
const S_2: &str = "        ";
const S_3: &str = "            ";
const S_4: &str = "                ";
const S_5: &str = "                    ";
const S_6: &str = "                        ";
const S_7: &str = "                            ";
const S_8: &str = "                                ";
const S_9: &str = "                                    ";
const S_10: &str = "                                        ";
const S_11: &str = "                                            ";
const S_12: &str = "                                                ";
const S_13: &str = "                                                    ";
const S_14: &str = "                                                        ";
const S_15: &str = "                                                            ";
const S_16: &str = "                                                                ";
const S_17: &str = "                                                                    ";
const S_18: &str = "                                                                        ";
const S_19: &str = "                                                                            ";
#[rustfmt::skip]
const S_20: &str = "                                                                                ";
#[rustfmt::skip]
const S_21: &str = "                                                                                    ";
#[rustfmt::skip]
const S_22: &str = "                                                                                        ";
#[rustfmt::skip]
const S_23: &str = "                                                                                            ";
#[rustfmt::skip]
const S_24: &str = "                                                                                                ";
#[rustfmt::skip]
const S_25: &str = "                                                                                                    ";
#[rustfmt::skip]
const S_26: &str = "                                                                                                        ";
#[rustfmt::skip]
const S_27: &str = "                                                                                                            ";
#[rustfmt::skip]
const S_28: &str = "                                                                                                                ";
#[rustfmt::skip]
const S_29: &str = "                                                                                                                    ";
#[rustfmt::skip]
const S__: &str = "                                                                                                                        ";

/// Return a string of `s`paces a multiple of [`stack_offset()`].
///
/// [`stack_offset()`]: stack_offset
pub fn so() -> &'static str {
    const LEAD: &str = " ";
    let so_ = stack_offset();
    match so_ {
        0 => concatcp!(S_0, LEAD),
        1 => concatcp!(S_1, LEAD),
        2 => concatcp!(S_2, LEAD),
        3 => concatcp!(S_3, LEAD),
        4 => concatcp!(S_4, LEAD),
        5 => concatcp!(S_5, LEAD),
        6 => concatcp!(S_6, LEAD),
        7 => concatcp!(S_7, LEAD),
        8 => concatcp!(S_8, LEAD),
        9 => concatcp!(S_9, LEAD),
        10 => concatcp!(S_10, LEAD),
        11 => concatcp!(S_11, LEAD),
        12 => concatcp!(S_12, LEAD),
        13 => concatcp!(S_13, LEAD),
        14 => concatcp!(S_14, LEAD),
        15 => concatcp!(S_15, LEAD),
        16 => concatcp!(S_16, LEAD),
        17 => concatcp!(S_17, LEAD),
        18 => concatcp!(S_18, LEAD),
        19 => concatcp!(S_19, LEAD),
        20 => concatcp!(S_20, LEAD),
        21 => concatcp!(S_21, LEAD),
        22 => concatcp!(S_22, LEAD),
        23 => concatcp!(S_23, LEAD),
        24 => concatcp!(S_24, LEAD),
        25 => concatcp!(S_25, LEAD),
        26 => concatcp!(S_26, LEAD),
        27 => concatcp!(S_27, LEAD),
        28 => concatcp!(S_28, LEAD),
        29 => concatcp!(S_29, LEAD),
        _ => concatcp!(S__, LEAD),
    }
}

/// Return a string of `s`paces a multiple of [`stack_offset()`] with trailing
/// `→` signifying e`n`tering a function.
///
/// [`stack_offset()`]: stack_offset
pub fn sn() -> &'static str {
    const LEAD: &str = "→";
    let so = stack_offset();
    match so {
        0 => concatcp!(S_0, LEAD),
        1 => concatcp!(S_1, LEAD),
        2 => concatcp!(S_2, LEAD),
        3 => concatcp!(S_3, LEAD),
        4 => concatcp!(S_4, LEAD),
        5 => concatcp!(S_5, LEAD),
        6 => concatcp!(S_6, LEAD),
        7 => concatcp!(S_7, LEAD),
        8 => concatcp!(S_8, LEAD),
        9 => concatcp!(S_9, LEAD),
        10 => concatcp!(S_10, LEAD),
        11 => concatcp!(S_11, LEAD),
        12 => concatcp!(S_12, LEAD),
        13 => concatcp!(S_13, LEAD),
        14 => concatcp!(S_14, LEAD),
        15 => concatcp!(S_15, LEAD),
        16 => concatcp!(S_16, LEAD),
        17 => concatcp!(S_17, LEAD),
        18 => concatcp!(S_18, LEAD),
        19 => concatcp!(S_19, LEAD),
        20 => concatcp!(S_20, LEAD),
        21 => concatcp!(S_21, LEAD),
        22 => concatcp!(S_22, LEAD),
        23 => concatcp!(S_23, LEAD),
        24 => concatcp!(S_24, LEAD),
        25 => concatcp!(S_25, LEAD),
        26 => concatcp!(S_26, LEAD),
        27 => concatcp!(S_27, LEAD),
        28 => concatcp!(S_28, LEAD),
        29 => concatcp!(S_29, LEAD),
        _ => concatcp!(S__, LEAD),
    }
}

/// Return a string of `s`paces a multiple of [`stack_offset()`] with trailing
/// `←` signifying e`x`iting a function.
///
/// [`stack_offset()`]: stack_offset
pub fn sx() -> &'static str {
    const LEAD: &str = "←";
    let so = stack_offset();
    match so {
        0 => concatcp!(S_0, LEAD),
        1 => concatcp!(S_1, LEAD),
        2 => concatcp!(S_2, LEAD),
        3 => concatcp!(S_3, LEAD),
        4 => concatcp!(S_4, LEAD),
        5 => concatcp!(S_5, LEAD),
        6 => concatcp!(S_6, LEAD),
        7 => concatcp!(S_7, LEAD),
        8 => concatcp!(S_8, LEAD),
        9 => concatcp!(S_9, LEAD),
        10 => concatcp!(S_10, LEAD),
        11 => concatcp!(S_11, LEAD),
        12 => concatcp!(S_12, LEAD),
        13 => concatcp!(S_13, LEAD),
        14 => concatcp!(S_14, LEAD),
        15 => concatcp!(S_15, LEAD),
        16 => concatcp!(S_16, LEAD),
        17 => concatcp!(S_17, LEAD),
        18 => concatcp!(S_18, LEAD),
        19 => concatcp!(S_19, LEAD),
        20 => concatcp!(S_20, LEAD),
        21 => concatcp!(S_21, LEAD),
        22 => concatcp!(S_22, LEAD),
        23 => concatcp!(S_23, LEAD),
        24 => concatcp!(S_24, LEAD),
        25 => concatcp!(S_25, LEAD),
        26 => concatcp!(S_26, LEAD),
        27 => concatcp!(S_27, LEAD),
        28 => concatcp!(S_28, LEAD),
        29 => concatcp!(S_29, LEAD),
        _ => concatcp!(S__, LEAD),
    }
}

/// Return a string of `s`paces a multiple of [`stack_offset()`] with trailing
/// `↔` signifying e`n`tering and e`x`iting a function.
///
/// [`stack_offset()`]: stack_offset
pub fn sn͓() -> &'static str {
    const LEAD: &str = "↔";
    let so = stack_offset();
    match so {
        0 => concatcp!(S_0, LEAD),
        1 => concatcp!(S_1, LEAD),
        2 => concatcp!(S_2, LEAD),
        3 => concatcp!(S_3, LEAD),
        4 => concatcp!(S_4, LEAD),
        5 => concatcp!(S_5, LEAD),
        6 => concatcp!(S_6, LEAD),
        7 => concatcp!(S_7, LEAD),
        8 => concatcp!(S_8, LEAD),
        9 => concatcp!(S_9, LEAD),
        10 => concatcp!(S_10, LEAD),
        11 => concatcp!(S_11, LEAD),
        12 => concatcp!(S_12, LEAD),
        13 => concatcp!(S_13, LEAD),
        14 => concatcp!(S_14, LEAD),
        15 => concatcp!(S_15, LEAD),
        16 => concatcp!(S_16, LEAD),
        17 => concatcp!(S_17, LEAD),
        18 => concatcp!(S_18, LEAD),
        19 => concatcp!(S_19, LEAD),
        20 => concatcp!(S_10, LEAD),
        21 => concatcp!(S_21, LEAD),
        22 => concatcp!(S_22, LEAD),
        23 => concatcp!(S_23, LEAD),
        24 => concatcp!(S_24, LEAD),
        25 => concatcp!(S_25, LEAD),
        26 => concatcp!(S_26, LEAD),
        27 => concatcp!(S_27, LEAD),
        28 => concatcp!(S_28, LEAD),
        29 => concatcp!(S_29, LEAD),
        _ => concatcp!(S__, LEAD),
    }
}

#[cfg(test)]
mod tests {
    use super::{sn, sn͓, so, stack_depth, stack_offset, stack_offset_set, sx, StackDepth};

    #[test]
    fn test_stack_depth() {
        let a = stack_depth();
        fn func1() -> StackDepth {
            stack_depth()
        }
        let b = func1();
        assert!(
            b - 1 == a,
            "expected stack depth difference of 1, got stack depth {}, plus a function stack depth {}",
            a,
            b,
        );
    }

    #[test]
    fn test_stack_offset() {
        stack_offset_set(Some(1));
        let a = stack_offset();
        fn func1() -> StackDepth {
            stack_offset()
        }
        let b = func1();
        assert!(
            b - 1 == a,
            "expected stack offset difference of 1, got stack offset {}, plus a function stack offset {}",
            a,
            b,
        );
    }

    #[test]
    fn test_so() {
        so();
    }

    #[test]
    fn test_sn() {
        sn();
    }

    #[test]
    fn test_sx() {
        sx();
    }

    #[test]
    fn test_sn͓() {
        sn͓();
    }
}
