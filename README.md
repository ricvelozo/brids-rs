# brids

Parse and generate random CPF and CNPJ, Brazil's ID numbers.

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
brids = "0.5"
```

## Features

All dependencies are optional and _disabled by default_:

* [`rand`] - enable to generate random numbers
* [`serde`] - enable to (de)serialize numbers

[`rand`]: https://crates.io/crates/rand
[`serde`]: https://crates.io/crates/serde

### `no_std` mode

To enable `no_std` mode, just disable the default features:

```toml
[dependencies]
brids = { version = "0.5", default-features = false }
```

## Examples

Parse and format:

```rust
use brids::Cpf;

fn main() {
    let mut buf = String::new();

    println!("Enter a CPF number:");

    while let Ok(2..) = std::io::stdin().read_line(&mut buf) {
        match buf.trim().parse::<Cpf>() {
            Ok(cpf) => println!("{cpf} is a valid number."),
            Err(err) => eprintln!("Error: {err}"),
        }
        buf.clear();
    }
}
```

Generate random CNPJ and CPF numbers:

```rust
use brids::{Cnpj, Cpf};

fn main() {
    println!("Random CNPJ number: {}", Cnpj::generate());
    println!("Random CPF number: {}", Cpf::generate());
}
```

## License

`brids` is licensed under either of the following, at your option:

*   Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
    https://www.apache.org/licenses/LICENSE-2.0)
*   MIT License ([LICENSE-MIT](LICENSE-MIT) or
    https://opensource.org/licenses/MIT)
