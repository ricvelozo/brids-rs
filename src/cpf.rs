// cpf.rs
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

#[cfg(all(feature = "serde", not(feature = "std")))]
use crate::alloc::string::ToString;
use core::{convert::TryFrom, fmt, str::FromStr};
use failure::Fail;
#[cfg(feature = "rand")]
use rand::{
    distributions::{Distribution, Standard},
    thread_rng, Rng,
};
#[cfg(feature = "serde")]
use serde::*;

/// An error which can be returned when parsing an CPF number.
#[derive(Fail, Debug, PartialEq, Eq)]
pub enum ParseCpfError {
    #[fail(display = "Empty.")]
    Empty,
    #[fail(display = "Invalid character `{}` at offset {}.", _0, _1)]
    InvalidCharacter(char, usize),
    #[fail(display = "Invalid CPF number.")]
    InvalidNumber,
}

/// A valid CPF number. Parsing recognizes numbers with or without separators (dot, minus,
/// and slash).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cpf([u8; 11]);

impl Cpf {
    /// Parses a byte slice of numbers as an CPF, guessing the missing parts.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// use brids::Cpf;
    ///
    /// match Cpf::from_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 9]) {
    ///     Ok(cpf) => println!("{} is a valid number.", cpf),
    ///     Err(err) => eprintln!("Error: {}", err),
    /// }
    /// ```
    ///
    /// Guess the check digits:
    ///
    /// ```rust
    /// use brids::Cpf;
    ///
    /// match Cpf::from_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 9]) {
    ///     Ok(cpf) => println!("{} is a valid number.", cpf),
    ///     Err(err) => eprintln!("Error: {}", err),
    /// }
    /// ```
    pub fn from_slice(slice: &[u8]) -> Result<Self, ParseCpfError> {
        let mut numbers = [0; 11];
        match slice.len() {
            0 => return Err(ParseCpfError::Empty),
            9 | 11 => (),
            _ => return Err(ParseCpfError::InvalidNumber),
        }

        for (y, x) in numbers.iter_mut().zip(slice.iter()) {
            // 0..=9
            if *x > 9 {
                return Err(ParseCpfError::InvalidNumber);
            }
            *y = *x;
        }

        // Checks for repeated numbers
        let first_number = numbers[0];
        if slice.len() == 11 && numbers.iter().all(|&x| x == first_number) {
            return Err(ParseCpfError::InvalidNumber);
        }

        for i in 0..=1 {
            let check_digit = numbers[9 + i];
            let mut remainder = numbers
                .iter()
                // Includes the first check digit in the second iteration
                .take(9 + i)
                // 10, 9, 8, ... 3, 2; and after: 11, 10, 9, 8, ... 3, 2
                .zip((2..=10 + i).rev())
                .map(|(&x, y)| u32::from(x) * y as u32)
                .sum::<u32>()
                * 10
                % 11;

            if let 10 | 11 = remainder {
                remainder = 0;
            }

            if slice.len() < 11 {
                numbers[9 + i] = remainder as u8; // check digit
            } else if remainder != u32::from(check_digit) {
                return Err(ParseCpfError::InvalidNumber);
            }
        }

        Ok(Cpf(numbers))
    }

    /// Returns a byte slice of the numbers.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brids::Cpf;
    ///
    /// let cpf = Cpf::generate();
    /// let bytes = cpf.as_bytes();
    /// ```
    #[inline]
    pub fn as_bytes(&self) -> &[u8; 11] {
        &self.0
    }

    /// Generates a random number, using [`rand::thread_rng`] (optional dependency enabled by
    /// default). To use a different generator, instantiate the generator directly.
    ///
    /// [`rand::thread_rng`]: https://docs.rs/rand/0.6/rand/fn.thread_rng.html
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brids::Cpf;
    ///
    /// let cpf = Cpf::generate();
    /// ```
    #[cfg(feature = "rand")]
    #[inline]
    pub fn generate() -> Self {
        thread_rng().gen()
    }
}

