// lib.rs
//
// Copyright 2018 Ricardo Silva Veloso <ricvelozo@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT License
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// SPDX-License-Identifier: (MIT OR Apache-2.0)

//! Parse and generate random CPF/ICN and CNPJ, Brazil's ID numbers.
//!
//! # Usage
//!
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! brids = "0.3"
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
//! brids = { version = "0.3", default-features = false }
//! ```
//!
//! # Examples
//!
//! Parse and format:
//!
//! ```rust
//! use brids::Cpf;
//! use std::io::{stdin, stdout, Write};
//!
//! fn main() {
//!     print!("Input a CPF/ICN number: ");
//!     stdout().flush().ok();
//!
//!     let mut input = String::with_capacity(14);
//!     stdin().read_line(&mut input).ok();
//!
//!     match input.trim().parse::<Cpf>() {
//!         Ok(cpf) => println!("{} is a valid number.", cpf),
//!         Err(err) => println!("Error: {}", err),
//!     }
//! }
//! ```
//!
//! Generate random CNPJ and CPF/ICN numbers:
//!
//! ```rust
//! use brids::{Cnpj, Cpf};
//!
//! fn main() {
//!     println!("Random CNPJ number: {}", Cnpj::generate());
//!     println!("Random CPF/ICN number: {}", Cpf::generate());
//! }
//! ```
//!
//! Using a different generator:
//!
//! ```rust
//! use brids::{Cnpj, Cpf};
//! use rand::{ChaChaRng, Rng};
//!
//! fn main() {
//!     let mut rng = ChaChaRng::new_unseeded();
//!     println!("Random CNPJ number: {}", rng.gen::<Cnpj>());
//!     println!("Random CPF/ICN number: {}", rng.gen::<Cpf>());
//! }
//! ```

mod cnpj;
mod cpf;

pub use crate::cpf::*;
pub type Icn = Cpf;
pub use crate::cnpj::*;
