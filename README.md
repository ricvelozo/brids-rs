# brids

Parse and generate random CPF and CNPJ, Brazil's ID numbers.

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
brids = "0.4"
```

## Examples

Parse and format:

```rust
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