impl AsRef<[u8]> for Cpf {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl TryFrom<&[u8]> for Cpf {
    type Error = ParseCpfError;

    #[inline]
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::from_slice(value)
    }
}

impl TryFrom<&[u8; 11]> for Cpf {
    type Error = ParseCpfError;

    #[inline]
    fn try_from(value: &[u8; 11]) -> Result<Self, Self::Error> {
        Self::from_slice(value)
    }
}

impl fmt::Debug for Cpf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cpf(\"{}\")", self)
    }
}

impl fmt::Display for Cpf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, number) in self.0.iter().enumerate() {
            if i % 9 == 0 && i != 0 {
                write!(f, "-")?;
            } else if i % 3 == 0 && i != 0 {
                write!(f, ".")?;
            }
            write!(f, "{}", number)?;
        }
        Ok(())
    }
}

impl FromStr for Cpf {
    type Err = ParseCpfError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers = [0; 11];

        if s.is_empty() {
            return Err(ParseCpfError::Empty);
        }

        // Checks for invalid symbols and converts numbers to integers
        let mut i = 0;
        let mut has_dot = false;
        for (offset, c) in s.chars().enumerate() {
            match (c, offset) {
                ('0'..='9', _) => {
                    if i < 11 {
                        numbers[i] = c.to_digit(10).unwrap() as u8;
                        i += 1;
                    } else {
                        return Err(ParseCpfError::InvalidNumber);
                    }
                }
                ('.', 3 | 7) => has_dot = true,
                ('-' | '/', 11) if has_dot => continue,
                ('-' | '/', 9) if !has_dot => continue,
                _ => return Err(ParseCpfError::InvalidCharacter(c, offset)),
            }
        }

        // Checks for repeated numbers
        let first_number = numbers[0];
        if numbers.iter().all(|&x| x == first_number) {
            return Err(ParseCpfError::InvalidNumber);
        }

        for i in 0..=1 {
            let check_digit = numbers[9 + i];
            let mut remainder = numbers
                .iter()
                // Includes the first check digit in the second iteration
                .take(9 + i)
                // 10, 9, 8, ... 3, 2; and after: 11, 10, 9, 8, ... 3, 2
                .zip((2..=10 + i).rev())
                .map(|(&x, y)| u32::from(x) * y as u32)
                .sum::<u32>()
                * 10
                % 11;

            if let 10 | 11 = remainder {
                remainder = 0;
            }

            if remainder != u32::from(check_digit) {
                return Err(ParseCpfError::InvalidNumber);
            }
        }

        Ok(Cpf(numbers))
    }
}

#[cfg(feature = "rand")]
impl Distribution<Cpf> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Cpf {
        let mut numbers = [0; 11];
        for number in numbers.iter_mut().take(9) {
            *number = rng.gen_range(0, 9);
        }

        for i in 0..=1 {
            let mut remainder = numbers
                .iter()
                // Includes the first check digit in the second iteration
                .take(9 + i)
                // 10, 9, 8, ... 3, 2; and after: 11, 10, 9, 8, ... 3, 2
                .zip((2..=10 + i).rev())
                .map(|(&x, y)| u32::from(x) * y as u32)
                .sum::<u32>()
                * 10
                % 11;

            if let 10 | 11 = remainder {
                remainder = 0;
            }

            numbers[9 + i] = remainder as u8; // check digit
        }

        Cpf(numbers)
    }
}

