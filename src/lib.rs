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
//! let maybe_valid = "123.456.789-09".parse::<Cpf>();
//! assert!(maybe_valid.is_ok()); // Checks validity
//!
//! let old_format = "123.456.789/09".parse::<Cpf>();
//! assert!(old_format.is_ok()); // Accepts the old format too
//!
//! let unformatted = "12345678909".parse::<Cpf>().expect("invalid CPF");
//! let formatted = unformatted.to_string(); // Formats
//! println!("CPF: {unformatted}"); // Formats too
//! ```
//!
//! Generate random CNPJ and CPF numbers (you must enable the [`rand` feature](#features)):
//!
//! ```rust, ignore
//! use brids::{Cnpj, Cpf};
//!
//! println!("Random CNPJ number: {}", Cnpj::generate());
//! println!("Random CPF number: {}", Cpf::generate());
//! ```
//!
//! If you are using the `no_std` mode, the `::generate()` methods are unavailable; instantiate the
//! generator directly instead:
//!
//! ```rust, ignore
//! use brids::{Cnpj, Cpf};
//! use rand::{Rng, SeedableRng, rngs::StdRng};
//!
//! let mut rng = StdRng::seed_from_u64(123); // Available in `no_std` mode
//! println!("Random CNPJ number: {}", rng.random::<Cnpj>());
//! println!("Random CPF number: {}", rng.random::<Cpf>());
//! ```
//!
//! Serialize and deserialize (you must enable the [`serde` feature](#features)):
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
//! let company1 = Company {
//!     name: "Banco do Brasil S/A",
//!     cnpj: "00.000.000/0001-91".parse().expect("invalid CNPJ"),
//! };
//!
//! // Serializes the struct into JSON
//! let json = serde_json::to_string(&company1).expect("failed to serialize");
//! println!("{json}");
//!
//! // Deserializes the struct back
//! let company2: Company = serde_json::from_str(&json).expect("failed to deserialize");
//! assert_eq!(company1, company2);
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

mod cnpj;
mod cpf;

pub use cnpj::*;
pub use cpf::*;
