# si_trace_print<!-- omit in TOC -->

A rust library to print messages indented to stack depth optionally preceded by the function name.

Useful for trace printing function flows.

[![Build status](https://github.com/jtmoon79/si_trace_print/actions/workflows/rust.yml/badge.svg)](https://github.com/jtmoon79/si_trace_print/actions?query=workflow:build)
[![Docs](https://docs.rs/si_trace_print/badge.svg)](https://docs.rs/si_trace_print)
[![codecov](https://codecov.io/gh/jtmoon79/si_trace_print/branch/main/graph/badge.svg?token=Q2OXTL7U02)](https://codecov.io/gh/jtmoon79/si_trace_print)
[![Crates.io](https://img.shields.io/crates/v/si_trace_print.svg)](https://crates.io/crates/si_trace_print)

---

<!-- TOC generated by Markdown All In One -->
- [Use](#use)
  - [Trace-printing example](#trace-printing-example)
  - [Manually setting the indentation](#manually-setting-the-indentation)
- [Shortcomings](#shortcomings)
  - [Release builds](#release-builds)
  - [Slow](#slow)

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
    my_func1(3);
    po!("main is done doing stuff...");
    p!("macro {:?} is just a call to {:?} using fewer characters.", "p!", "println!");
    px!("goodbye from main");
}

fn my_func1(var: usize) {
    pfn!("({:?})", var);
    my_func2(var + 1);
    pfx!("({:?})", var);
}

fn my_func2(var: usize) {
    pfn!("({:?})", var);
    pfo!("calling func3...");
    my_func3();
    pfx!("({:?})", var);
}

fn my_func3() {
    pfn!();
    my_func4();
    pfo!("...");
    pfx!();
}

fn my_func4() {
    pfñ!("func4 is a short function.");
}
```

should print

```text
→hello from main
 main will be doing stuff...
    →my_func1: (3)
        →my_func2: (4)
This "p!" is just a call to "println!" but fewer characters.
         my_func2: calling func3...
            →my_func3:
                ↔my_func4: func4 is a short function.
             my_func3: ...
            ←my_func3:
        ←my_func2: (4)
    ←my_func1: (3)
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
    my_func1(3);
    deo!("main is done doing stuff...");
    dex!("goodbye from main");
}

fn my_func1(_var: usize) {
    defn!("({:?})", _var);
    defx!("({:?})", _var);
}
```

This printed

```text
$ cargo run
→hello from main
 main will be doing stuff...
    →my_func1: (3)
    ←my_func1: (3)
 main is done doing stuff...
←goodbye from main
```

But with `--release` the statements are not compiled so this prints

<!-- markdownlint-disable commands-show-output -->
```text
$ cargo run --release
```
<!-- markdownlint-enable commands-show-output -->

(no program output is printed)

### Manually setting the indentation

If the first use of this library is several functions into a program then
later printing may be lose indentation.

```rust
use si_trace_print::{
    pfo, pfn, pfx, pfñ,
};

fn main() {
    my_func1(3);
    pfx!("goodbye from main (this is not indented!)");
}

fn my_func1(var: usize) {
    my_func2(var);
    pfñ!("({:?}) (this is not indented!)", var);
}

fn my_func2(var: usize) {
    // this is the first call to a si_trace_print function
    // the "original" stack offset will be set from here
    pfn!("({:?})", var);
    pfo!("stack_depth {:?}, stack_offset {:?}", stack_depth(), stack_offset());
    pfx!("({:?})", var);
}
```

prints poorly indented output

```text
→my_func2: (3)
 my_func2: stack_depth 15, stack_offset 0
←my_func2: (3)
↔my_func1: (3) (this is not indented!)
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
    my_func1(3);
    pfx!("goodbye from main");
}

fn my_func1(var: usize) {
    my_func2(var);
    pfñ!("stack_depth {:?}, stack_offset {:?}", stack_depth(), stack_offset());
}

fn my_func2(var: usize) {
    pfn!("({:?})", var);
    pfo!("stack_depth {:?}, stack_offset {:?}", stack_depth(), stack_offset());
    pfx!("({:?})", var);
}
```

this printed

```text
            →my_func2: (3)
             my_func2: stack_depth 15, stack_offset 2
            ←my_func2: (3)
        ↔my_func1: stack_depth 14, stack_offset 1
    ←main: goodbye from main
```

The indentation is improved but slightly too much.
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
        →my_func2: (3)
         my_func2: stack_depth 15, stack_offset 1
        ←my_func2: (3)
    ↔my_func1: stack_depth 14, stack_offset 0
←main: goodbye from main
```

## Shortcomings

### Release builds

The calculation of function depth depends on stack frames counted by
[`backtrace::trace`]. In `--release` builds or under other optimization profiles, some functions may be optimized inline.
The count of stack frames may not change among function calls.
This means the printed indentation will not reflect function call depth.
This can be forcibly avoided by adding attribute `#[inline(never)]` to such
functions.

### Slow

This trace function may signifcantly slow a program. It is recommended to
use the **d**ebug version of provided macros.

<!-- links -->

[`backtrace::trace`]: https://docs.rs/backtrace/0.3.66/backtrace/fn.trace.html
