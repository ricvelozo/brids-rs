// seed.rs
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

use brids::{Cnpj, Cpf};
use rand::{SeedableRng, Rng, rngs::StdRng};

fn main() {
    let mut rng = StdRng::seed_from_u64(123);
    println!("Random CNPJ number: {}", rng.gen::<Cnpj>());
    println!("Random CPF number: {}", rng.gen::<Cpf>());
}
