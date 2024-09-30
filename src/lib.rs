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

//! Parse and generate random CPF and CNPJ, Brazil's ID numbers.
//!
//! # Usage
//!
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! brids = "0.5"
//! ```
//!
//! # Features
//!
//! All dependencies are optional and _disabled by default_:
//!
//! * [`rand`] - enable to generate random numbers
//! * [`serde`] - enable to (de)serialize numbers
//!
//! [`rand`]: https://crates.io/crates/rand
//! [`serde`]: https://crates.io/crates/serde
//!
//! ## `no_std` mode
//!
//! To enable `no_std` mode, just disable the default features:
//!
//! ```toml
//! [dependencies]
//! brids = { version = "0.5", default-features = false }
//! ```
//!
//! # Examples
//!
//! Parse and format:
//!
//! ```rust
//! use brids::Cpf;
//!
//! fn main() {
//!     let mut buf = String::new();
//!
//!     println!("Enter a CPF number:");
//!
//!     while let Ok(2..) = std::io::stdin().read_line(&mut buf) {
//!         match buf.trim().parse::<Cpf>() {
//!             Ok(cpf) => println!("{cpf} is a valid number."),
//!             Err(err) => eprintln!("Error: {err}"),
//!         }
//!         buf.clear();
//!     }
//! }
//! ```
//!
//! Generate random CNPJ and CPF numbers (you must enable the [`rand` feature](#dependencies)):
//!
//! ```rust, ignore
//! use brids::{Cnpj, Cpf};
//!
//! fn main() {
//!     println!("Random CNPJ number: {}", Cnpj::generate());
//!     println!("Random CPF number: {}", Cpf::generate());
//! }
//! ```
//!
//! Using a different generator:
//!
//! ```rust, ignore
//! use brids::{Cnpj, Cpf};
//! use rand::{rngs::StdRng, Rng, SeedableRng};
//!
//! fn main() {
//!     let mut rng = StdRng::seed_from_u64(123);
//!     println!("Random CNPJ number: {}", rng.gen::<Cnpj>());
//!     println!("Random CPF number: {}", rng.gen::<Cpf>());
//! }
//! ```
//!
//! Serialize and deserialize (you must enable the [`serde` feature](#dependencies)):
//!
//! ```rust, ignore
//! use brids::Cnpj;
//! use serde::{Deserialize, Serialize};
//! use serde_json;
//!
//! #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
//! struct Company<'a> {
//!     name: &'a str,
//!     cnpj: Cnpj,
//! }
//!
//! fn main() {
//!     let company1 = Company {
//!         name: "ACME",
//!         cnpj: Cnpj::generate(),
//!     };
//!
//!     // Serializes the struct into JSON
//!     let json = serde_json::to_string(&company1).expect("Failed to serialize");
//!     println!("{json}");
//!
//!     // Deserializes the struct back
//!     let company2: Company = serde_json::from_str(&json).expect("Failed to deserialize");
//!     assert_eq!(company1, company2);
//! }
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

mod cnpj;
mod cpf;

pub use cnpj::*;
pub use cpf::*;