#[cfg(feature = "serde")]
impl Serialize for Cpf {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() {
            serializer.serialize_str(&self.to_string())
        } else {
            serializer.serialize_bytes(&self.as_ref())
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Cpf {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        if deserializer.is_human_readable() {
            struct CpfStringVisitor;

            impl<'vi> de::Visitor<'vi> for CpfStringVisitor {
                type Value = Cpf;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    write!(formatter, "a CPF string")
                }

                fn visit_str<E: de::Error>(self, value: &str) -> Result<Cpf, E> {
                    value.parse::<Cpf>().map_err(E::custom)
                }

                fn visit_bytes<E: de::Error>(self, value: &[u8]) -> Result<Cpf, E> {
                    Cpf::try_from(value).map_err(E::custom)
                }
            }

            deserializer.deserialize_str(CpfStringVisitor)
        } else {
            struct CpfBytesVisitor;

            impl<'vi> de::Visitor<'vi> for CpfBytesVisitor {
                type Value = Cpf;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    write!(formatter, "bytes")
                }

                fn visit_bytes<E: de::Error>(self, value: &[u8]) -> Result<Cpf, E> {
                    Cpf::try_from(value).map_err(E::custom)
                }
            }

            deserializer.deserialize_bytes(CpfBytesVisitor)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(not(feature = "std"))]
    use crate::alloc::string::ToString;

    #[test]
    fn from_slice() {
        let a = Cpf([1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 9]);
        let b: [u8; 11] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 9];
        let c: [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

        assert_eq!(a, Cpf::from_slice(&b).unwrap());
        assert_eq!(a, Cpf::from_slice(&c).unwrap());
    }

    #[test]
    fn as_bytes() {
        let a: [u8; 11] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 9];
        let b = Cpf([1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 9]);

        assert_eq!(&a, b.as_bytes());
    }

    #[cfg(feature = "rand")]
    #[test]
    fn generate() {
        let a = Cpf::generate();
        let b = a.to_string().parse::<Cpf>().unwrap();

        assert_eq!(a, b);
    }

    #[test]
    fn as_ref() {
        fn test_trait<T: AsRef<[u8]>>(b: T) {
            let a: [u8; 11] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 9];
            assert_eq!(&a, b.as_ref());
        }

        let b = Cpf([1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 9]);

        test_trait(b);
    }

    #[test]
    fn try_from() {
        let a: [u8; 11] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 9];
        let b = Cpf([1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 9]);

        assert_eq!(Cpf::try_from(&a).unwrap(), b);
    }

    #[test]
    fn cmp() {
        let a = Cpf([1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 9]);
        let b = Cpf([1, 2, 3, 4, 5, 6, 7, 9, 0, 3, 4]);

        assert!(a < b);
    }

    #[test]
    fn debug() {
        let a = r#"Cpf("123.456.789-09")"#;
        let b = Cpf([1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 9]);

        assert_eq!(a, format!("{:?}", b));
    }

    #[test]
    fn display() {
        let a = "123.456.789-09";
        let b = Cpf([1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 9]);

        assert_eq!(a, format!("{}", b));
    }

    #[test]
    fn from_str() {
        let a = "123.456.789-09".parse::<Cpf>().unwrap();
        let b = "123456789/09".parse::<Cpf>().unwrap();
        let c = "12345678909".parse::<Cpf>().unwrap();

        assert_eq!(a, b);
        assert_eq!(a, c);
        assert_eq!("".parse::<Cpf>(), Err(ParseCpfError::Empty));
        assert_eq!(
            "123-456-789-09".parse::<Cpf>(),
            Err(ParseCpfError::InvalidCharacter('-', 3))
        );
        assert_eq!(
            "123.456.789-10".parse::<Cpf>(),
            Err(ParseCpfError::InvalidNumber)
        );
        assert_eq!(
            "123.456.789-009".parse::<Cpf>(),
            Err(ParseCpfError::InvalidNumber)
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serialize_readable() {
        use serde_test::Configure;

        let cpf_str = "123.456.789-09";
        let cpf = Cpf::from_str(cpf_str).unwrap();
        serde_test::assert_tokens(&cpf.readable(), &[serde_test::Token::Str(cpf_str)]);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serialize_compact() {
        use serde_test::Configure;

        let cpf_bytes = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 9];
        let cpf = Cpf::try_from(cpf_bytes).unwrap();
        serde_test::assert_tokens(&cpf.compact(), &[serde_test::Token::Bytes(cpf_bytes)]);
    }
}
