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

use failure::Fail;
#[cfg(feature = "random")]
use rand::{
    distributions::{Distribution, Standard},
    thread_rng, Rng,
};
use std::{fmt, str::FromStr};

/// An error which can be returned when parsing an CNPJ number.
#[derive(Fail, Debug, PartialEq, Eq)]
pub enum ParseCnpjError {
    #[fail(display = "Empty string.")]
    Empty,
    #[fail(display = "Invalid character `{}` at offset {}.", _0, _1)]
    InvalidCharacter(char, usize),
    #[fail(display = "Invalid CNPJ number.")]
    InvalidNumber,
}

/// A valid CNPJ number. Parsing recognizes numbers with or without separators (dot, minus, slash,
/// and space).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cnpj {
    numbers: [u8; 14],
}

impl Cnpj {
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
        &self.numbers
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
        self.numbers[8..=11]
            .iter()
            .rev()
            .enumerate()
            .map(|(i, &x)| u16::from(x) * 10u16.pow(i as u32))
            .sum::<u16>()
    }

    /// Generates a random number, using [`rand::thread_rng`][rand] (optional dependency enabled
    /// by default). To use a different generator, instantiate the generator directly.
    ///
    /// [rand]: https://crates.io/crates/rand
    ///
    /// # Examples
    ///
    /// Basic use:
    ///
    /// ```rust
    /// use brids::Cnpj;
    ///
    /// let cnpj = Cnpj::generate();
    /// ```
    #[cfg(feature = "random")]
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

impl fmt::Debug for Cnpj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cnpj(\"{}\")", self)
    }
}

impl fmt::Display for Cnpj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}.", self.numbers[0], self.numbers[1])?;
        for (i, number) in self.numbers.iter().skip(2).enumerate() {
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

        // Must start with a number
        let mut chars = s.chars();
        let first_number = match chars.next() {
            Some(c @ '0'..='9') => c.to_digit(10).unwrap() as u8,
            Some(c) => return Err(ParseCnpjError::InvalidCharacter(c, 0)),
            None => return Err(ParseCnpjError::Empty),
        };
        numbers[0] = first_number;

        // Checks for invalid symbols and converts numbers to integers
        let mut i = 0;
        for (offset, c) in chars.enumerate() {
            match c {
                '0'..='9' => {
                    if i < 13 {
                        numbers[i + 1] = c.to_digit(10).unwrap() as u8;
                        i += 1;
                    } else {
                        return Err(ParseCnpjError::InvalidNumber);
                    }
                }
                '.' | '-' | '/' | ' ' => continue,
                _ => return Err(ParseCnpjError::InvalidCharacter(c, offset + 1)),
            };
        }

        // Checks for repeated numbers
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

            if remainder == 10 || remainder == 11 {
                remainder = 0;
            }

            if remainder != u32::from(check_digit) {
                return Err(ParseCnpjError::InvalidNumber);
            }
        }

        Ok(Self { numbers })
    }
}

#[cfg(feature = "random")]
impl Distribution<Cnpj> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Cnpj {
        let mut numbers = [0; 14];
        for number in numbers.iter_mut().take(8) {
            *number = rng.gen_range(0, 9);
        }
        numbers[11] = 1; // Company headquarters

        for i in 0..=1 {
            let mut check_digit = numbers
                .iter()
                // Includes the first check digit in the second iteration
                .take(12 + i)
                // 5, 4, 3, 2, 9, 8, 7, ... 3, 2; and after: 6, 5, 4, 3, 2, 9, 8, 7, ... 3, 2
                .zip((2..=9).chain(2..=5 + i).rev())
                .map(|(&x, y)| u32::from(x) * y as u32)
                .sum::<u32>()
                * 10
                % 11;

            if check_digit == 10 || check_digit == 11 {
                check_digit = 0;
            }

            numbers[12 + i] = check_digit as u8;
        }

        Cnpj { numbers }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_bytes() {
        let a: [u8; 14] = [1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 1, 9, 5];
        let b = Cnpj {
            numbers: [1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 1, 9, 5],
        };

        assert_eq!(&a, b.as_bytes());
    }

    #[test]
    fn branch() {
        let cnpj = Cnpj {
            numbers: [1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 2, 7, 2, 4],
        };
        assert_eq!(27, cnpj.branch());
    }

    #[cfg(feature = "random")]
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

        let b = Cnpj {
            numbers: [1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 1, 9, 5],
        };

        test_trait(b);
    }

    #[test]
    fn cmp() {
        let a = Cnpj {
            numbers: [1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 1, 9, 5],
        };
        let b = Cnpj {
            numbers: [1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 2, 7, 2, 4],
        };

        assert!(a < b);
    }

    #[test]
    fn debug() {
        let a = r#"Cnpj("12.345.678/0001-95")"#;
        let b = Cnpj {
            numbers: [1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 1, 9, 5],
        };

        assert_eq!(a, format!("{:?}", b));
    }

    #[test]
    fn display() {
        let a = "12.345.678/0001-95";
        let b = Cnpj {
            numbers: [1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 1, 9, 5],
        };

        assert_eq!(a, format!("{}", b));
    }

    #[test]
    fn from_str() {
        let a = "12.345.678/0001-95".parse::<Cnpj>().unwrap();
        let b = "12345678000195".parse::<Cnpj>().unwrap();
        let c = "12 345 678 0001 95".parse::<Cnpj>().unwrap();

        assert_eq!(a, b);
        assert_eq!(a, c);
        assert_eq!("".parse::<Cnpj>(), Err(ParseCnpjError::Empty));
        assert_eq!(
            "12;345;678/0001-95".parse::<Cnpj>(),
            Err(ParseCnpjError::InvalidCharacter(';', 2))
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
}
