// Copyright 2018 Ricardo Silva Veloso
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate brids;

use brids::Cpf;
use std::io::{stdin, stdout, Write};

fn main() {
    print!("Input a CPF/ICN number: ");
    stdout().flush().ok();

    let mut input = String::with_capacity(14);
    stdin().read_line(&mut input).ok();

    match input.trim().parse::<Cpf>() {
        Ok(cpf) => println!("{} is a valid number.", cpf),
        Err(_) => println!("Invalid number."),
    }
}
