# `var!`

[![Build Status](https://travis-ci.org/huonw/var.png)](https://travis-ci.org/huonw/var)

A Rust macro for declaring and initialising multiple mutable variables
in a single statement.

```rust
#[macro_use] extern crate var;

var! {
    a = 1,
    b: &str = "foo",
    c = 3.0,
}

a += 1;
b = "bar";
c *= 7.0;
```

[Documentation](http://huonw.github.io/var/var), [crates.io](https://crates.io/crates/var)
