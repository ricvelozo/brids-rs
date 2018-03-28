// Copyright 2018 Ricardo Silva Veloso
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Parse and generate random ICN/CPF and CNPJ (soon), Brazil's ID numbers.
//!
//! # Usage
//!
//! First, add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! brids = "0.1"
//! ```
//!
//! Next, add this to your crate root:
//!
//! ```rust
//! extern crate brids;
//! ```
//!
//! # Dependencies
//!
//! The [rand] crate is an optional dependency enabled by default. To disable, use like this:
//!
//! [rand]: https://crates.io/crates/rand
//!
//! ```toml
//! [dependencies]
//! brids = { version = "0.1", default-features = false }
//! ```
//!
//! # Examples
//!
//! Parse and format:
//!
//! ```rust
//! extern crate brids;
//!
//! use brids::Cpf;
//! use std::io::{stdin, stdout, Write};
//!
//! fn main() {
//!     print!("Input a CPF number: ");
//!     stdout().flush().ok();
//!
//!     let mut input = String::with_capacity(14);
//!     stdin().read_line(&mut input).ok();
//!
//!     match input.trim().parse::<Cpf>() {
//!         Ok(cpf) => println!("{} is a valid CPF number.", cpf),
//!         Err(_) => println!("Invalid number."),
//!     }
//! }
//! ```
//!
//! Generate a random CPF number:
//!
//! ```rust
//! extern crate brids;
//!
//! use brids::Cpf;
//!
//! fn main() {
//!     println!("Random CPF number: {}", Cpf::generate());
//! }
//! ```
//!
//! Using a different generator:
//!
//! ```rust
//! extern crate brids;
//! extern crate rand;
//!
//! use brids::Cpf;
//! use rand::{ChaChaRng, Rng};
//!
//! fn main() {
//!     let mut rng = ChaChaRng::new_unseeded();
//!     println!("Random CPF number: {}", rng.gen::<Cpf>());
//! }
//! ```

mod cpf;

pub use cpf::Cpf;
