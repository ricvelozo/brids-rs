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

use brids::Cpf;
use std::io;

fn main() {
    println!("Enter a CPF number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim().parse::<Cpf>() {
        Ok(cpf) => println!("{cpf} is a valid number."),
        Err(err) => eprintln!("Error: {err}"),
    }
}
