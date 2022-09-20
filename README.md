# si_trace_print<!-- omit in TOC -->

A rust library to print messages indented to stack depth optionally preceded by the function name.

Useful for trace printing function flows.

[![Build status](https://img.shields.io/github/workflow/status/jtmoon79/si_trace_print/Rust?style=flat-square&logo=github)](https://github.com/jtmoon79/si_trace_print/actions?query=workflow%3Arust)
[![docs.rs](https://img.shields.io/docsrs/si_trace_print/latest?badge.svg&style=flat-square&logo=docsdotrs)](https://docs.rs/si_trace_print/latest/s4lib/)
[![codecov.io](https://img.shields.io/codecov/c/github/jtmoon79/si_trace_print/branch?main&token=Q2OXTL7U02&style=flat-square&logo=codecov)](https://codecov.io/gh/jtmoon79/super-si_trace_print)
[![crates.io](https://img.shields.io/crates/v/si_trace_print.svg?style=flat-square&logo=rust)](https://crates.io/crates/si_trace_print)
[![Commits since](https://img.shields.io/github/commits-since/jtmoon79/si_trace_print/latest.svg)](https://img.shields.io/github/commits-since/jtmoon79/si_trace_print/latest.svg?style=flat-square)
[![Requirements Status](https://requires.io/github/jtmoon79/si_trace_print/requirements.svg?branch=main)](https://requires.io/github/jtmoon79/si_trace_print/requirements/?branch=main)
[![License](https://img.shields.io/crates/l/si_trace_print?style=flat-square)](LICENSE)

---

<!-- TOC generated by Markdown All In One -->
- [Use](#use)
  - [Trace-printing example](#trace-printing-example)
  - [Manually setting the indentation](#manually-setting-the-indentation)
- [Shortcomings](#shortcomings)
  - [Slow](#slow)
  - [Release builds](#release-builds)

---

## Use

Add `si_trace_print` entry to the project `Cargo.toml` section `[dependencies]`.

### Trace-printing example

```rust
use si_trace_print::{
    p, pn, po, px, pfn, pfñ, pfo, pfx,
};

fn main() {
    pn!("hello from main");
    po!("main will be doing stuff...");
    func1(3);
    po!("main is done doing stuff...");
    px!("goodbye from main");
}

fn func1(var: usize) {
    pfn!("({:?})", var);
    func2(var + 1);
    pfx!("({:?})", var);
}

fn func2(var: usize) {
    pfn!("({:?})", var);
    pfo!("calling func3...");
    func3();
    pfx!("({:?})", var);
}

fn func3() {
    pfn!();
    func4();
    pfo!("almost complete...");
    pfx!();
}

fn func4() {
    pfñ!("func4 is a short function.");
}
```

should print

```text
→hello from main
 main will be doing stuff...
    →func1: (3)
        →func2: (4)
         func2: calling func3...
            →func3:
                ↔func4: func4 is a short function.
             func3: almost complete...
            ←func3:
        ←func2: (4)
    ←func1: (3)
 main is done doing stuff...
←goodbye from main
```

Most users will want to use the **d**ebug-only printing to std**e**rr.

```rust
use si_trace_print::{
    den, deo, dex, defn, defx,
};

fn main() {
    den!("hello from main");
    deo!("main will be doing stuff...");
    func1(3);
    deo!("main is done doing stuff...");
    dex!("goodbye from main");
}

fn func1(_var: usize) {
    defn!("({:?})", _var);
    defx!("({:?})", _var);
}
```

This printed

```text
$ cargo run
→hello from main
 main will be doing stuff...
    →func1: (3)
    ←func1: (3)
 main is done doing stuff...
←goodbye from main
```

If built with `--release` then the statements are not compiled and nothing would
be printed.

### Manually setting the indentation

The first use of a library macro will set the "original" stack depth.
This is later used to calculate indentation offsets.
If the first use of this library is several functions into a program then
later printing may be lose indentation.

```rust
use si_trace_print::{
    pfo, pfn, pfx, pfñ,
};

fn main() {
    func1(3);
    pfx!("goodbye from main (this is not indented!)");
}

fn func1(var: usize) {
    func2(var);
    pfñ!("({:?}) (this is not indented!)", var);
}

fn func2(var: usize) {
    // this is the first call to a si_trace_print function
    // the "original" stack offset will be set from here
    pfn!("({:?})", var);
    pfo!("stack_depth {:?}, stack_offset {:?}", stack_depth(), stack_offset());
    pfx!("({:?})", var);
}
```

prints poorly indented output

```text
→func2: (3)
 func2: stack_depth 15, stack_offset 0
←func2: (3)
↔func1: (3) (this is not indented!)
←main: goodbye from main (this is not indented!)
```

Explictly call `stack_offset_set` near the beginning of the thread.

```rust
use si_trace_print::{
    pfo, pfn, pfx, pfñ,
};

fn main() {
    // the "original" stack offset will be set from here
    stack_offset_set(None);
    func1(3);
    pfx!("goodbye from main");
}

fn func1(var: usize) {
    func2(var);
    pfñ!("stack_depth {:?}, stack_offset {:?}", stack_depth(), stack_offset());
}

fn func2(var: usize) {
    pfn!("({:?})", var);
    pfo!("stack_depth {:?}, stack_offset {:?}", stack_depth(), stack_offset());
    pfx!("({:?})", var);
}
```

this printed

```text
            →func2: (3)
             func2: stack_depth 15, stack_offset 2
            ←func2: (3)
        ↔func1: stack_depth 14, stack_offset 1
    ←main: goodbye from main
```

The indentation is improved but is too far indented.
The indentation amount to pass to `stack_offset_set` can be somewhat unpredictable.
It depends on build settings and which thread is running, among other things.
In this case, experimentation revealed value `-1` to be best:

```rust
// ...
fn main() {
    stack_offset_set(Some(-1));
// ...
```

this printed

```text
        →func2: (3)
         func2: stack_depth 15, stack_offset 1
        ←func2: (3)
    ↔func1: stack_depth 14, stack_offset 0
←main: goodbye from main
```

## Shortcomings

### Slow

This trace function may significantly slow a program. It is recommended to
use the **d**ebug version of provided macros.

### Release builds

The calculation of function depth depends on stack frames counted by
[`backtrace::trace`]. In `--release` builds or under other optimization profiles, some functions may be optimized inline.
The count of stack frames may not change among function calls.
This means the printed indentation will not reflect function call depth.
This can be forcibly avoided by adding attribute `#[inline(never)]` to such
functions.

<!-- links -->

[`backtrace::trace`]: https://docs.rs/backtrace/0.3.66/backtrace/fn.trace.html

---

<a href="https://stackexchange.com/users/216253/jamesthomasmoon1979"><img src="https://stackexchange.com/users/flair/216253.png" width="208" height="58" alt="profile for JamesThomasMoon1979 on Stack Exchange, a network of free, community-driven Q&amp;A sites" title="profile for JamesThomasMoon1979 on Stack Exchange, a network of free, community-driven Q&amp;A sites" /></a>
