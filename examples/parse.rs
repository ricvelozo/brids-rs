// parse.rs
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

use std::io;

use brids::Cpf;

fn main() {
    let mut buf = String::new();

    println!("Enter a CPF number:");
    while let Ok(2..) = io::stdin().read_line(&mut buf) {
        match buf.trim().parse::<Cpf>() {
            Ok(cpf) => println!("{cpf} is a valid number."),
            Err(err) => eprintln!("Error: {err}"),
        }
        buf.clear();
    }
}
