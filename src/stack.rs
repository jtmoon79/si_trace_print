// src/stack.rs

//! Functions to store a stack offset for indented trace prints and
//! return the appropriate preprint `str`.
//!
//! Library users should use macros provided in [`printers`].
//!
//! Function `stack_offset_set` is to force the setting of the "original" stack
//! depth for a thread.
//!
//! Functions `sn`, `so`, `sx`, and `sñ` return a `&str` to preprint before
//! tracing messages. These functions are used by macros in `printers`.
//!
//! The stack-based indentation amount depend on optimization settings.
//! In an optimized build, an inlined function will not add to the stack
//! depth.
//! Adding explicit `inline` attributes may fix such a problem.
//! According to [_The Rust Performance Book_]:
//!
//! > Inline attributes do not guarantee that a function is inlined or not
//! > inlined, but in practice, `#[inline(always)]` will cause inlining in all
//! > but the most exceptional cases.
//!
//! At worst, the indentation may not change and all printing will align
//! at the same column.
//!
//! Lack of indentation may occur in a `--release` build or other optimized
//! builds.
//!
//! [`printers`]: crate::printers
//! [_The Rust Performance Book_]: https://nnethercote.github.io/perf-book/inlining.html

use std::collections::HashMap;
use std::thread;
use std::thread::ThreadId;

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
    ///
    // XXX: a mutable static reference for "complex types" is not allowed in
    //      rust.
    //      The `STACK_OFFSET_TABLE` instance is a combined `lazy_static` and
    //      `mut_static`.
    //      See https://github.com/tyleo/mut_static#quickstart
    //
    static ref STACK_OFFSET_TABLE: mut_static::MutStatic<MapThreadidSd<'static>> = {
        mut_static::MutStatic::new()
    };
}

/// Return current absolute stack depth according to [`backtrace::trace`],
/// not including this function `stack_depth`.
/// Users should prefer to use [`stack_offset`].
///
/// `stack_depth` decrements it's own stack depth by implied result of
/// attribute `#[inline(always)]`.
///
/// [`stack_offset`]: stack_offset
/// [`backtrace::trace`]: https://docs.rs/backtrace/0.3.66/backtrace/fn.trace.html
/// [_The Rust Performance Book_]: https://nnethercote.github.io/perf-book/inlining.html
#[inline(always)]
fn stack_depth() -> StackDepth {
    let mut sd: StackDepth = 0;
    backtrace::trace(|_| {
        sd += 1;
        true
    });

    sd
}

/// Make sure the global STACK_OFFSET_TABLE has been created.
#[inline(never)]
fn stack_offset_table_create() -> bool {
    match STACK_OFFSET_TABLE.is_set() {
        Ok(false) => {
            // STACK_OFFSET_TABLE not yet created so create it.
            #[allow(clippy::single_match)]
            match STACK_OFFSET_TABLE.set(MapThreadidSd::new()) {
                Err(err) => {
                    // rare case that depends on runtime race conditions.
                    if matches!(err.kind(), mut_static::ErrorKind::StaticIsAlreadySet) {
                        // this is fine
                        return true;
                    }
                    // this is not fine
                    let tid = thread::current().id();
                    eprintln!(
                        "ERROR: stack_offset: STACK_OFFSET_TABLE.set failed in thread {:?}; {:?}",
                        tid, err
                    );
                    return false;
                }
                _ => {}
            }
        }
        Ok(true) => {}
        Err(err) => {
            panic!("STACK_OFFSET_TABLE.is_set() failed {}", err);
        }
    }

    true
}

/// Return current stack depth _offset_ compared to "original" stack depth.
///
/// The "original" stack depth is recorded by either:
/// - an explicit call to [`stack_offset_set`].
/// - an implicit call to [`stack_offset_set`] via calling this `stack_offset`.
///
/// [`stack_offset_set`]: stack_offset_set
#[inline(never)]
fn stack_offset() -> StackDepth {
    // call `stack_offset_set` which will both check the table exists
    // and has an offset entry for this thread. If an entry is not already
    // present than initialize with `1` correction, to correct this function
    // `stack_offset`.
    stack_offset_set(Some(1));
    let mut sd: StackDepth = stack_depth();
    if sd > 0 {
        sd -= 1;
    }

    let tid: ThreadId = thread::current().id();
    let so_table = match STACK_OFFSET_TABLE.read() {
        Ok(table) => table,
        Err(_err) => {
            //eprintln!("ERROR: stack_offset: STACK_OFFSET_TABLE.read failed {:?}", err);
            return 0;
        }
    };
    let sd_: StackDepth = sd; // XXX: copy `sd` to avoid borrow error
                              // "original" stack offset
    let so: &usize = so_table.get(&tid).unwrap_or(&sd_);
    if &sd < so {
        return 0;
    }
    sd -= so;

    sd
}

