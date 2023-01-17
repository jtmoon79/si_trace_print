# si_trace_print<!-- omit in toc -->

***s***tack ***i***ndented ***trace*** ***print***ing;
a simple rust library to print messages indented to stack depth optionally preceded by the function name.

An "entry-level" tracing library that prints function flows in a simple manual
approach.

[![Build status](https://img.shields.io/github/actions/workflow/status/jtmoon79/si_trace_print/rust.yml?branch=main&style=flat-square&logo=github)](https://github.com/jtmoon79/si_trace_print/actions?query=workflow%3Arust)
[![docs.rs](https://img.shields.io/docsrs/si_trace_print/latest?badge.svg&style=flat-square&logo=docsdotrs)](https://docs.rs/si_trace_print/latest/)
[![codecov.io](https://img.shields.io/codecov/c/github/jtmoon79/si_trace_print/branch?main&token=Q2OXTL7U02&style=flat-square&logo=codecov)](https://codecov.io/gh/jtmoon79/si_trace_print)
[![crates.io](https://img.shields.io/crates/v/si_trace_print.svg?style=flat-square&logo=rust)](https://crates.io/crates/si_trace_print)
[![Commits since](https://img.shields.io/github/commits-since/jtmoon79/si_trace_print/latest.svg?style=flat-square)](https://github.com/jtmoon79/si_trace_print/commits/main)
[![License](https://img.shields.io/crates/l/si_trace_print?style=flat-square)](https://github.com/jtmoon79/si_trace_print/blob/main/LICENSE)

---

<!-- TOC generated by Markdown All In One -->
- [About](#about)
- [Use](#use)
  - [Trace-printing example](#trace-printing-example)
  - [Manually setting the indentation](#manually-setting-the-indentation)
- [Shortcomings](#shortcomings)
  - [Slow](#slow)
  - [Release builds](#release-builds)
  - [Code Clutter](#code-clutter)
  - [Hogs *stdout* Or *stderr*](#hogs-stdout-or-stderr)
- [Other tracing crates](#other-tracing-crates)

---

## About

The aim of `si_trace_print` is to be a simple library to aid developers manually
reviewing singular program runs. It is simple to use; not a framework, does
not require compiler changes, no new Computer Science theories must be studied
to understand how it works.
It's best suited for easily adding rudimentary tracing in debug builds (though
it may also be used in release builds) without the burden of adhering to a
"tracing framework" or adding unusual build parameters.

`si_trace_print` is a good "entry-level" tracing library.

## Use

Add `si_trace_print` entry to the project `Cargo.toml` section `[dependencies]`.

### Trace-printing example

The most common use will likely be ***d***ebug-only ***e***println with
a preceding ***f***unction name.

```rust
use si_trace_print::{
    den, deo, dex, defn, defx,
};

fn main() {
    den!("hello from main");
    deo!("main will be doing stuff...");
    func1(3);
    deo!("main is done doing stuff.");
    dex!("goodbye from main");
}

fn func1(_var: usize) {
    defn!("({:?})", _var);
    defo!("doing even more stuff...");
    defx!();
}
```

this should print to *stderr*

```text
$ cargo run
→hello from main
 main will be doing stuff...
    →func1: (3)
     func1: doing even more stuff...
    ←func1: 
 main is done doing stuff.
←goodbye from main
```

If built with `--release` then the `de` statements are not compiled and nothing
would be printed.

<br/>

An example using a variety of the available ***p***rintln macros. These compile
into debug *and release* builds and print to *stdout*.

```rust
extern crate si_trace_print;
use si_trace_print::{pf1n, pf2n, pfn, pn, po, px};

fn main() {
    pn!("hello from main");
    pfn!("hello again from main");
    pf1n!("hello again from main!");
    pf2n!("HELLO AGAIN FROM MAIN!!!");
    po!("main will be doing stuff...");
    mod1::mod2::func1(3);
    po!("main is done doing stuff...");
    px!("goodbye from main");
}

mod mod1 {
    pub mod mod2 {
        use si_trace_print::{
            pf1n, pf1o, pf1x, pf1ñ, pf2n, pf2o, pf2x, pf2ñ, pfn, pfo, pfx, pfñ, pñ,
        };
        pub fn func1(var: usize) {
            pf1n!("({:?})", var);
            pf1o!("func1 calling func2...");
            func2(var + 1);
            pf1x!("({:?})", var);
        }
        fn func2(var: usize) {
            pf2n!("({:?})", var);
            pf2o!("calling func3...");
            func3();
            pf2x!("({:?})", var);
        }
        fn func3() {
            pfn!();
            func4();
            pfo!("almost complete...");
            pfx!();
        }
        fn func4() {
            pñ!("func4 is a short function.");
            pfñ!("func4 is a short function.");
            pf1ñ!("func4 is a short function.");
            pf2ñ!("func4 is a short function.");
        }
    }
}
```

should print to *stdout*

```text
→hello from main
→main: hello again from main
→main: hello again from main!
→main: HELLO AGAIN FROM MAIN!!!
 main will be doing stuff...
    →mod2::func1: (3)
     mod2::func1: func1 calling func2...
        →mod1::mod2::func2: (4)
         mod1::mod2::func2: calling func3...
            →func3:
                ↔func4 is a short function.
                ↔func4: func4 is a short function.
                ↔mod2::func4: func4 is a short function.
                ↔mod1::mod2::func4: func4 is a short function.
             func3: almost complete...
            ←func3:
        ←mod1::mod2::func2: (4)
    ←mod2::func1: (3)
 main is done doing stuff...
←goodbye from main
```

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

Explicitly call `stack_offset_set` near the beginning of the thread.

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
The indentation amount to pass to `stack_offset_set` can be somewhat
unpredictable.
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
use the ***d***ebug version of provided macros.

### Release builds

The calculation of function depth depends on stack frames counted by
[`backtrace::trace`]. In `--release` builds or under other optimization
profiles, some functions may be optimized inline.
The count of stack frames may not change among function calls.
This means the printed indentation will not reflect function call depth.
This can be forcibly avoided by adding attribute `#[inline(never)]` to such
functions, though [even that is not guaranteed to work].

> Inline attributes do not guarantee that a function is inlined or not inlined,
> but in practice, `#[inline(always)]` will cause inlining in all but the most
> exceptional cases.

### Code Clutter

This simple tracing helper requires explicit statements that some may find
too messy.

### Hogs *stdout* Or *stderr*

If your rust code requires sending messages on both stdout and stderr then
`si_trace_print` will interfere.

## Other tracing crates

Here are some other tracing crates with different features.

- [`trace`] a procedural macro that acts like function wrapper.
- [`tracing`] a heavy-dute framework for in-depth program analysis
- [`rftrace`] uses compiler-provided function tracing via compiler feature
  `mcount`.
  The crates page has it's own section *Alternative Tracers*.
- [`uftrace`] originally a C/C++ tracer that now supports rust. Requires
  specific installed libraries and compiler configuration.
- [`gsingh93/trace`] a macro-based trace library, similar to `si_trace_print`

<!-- links -->

[`backtrace::trace`]: https://docs.rs/backtrace/0.3.66/backtrace/fn.trace.html
[even that is not guaranteed to work]: https://nnethercote.github.io/perf-book/inlining.html
[`trace`]: https://crates.io/crates/trace
[`tracing`]: https://crates.io/crates/tracing
[`rftrace`]: https://crates.io/crates/rftrace
[`uftrace`]: https://github.com/namhyung/uftrace/
[`gsingh93/trace`]: https://github.com/gsingh93/trace

---

<a href="https://stackexchange.com/users/216253/"><img src="https://stackexchange.com/users/flair/216253.png" width="208" height="58" alt="profile for @JamesThomasMoon on Stack Exchange, a network of free, community-driven Q&amp;A sites" title="profile for @JamesThomasMoon on Stack Exchange, a network of free, community-driven Q&amp;A sites" /></a>
