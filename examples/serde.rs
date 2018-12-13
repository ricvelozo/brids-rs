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
use serde_derive::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Company<'a> {
    name: &'a str,
    cnpj: Cnpj,
}

fn main() {
    let company1 = Company {
        name: "ACME",
        cnpj: Cnpj::generate(),
    };

    // Serializes the struct into JSON
    let json = serde_json::to_string(&company1).unwrap();
    println!("{}", json);

    // Deserializes the struct back
    let company2: Company = serde_json::from_str(&json).unwrap();
    assert_eq!(company1, company2);
}
