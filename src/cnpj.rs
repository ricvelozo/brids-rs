// cnpj.rs
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

/// An error which can be returned when parsing an CNPJ number.
#[derive(Fail, Debug, PartialEq, Eq)]
pub enum ParseCnpjError {
    #[fail(display = "Empty.")]
    Empty,
    #[fail(display = "Invalid character `{}` at offset {}.", _0, _1)]
    InvalidCharacter(char, usize),
    #[fail(display = "Invalid CNPJ number.")]
    InvalidNumber,
}

/// A valid CNPJ number. Parsing recognizes numbers with or without separators (dot, minus,
/// and slash).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cnpj([u8; 14]);

impl Cnpj {
    /// Parses a byte slice of numbers as an CNPJ, guessing the missing parts.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// use brids::Cnpj;
    ///
    /// match Cnpj::from_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 1, 9, 5]) {
    ///     Ok(cnpj) => println!("{} is a valid number.", cnpj),
    ///     Err(err) => eprintln!("Error: {}", err),
    /// }
    /// ```
    ///
    /// Guess the check digits:
    ///
    /// ```rust
    /// use brids::Cnpj;
    ///
    /// match Cnpj::from_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 2, 7]) {
    ///     Ok(cnpj) => println!("{} is a valid number.", cnpj),
    ///     Err(err) => eprintln!("Error: {}", err),
    /// }
    /// ```
    ///
    /// Guess the branch and check digits:
    ///
    /// ```rust
    /// use brids::Cnpj;
    ///
    /// match Cnpj::from_slice(&[1, 2, 3, 4, 5, 6, 7, 8]) {
    ///     Ok(cnpj) => println!("{} is a valid number.", cnpj),
    ///     Err(err) => eprintln!("Error: {}", err),
    /// }
    /// ```
    pub fn from_slice(slice: &[u8]) -> Result<Self, ParseCnpjError> {
        let mut numbers = [0; 14];
        match slice.len() {
            0 => return Err(ParseCnpjError::Empty),
            8 => numbers[11] = 1, // Company headquarters
            12 | 14 => (),
            _ => return Err(ParseCnpjError::InvalidNumber),
        }

        for (y, x) in numbers.iter_mut().zip(slice.iter()) {
            // 0..=9
            if *x > 9 {
                return Err(ParseCnpjError::InvalidNumber);
            }
            *y = *x;
        }

        // Checks for repeated numbers
        let first_number = numbers[0];
        if slice.len() == 14 && numbers.iter().all(|&x| x == first_number) {
            return Err(ParseCnpjError::InvalidNumber);
        }

        for i in 0..=1 {
            let check_digit = numbers[12 + i];
            let mut remainder = numbers
                .iter()
                // Includes the first check digit in the second iteration
                .take(12 + i)
                // 5, 4, 3, 2, 9, 8, 7, ... 3, 2; and after: 6, 5, 4, 3, 2, 9, 8, 7, ... 3, 2
                .zip((2..=9).chain(2..=5 + i).rev())
                .map(|(&x, y)| u32::from(x) * y as u32)
                .sum::<u32>()
                * 10
                % 11;

            if let 10 | 11 = remainder {
                remainder = 0;
            }

            if slice.len() < 14 {
                numbers[12 + i] = remainder as u8; // check digit
            } else if remainder != u32::from(check_digit) {
                return Err(ParseCnpjError::InvalidNumber);
            }
        }

        Ok(Cnpj(numbers))
    }

    /// Returns a byte slice of the numbers.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brids::Cnpj;
    ///
    /// let cnpj = Cnpj::generate();
    /// let bytes = cnpj.as_bytes();
    /// ```
    #[inline]
    pub fn as_bytes(&self) -> &[u8; 14] {
        &self.0
    }

    /// Returns the entity branch.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brids::Cnpj;
    ///
    /// let cnpj = Cnpj::generate();
    /// let branch = cnpj.branch();
    /// ```
    #[inline]
    pub fn branch(&self) -> u16 {
        self.0[8..=11]
            .iter()
            .rev()
            .enumerate()
            .map(|(i, &x)| u16::from(x) * 10u16.pow(i as u32))
            .sum::<u16>()
    }

    /// Generates a random number, using [`rand::thread_rng`] (optional dependency enabled by
    /// default). To use a different generator, instantiate the generator directly.
    ///
    /// [`rand::thread_rng`]: https://docs.rs/rand/0.6/rand/fn.thread_rng.html
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brids::Cnpj;
    ///
    /// let cnpj = Cnpj::generate();
    /// ```
    #[cfg(feature = "rand")]
    #[inline]
    pub fn generate() -> Self {
        thread_rng().gen()
    }
}

