// serde.rs
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

use brids::Cnpj;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Company<'a> {
    name: &'a str,
    cnpj: Cnpj,
}

fn main() {
    let company1 = Company {
        name: "Banco do Brasil S/A",
        cnpj: "00.000.000/0001-91".parse().expect("Invalid CNPJ"),
    };

    // Serializes the struct into JSON
    let json = serde_json::to_string(&company1).expect("Failed to serialize");
    println!("{json}");

    // Deserializes the struct back
    let company2: Company = serde_json::from_str(&json).expect("Failed to deserialize");
    assert_eq!(company1, company2);
}
