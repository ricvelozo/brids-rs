# brids

[![Build status]][travis-ci.org] [![Latest version]][crates.io] [![Documentation]][docs.rs]

[Build Status]: https://api.travis-ci.org/ricvelozo/brids-rs.svg?branch=master
[travis-ci.org]: https://travis-ci.org/ricvelozo/brids-rs
[Latest Version]: https://img.shields.io/crates/v/brids.svg
[crates.io]: https://crates.io/crates/brids
[Documentation]: https://docs.rs/brids/badge.svg
[docs.rs]: https://docs.rs/brids

Parse and generate random CPF/ICN and CNPJ, Brazil's ID numbers.

## Usage

First, add the following to your `Cargo.toml`:

```toml
[dependencies]
brids = "0.1"
```

Next, add this to your crate root:

```rust
extern crate brids;
```

## Examples

Parse and format:

```rust
extern crate brids;

use brids::Cpf;
use std::io::{stdin, stdout, Write};

fn main() {
    print!("Input a CPF number: ");
    stdout().flush().ok();

    let mut input = String::with_capacity(14);
    stdin().read_line(&mut input).ok();

    match input.trim().parse::<Cpf>() {
        Ok(cpf) => println!("{} is a valid CPF number.", cpf),
        Err(_) => println!("Invalid number."),
    }
}
```

Generate random CNPJ and CPF/ICN numbers:

```rust
extern crate brids;

use brids::{Cnpj, Cpf};

fn main() {
    println!("Random CNPJ number: {}", Cnpj::generate());
    println!("Random CPF number: {}", Cpf::generate());
}
```

## License

`brids` is licensed under either of the following, at your option:

*   Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
    https://www.apache.org/licenses/LICENSE-2.0)
*   MIT License ([LICENSE-MIT](LICENSE-MIT) or
    https://opensource.org/licenses/MIT)