impl AsRef<[u8]> for Cnpj {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl TryFrom<&[u8]> for Cnpj {
    type Error = ParseCnpjError;

    #[inline]
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::from_slice(value)
    }
}

impl TryFrom<&[u8; 14]> for Cnpj {
    type Error = ParseCnpjError;

    #[inline]
    fn try_from(value: &[u8; 14]) -> Result<Self, Self::Error> {
        Self::from_slice(value)
    }
}

impl fmt::Debug for Cnpj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cnpj(\"{}\")", self)
    }
}

impl fmt::Display for Cnpj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}.", self.0[0], self.0[1])?;
        for (i, number) in self.0.iter().skip(2).enumerate() {
            if i % 10 == 0 && i != 0 {
                write!(f, "-")?;
            } else if i % 6 == 0 && i != 0 {
                write!(f, "/")?;
            } else if i % 3 == 0 && i != 0 && i < 6 {
                write!(f, ".")?;
            }
            write!(f, "{}", number)?;
        }
        Ok(())
    }
}

impl FromStr for Cnpj {
    type Err = ParseCnpjError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers = [0; 14];

        if s.is_empty() {
            return Err(ParseCnpjError::Empty);
        }

        // Checks for invalid symbols and converts numbers to integers
        let mut i = 0;
        let mut has_dot = false;
        for (offset, c) in s.chars().enumerate() {
            match (c, offset) {
                ('0'..='9', _) => {
                    if i < 14 {
                        numbers[i] = c.to_digit(10).unwrap() as u8;
                        i += 1;
                    } else {
                        return Err(ParseCnpjError::InvalidNumber);
                    }
                }
                ('.', 2 | 6) => has_dot = true,
                ('/', 10) if has_dot => continue,
                ('/', 8) if !has_dot => continue,
                ('-', 15) if has_dot => continue,
                ('-', 13) if !has_dot => continue,
                _ => return Err(ParseCnpjError::InvalidCharacter(c, offset)),
            }
        }

        // Checks for repeated numbers
        let first_number = numbers[0];
        if numbers.iter().all(|&x| x == first_number) {
            return Err(ParseCnpjError::InvalidNumber);
        }

        for i in 0..=1 {
            let check_digit = numbers[12 + i];
            let mut remainder = numbers
                .iter()
                // Includes the first check digit in the second iteration
                .take(12 + i)
                // 5, 4, 3, 2, 9, 8, 7, ... 3, 2; and after: 6, 5, 4, 3, 2, 9, 8, 7, ... 3, 2
                .zip((2..=9).chain(2..=5 + i).rev())
                .map(|(&x, y)| u32::from(x) * y as u32)
                .sum::<u32>()
                * 10
                % 11;

            if let 10 | 11 = remainder {
                remainder = 0;
            }

            if remainder != u32::from(check_digit) {
                return Err(ParseCnpjError::InvalidNumber);
            }
        }

        Ok(Cnpj(numbers))
    }
}

#[cfg(feature = "rand")]
impl Distribution<Cnpj> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Cnpj {
        let mut numbers = [0; 14];
        for number in numbers.iter_mut().take(8) {
            *number = rng.gen_range(0, 9);
        }
        numbers[11] = 1; // Company headquarters

        for i in 0..=1 {
            let mut remainder = numbers
                .iter()
                // Includes the first check digit in the second iteration
                .take(12 + i)
                // 5, 4, 3, 2, 9, 8, 7, ... 3, 2; and after: 6, 5, 4, 3, 2, 9, 8, 7, ... 3, 2
                .zip((2..=9).chain(2..=5 + i).rev())
                .map(|(&x, y)| u32::from(x) * y as u32)
                .sum::<u32>()
                * 10
                % 11;

            if let 10 | 11 = remainder {
                remainder = 0;
            }

            numbers[12 + i] = remainder as u8; // check digit
        }

        Cnpj(numbers)
    }
}