/// Function `stack_offset_set` gets a baseline "offset" value
/// (retrieved from private function `stack_depth`) and stores it in the
/// private global `STACK_OFFSET_TABLE`.
/// `stack_offset_set` can be explicitly called to force
/// the "original" stack depth value to be set.
/// This explicit call must be done before calling dependent macros
/// (e.g. `po()`, `den()`, etc.) and before calling any dependent
/// functions (e.g. `so()`), otherwise the call will be ignored.
/// Function `stack_offset_set` is implicitly called by the macros in
/// [`printers`].
///
/// Only the first call to `stack_offset_set` within a thread is used.
/// Subsequent calls are ignored.
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
/// [`printers`]: crate::printers
#[inline(never)]
pub fn stack_offset_set(correction: Option<isize>) {
    if !stack_offset_table_create() {
        return;
    }
    let tid: ThreadId = thread::current().id();
    {
        if STACK_OFFSET_TABLE.read().unwrap().contains_key(&tid) {
            // only the first call to `stack_offset_set` is used, ignore further
            // calls
            return;
        }
    }
    let mut sd: StackDepth = stack_depth();
    if sd > 0 {
        // remove this function `stack_offset_set` stack frame depth
        sd -= 1;
    }
    let sdi: isize = (sd as isize) - correction.unwrap_or(0);
    // set the "original" stack offset
    let so: StackDepth = std::cmp::max(sdi, 0) as StackDepth;
    match STACK_OFFSET_TABLE.write() {
        Ok(mut table) => {
            table.insert(tid, so);
        }
        Err(_err) => {}
    }
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

/// Return a string of **s**paces that is a multiple of the current
/// stack offset with one trailing space.
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

/// Return a string of **s**paces that is a multiple of the current
/// stack offset with trailing `→` signifying e**n**tering a function.
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

/// Return a string of **s**paces that is a multiple of the current
/// stack offset with trailing `←` signifying e**x**iting a function.
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

/// Return a string of **s**paces that is a multiple of the current
/// stack_offset with trailing `↔` signifying e**n**tering and e**x**iting
/// a function.
///
/// [`stack_offset()`]: stack_offset
pub fn sñ() -> &'static str {
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
    use super::{sn, so, stack_depth, stack_offset, stack_offset_set, sx, sñ, StackDepth};

    #[test]
    fn test_stack_depth() {
        let a = stack_depth();
        fn func1() -> StackDepth {
            stack_depth()
        }
        fn func2() -> StackDepth {
            stack_depth()
        }
        let b = func1();
        let c = func2();
        assert!(
            b - 1 == a,
            "expected stack depth difference of 1, got stack depth {}, plus a function stack depth {}",
            a,
            b,
        );
        assert!(b == c, "expected same, got stack depths {} {}", b, c,);
    }

    #[test]
    fn test_stack_offset_a_b() {
        let a = stack_offset();
        fn func1() -> StackDepth {
            stack_offset()
        }
        let b = func1();
        assert!(
            b == a + 1,
            "expected stack offset difference of 1, got stack offset {}, plus a function stack offset {}",
            a,
            b,
        );
    }

    #[test]
    fn test_stack_offset_a_b_c() {
        let a = stack_offset();
        fn func1() -> StackDepth {
            stack_offset()
        }
        fn func2() -> StackDepth {
            stack_offset()
        }
        let b = func1();
        let c = func2();
        assert!(
            b == a + 1,
            "expected stack offset difference of 1, got stack offset {}, plus a function stack offset {}",
            a,
            b,
        );
        assert!(b == c, "expected same, got stack depths {} {}", b, c,);
    }

    #[test]
    fn test_stack_offset_set_none() {
        stack_offset_set(None);
        let a = stack_offset();
        fn func1() -> StackDepth {
            stack_offset()
        }
        let b = func1();
        assert!(
            b == a + 1,
            "expected stack offset difference of 1, got stack offset {}, plus a function stack offset {}",
            a,
            b,
        );
    }

    #[test]
    fn test_stack_offset_set_none_999() {
        stack_offset_set(None);
        let a = stack_offset();
        fn func1() -> StackDepth {
            // attempt to set again
            stack_offset_set(Some(999));
            stack_offset()
        }
        let b = func1();
        assert!(
            b == a + 1,
            "expected stack offset difference of 1, got stack offset {}, plus a function stack offset {}",
            a,
            b,
        );
    }

    #[test]
    fn test_stack_offset_set_multiple_threads() {
        let mut handles = Vec::<std::thread::JoinHandle<()>>::new();

        for _i in 1..99 {
            let handle = std::thread::spawn(|| {
                stack_offset_set(None);
                let a = stack_offset();
                fn func1() -> StackDepth {
                    stack_offset()
                }
                let b = func1();
                assert!(
                    b == a + 1,
                    "expected stack offset difference of 1, got stack offset {}, plus a function stack offset {}",
                    a,
                    b,
                );
            });
            handles.push(handle);
        }
        for handle in handles.into_iter() {
            match handle.join() {
                Ok(_) => {}
                Err(err) => {
                    panic!("handle.join failed {:?}", err);
                }
            }
        }
    }

    #[test]
    fn test_stack_offset_set_10() {
        stack_offset_set(Some(10));
        let a = stack_offset();
        fn func1() -> StackDepth {
            stack_offset()
        }
        let b = func1();
        assert!(
            b == a + 1,
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
    fn test_sñ() {
        sñ();
    }
}
