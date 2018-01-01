# Code Examples for _Programming Rust_

This repository contains complete code for the larger example programs
from the book “Programming Rust”, by Jim Blandy and Jason Orendorff.

Each subdirectory is a distinct Rust project, with its own Cargo.toml file. You
should be able to enter each directory and use `cargo build` and `cargo test`.
For those projects that define programs, `cargo run` should run them.

The example code in this directory and its subdirectories is licensed under the
terms of the MIT license. See [LICENSE-MIT](LICENSE-MIT) for details.

## Chapter 2: A Tour of Rust

- The `gcd` directory holds the command-line program for computing the greatest
  common denominator of a list of numbers.

- The `iron-gcd` directory holds the code for the simple web service,
  implemented using the [`iron`] framework, that computes greatest common
  denominators.

- The Mandelbrot plotting program has its own repository, at
  `https://github.com/ProgrammingRust/mandelbrot`. This repository contains
  several branches, each showing a different implementation strategy. The
  `single-threaded` branch holds the code for the single-threaded version, and
  the `bands` branch holds the multi-threaded version. Chapter 19,
  “Concurrency”, shows several other approaches, which appear on other branches;
  see the repository's [README.md][mandel-readme] file for details.

[`iron`]: https://crates.io/crates/iron

## Chapter 8: Crates and Modules

- We did not actually write a fern simulator. Please accept our sincere apology
  for this feckless deception. But the skeleton of modules and definitions we
  show in the book is in the `fern_sim` subdirectory.

## Chapter 9: Structs

- The `queue` directory holds a library that defines the `Queue` type,
  representing a queue of `char` values.

- The `generic-queue` directory holds code for generic `Queue` type.

## Chapter 10: Enums and Patterns

- The `binary-tree` directory holds the source code for the `BinaryTree` type
  that appears in the “Generic Enums” and “Populating a Binary Tree” sections.

## Chapter 12: Operator Overloading

- The `complex` directory holds the `Complex` type used as a running example
  throughout the chapter.

- The `interval` directory holds the `Interval` type for which the book
  implements the `std::cmp::PartialOrd` trait.

## Chapter 14: Closures

- The 'basic-router' directory holds the `BasicRouter` type used as an example
  in the “Callbacks” section.

## Chapter 15: Iterators

- The `binary-tree` directory holds the implementation of the `Iterator` trait
  for the `BinaryTree` type originally defined in the “Enums and Patterns”
  chapter.

## Chapter 17: Strings and Text

- The `complex` directory includes the implementation of the `std::fmt::Display`
  formatting trait for a complex number type, shown in the section “Formatting
  Your Own Types”.

## Chapter 18: Input and Output

- The `grep` directory holds the simple grep-like program shown in the section
  “Reading Lines”.

- The `copy` directory holds the program for copying directory trees from the
  section “Reading Directories”, including the additions shown in the next
  section, “Platform-Specific Features”.

- The `echo-server` directory holds the simple network service shown in the
  “Networking” section.

- The `http-get` directory holds the command-line program that uses the
  `reqwest` crate to carry out an HTTP request.

## Chapter 19: Concurrency

- The search engine used as a running example throughout the book has its own
  repository, at `https://github.com/ProgrammingRust/fingertips`.

- The Mandelbrot set plotter discussed in the section “Revisiting the Mandelbrot
  Set” also has its own repository, at `https://github.com/ProgrammingRust/mandelbrot`.
  The repository includes several branches exploring different implementations;
  see the repository's [README.md][mandel-readme] file for details.

[mandel-readme]: https://github.com/ProgrammingRust/mandelbrot/blob/master/README.md

## Chapter 20: Macros

- The `json-macro` directory holds the definition of the `json!` macro built in
  the section “The json! Macro”.

## Chapter 21: Unsafe Code

- The `ascii` directory holds the `Ascii` type used as an example in the
  sections “Unsafe Blocks” and “Unsafe Functions”.

- The `ref-with-flag` directory holds the `RefWithFlag` type from the “Raw
  Pointers” section.

- The `gap-buffer` directory holds the `GapBuffer` type, used in the “Raw
  Pointers” section to illustrate pointer arithmetic and `std::ptr::read` and
  `std::ptr::write`.

- The `libgit2-rs` and `libgit2-rs-safe` directories contain the two versions of
  the program that uses Rust's foreign function interface to call functions from
  the `libgit2` C library. The version in `libgit2-rs` is written as a single
  giant block of unsafe code, whereas the version in `libgit2-rs-safe`
  implements a safe Rust interface to the same functionality, using Rust's type
  system to enforce libgit2's rules for proper use of the library.

  Note that both of these require you to have a copy of `libgit2` present on
  your system. The chapter provides detailed instructions for building the
  correct version, for Linux, macOS, and Microsoft Windows.