#[cfg(feature = "serde")]
impl Serialize for Cnpj {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() {
            serializer.serialize_str(&self.to_string())
        } else {
            serializer.serialize_bytes(&self.as_ref())
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Cnpj {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        if deserializer.is_human_readable() {
            struct CnpjStringVisitor;

            impl<'vi> de::Visitor<'vi> for CnpjStringVisitor {
                type Value = Cnpj;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    write!(formatter, "a CNPJ string")
                }

                fn visit_str<E: de::Error>(self, value: &str) -> Result<Cnpj, E> {
                    value.parse::<Cnpj>().map_err(E::custom)
                }

                fn visit_bytes<E: de::Error>(self, value: &[u8]) -> Result<Cnpj, E> {
                    Cnpj::try_from(value).map_err(E::custom)
                }
            }

            deserializer.deserialize_str(CnpjStringVisitor)
        } else {
            struct CnpjBytesVisitor;

            impl<'vi> de::Visitor<'vi> for CnpjBytesVisitor {
                type Value = Cnpj;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    write!(formatter, "bytes")
                }

                fn visit_bytes<E: de::Error>(self, value: &[u8]) -> Result<Cnpj, E> {
                    Cnpj::try_from(value).map_err(E::custom)
                }
            }

            deserializer.deserialize_bytes(CnpjBytesVisitor)
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
        let a = Cnpj([1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 1, 9, 5]);
        let b = Cnpj([1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 2, 7, 2, 4]);
        let c: [u8; 14] = [1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 1, 9, 5];
        let d: [u8; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 1];
        let e: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
        let f: [u8; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 2, 7];

        assert_eq!(a, Cnpj::from_slice(&c).unwrap());
        assert_eq!(a, Cnpj::from_slice(&d).unwrap());
        assert_eq!(a, Cnpj::from_slice(&e).unwrap());
        assert_eq!(b, Cnpj::from_slice(&f).unwrap());
    }

    #[test]
    fn as_bytes() {
        let a: [u8; 14] = [1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 1, 9, 5];
        let b = Cnpj([1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 1, 9, 5]);

        assert_eq!(&a, b.as_bytes());
    }

    #[test]
    fn branch() {
        let cnpj = Cnpj([1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 2, 7, 2, 4]);
        assert_eq!(27, cnpj.branch());
    }

    #[cfg(feature = "rand")]
    #[test]
    fn generate() {
        let a = Cnpj::generate();
        let b = a.to_string().parse::<Cnpj>().unwrap();

        assert_eq!(a, b);
    }

    #[test]
    fn as_ref() {
        fn test_trait<T: AsRef<[u8]>>(b: T) {
            let a: [u8; 14] = [1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 1, 9, 5];
            assert_eq!(&a, b.as_ref());
        }

        let b = Cnpj([1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 1, 9, 5]);

        test_trait(b);
    }

    #[test]
    fn try_from() {
        let a: [u8; 14] = [1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 1, 9, 5];
        let b = Cnpj([1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 1, 9, 5]);

        assert_eq!(Cnpj::try_from(&a).unwrap(), b);
    }

    #[test]
    fn cmp() {
        let a = Cnpj([1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 1, 9, 5]);
        let b = Cnpj([1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 2, 7, 2, 4]);

        assert!(a < b);
    }

    #[test]
    fn debug() {
        let a = r#"Cnpj("12.345.678/0001-95")"#;
        let b = Cnpj([1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 1, 9, 5]);

        assert_eq!(a, format!("{:?}", b));
    }

    #[test]
    fn display() {
        let a = "12.345.678/0001-95";
        let b = Cnpj([1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 1, 9, 5]);

        assert_eq!(a, format!("{}", b));
    }

    #[test]
    fn from_str() {
        let a = "12.345.678/0001-95".parse::<Cnpj>().unwrap();
        let b = "12345678/0001-95".parse::<Cnpj>().unwrap();
        let c = "12345678000195".parse::<Cnpj>().unwrap();

        assert_eq!(a, b);
        assert_eq!(a, c);
        assert_eq!("".parse::<Cnpj>(), Err(ParseCnpjError::Empty));
        assert_eq!(
            "12-345-678/0001-95".parse::<Cnpj>(),
            Err(ParseCnpjError::InvalidCharacter('-', 2))
        );
        assert_eq!(
            "12.345.678/0001-96".parse::<Cnpj>(),
            Err(ParseCnpjError::InvalidNumber)
        );
        assert_eq!(
            "12.345.678/0001-995".parse::<Cnpj>(),
            Err(ParseCnpjError::InvalidNumber)
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serialize_readable() {
        use serde_test::Configure;

        let cnpj_str = "12.345.678/0001-95";
        let cnpj = Cnpj::from_str(cnpj_str).unwrap();
        serde_test::assert_tokens(&cnpj.readable(), &[serde_test::Token::Str(cnpj_str)]);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serialize_compact() {
        use serde_test::Configure;

        let cnpj_bytes = &[1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 1, 9, 5];
        let cnpj = Cnpj::try_from(cnpj_bytes).unwrap();
        serde_test::assert_tokens(&cnpj.compact(), &[serde_test::Token::Bytes(cnpj_bytes)]);
    }
}
