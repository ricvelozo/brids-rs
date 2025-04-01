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
use brids::{Cnpj, Cpf};

let maybe_valid = "123.456.789-09".parse::<Cpf>();
assert!(maybe_valid.is_ok()); // Checks validity

let old_format = "123.456.789/09".parse::<Cpf>();
assert!(old_format.is_ok()); // Accepts the old format too

let unformatted = "12345678909".parse::<Cpf>().expect("invalid CPF");
let formatted = unformatted.to_string(); // Formats
println!("CPF: {unformatted}"); // Formats too

// Generate random CNPJ and CPF numbers
println!("Random CNPJ number: {}", Cnpj::generate());
println!("Random CPF number: {}", Cpf::generate());
```

## License

`brids` is licensed under either of the following, at your option:

*   Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
    https://www.apache.org/licenses/LICENSE-2.0)
*   MIT License ([LICENSE-MIT](LICENSE-MIT) or
    https://opensource.org/licenses/MIT)
