//! A macro to declare multiple mutable variables in one statement.
//!
//! `var! { ... }` is a generalised form of `let mut x = ...;`,
//! allowing for several mutable variables to be declared and
//! initialised at once, inspired by the keyword of the same name in
//! languages like Nim and C#.
//!
//! [Available on
//! crates.io](https://crates.io/crates/var). [Source](https://github.com/huonw/var).
//!
//! # Grammar
//!
//! ```not_rust
//! "var! {"
//!     (
//!         identifier (":" type)? "=" expression
//!     )*
//! "}"
//! ```
//!
//! where
//!
//! - `"..."` represents a literal `...`
//! - `(...)` is for grouping
//! - `*` is zero-or-more copies of the previous entity, *comma separated*, with
//!   optional trailing comma
//! - `?` is zero-or-one copies of the pervious entity (i.e. optional)
//!
//! Notably,
//!
//! - `var!` should always be invoked with `{}`s or else one will get
//!   compile errors (invoking a macro with `()` and `[]` mean the its internals
//!   are parsed as an expression, but declaring a variable with `let` is a
//!   statement),
//! - an initialising expression is required, unlike conventional `let`,
//! - pattern matching cannot be performed.
//!
//! # Examples
//!
//! ```rust
//! #[macro_use] extern crate var;
//!
//! fn fibonacci(n: u32) -> u64 {
//!     var! {
//!         a: u64 = 0,
//!         b = 1,
//!     }
//!     for _ in 0..n {
//!         let tmp = a + b;
//!         a = b;
//!         b = tmp;
//!     }
//!     return a
//! }
//!
//! fn main() {
//!     assert_eq!(fibonacci(10), 55);
//! }
//! ```

/// The main macro, see crate docs for details.
#[macro_export]
macro_rules! var {
    ($($vals: tt)*) => {
        __var_internals! {__make_things_work: () = (); $($vals)*,}
    };
}

/// Implementation details. **Do not use this directly.**
#[macro_export]
macro_rules! __var_internals {
    ($($name: ident : $t: ty = $init: expr),*; $(,)*) => {
        let ($(mut $name),* ,): ($($t),* ,) = ($($init),* ,);
    };
    ($($bname: ident : $bt: ty = $binit: expr),*; $name: ident = $init: expr, $($rest: tt)*) => {
        __var_internals!{$($bname: $bt = $binit,)* $name: _ = $init; $($rest)*}
    };
    ($($bname: ident : $bt: ty = $binit: expr),* ; $name: ident : $t: ty = $init: expr, $($rest: tt)*) => {
        __var_internals!{$($bname: $bt = $binit,)* $name: $t = $init; $($rest)*}
    };
}
