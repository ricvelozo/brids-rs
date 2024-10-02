# Releases

## Version 0.5.1 (2024-10-02)

* Refactor and improve documentation

## Version 0.5.0 (2024-09-29)

* Update to Rust 2021 edition and add minimum supported Rust version (1.81)
* Add `no_std` support
* Remove `failure` dependency
* Remove "compact" (de)serialization
* Stricter parsing
* Refactor and some optimizations

## Version 0.4.0 (2018-12-21)

* Update to Rust 2018 edition
* Update dependencies
* Configure GitLab CI
* Add methods to parse from byte slices
* Add method to get the CNPJ entity branch
* Implement `AsRef<[u8]>`, `PartialOrd` and `Ord`
* Turn `Cnpj` and `Cpf` structs into newtypes
* Add support to serialization with `serde`
* Add more examples and improve documentation

## Version 0.3.1 (2018-06-19)

* Fix parsing when have too much numbers and add test
* Migrate to GitLab

## Version 0.3.0 (2018-04-08)

* Add `failure` errors, and use `matches!()` in tests
* Use `pub type` instead of `pub use` for Icn
* Export `ParseCnpjError` and `ParseCpfError`
* Add `fmt::Debug` impl
* Force generate CNPJ number for headquarters

## Version 0.2.1 (2018-03-29)

* Update examples and version number in README

## Version 0.2.0 (2018-03-29)

* Add CNPJ support
* Configure Travis CI
* Update examples, documentation and tests

## Version 0.1.0 (2018-03-28)

* Add ICN/CPF support
