#![feature(iter_intersperse, box_patterns, is_ascii_octdigit, slice_split_once, assert_matches)]
//!
//! # AvPony
//! > What is AvPony?
//!
//! AvPony is a new UI language that will be used for AvdanOS components.
//!
//! It's an abstraction over Skia, or another graphical backend, with a
//! JSX-like frontend.
//!
//! You can use the language in combination of either:
//! * TypeScript
//! * Rust
//!
//! ## What does this crate do?
//! `avpony-lang` features the definition of the language's grammar,
//! a parser made using [chumsky], and syntax errors with [ariadne].
//!
//! ## Building
//! We are nightly compiler only, so make sure you have the nightly compiler in
//! your Rustup toolchain: `rustup install +nighlty`
//!

pub mod lexical;
pub mod ponyx;
pub mod syntax;
pub mod utils;
