# brids

Parse and generate random CPF/ICN and CNPJ, Brazil's ID numbers.

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
use std::io::{stdin, stdout, Write};

fn main() {
    print!("Input a CPF/ICN number: ");
    stdout().flush().ok();

    let mut input = String::with_capacity(14);
    stdin().read_line(&mut input).ok();

    match input.trim().parse::<Cpf>() {
        Ok(cpf) => println!("{} is a valid number.", cpf),
        Err(err) => println!("Error: {}", err),
    }
}
```

Generate random CNPJ and CPF/ICN numbers:

```rust
use brids::{Cnpj, Cpf};

fn main() {
    println!("Random CNPJ number: {}", Cnpj::generate());
    println!("Random CPF/ICN number: {}", Cpf::generate());
}
```

## License

`brids` is licensed under either of the following, at your option:

*   Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
    https://www.apache.org/licenses/LICENSE-2.0)
*   MIT License ([LICENSE-MIT](LICENSE-MIT) or
    https://opensource.org/licenses/MIT)
