// Copyright 2018 Ricardo Silva Veloso
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[cfg(feature = "random")]
use rand::{thread_rng, Rand, Rng};
use std::{fmt, str::FromStr};

/// An error which can be returned when parsing an CNPJ number.
#[derive(Fail, Debug)]
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
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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
    pub fn as_bytes(&self) -> &[u8] {
        &self.numbers
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

impl fmt::Debug for Cnpj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cnpj(\"{}\")", self.to_string())
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
            Some(c) => match c {
                '0'...'9' => c.to_digit(10).unwrap() as u8,
                _ => return Err(ParseCnpjError::InvalidCharacter(c, 0)),
            },
            None => return Err(ParseCnpjError::Empty),
        };
        numbers[0] = first_number;

        // Checks for invalid symbols and converts numbers to integers
        let mut i = 0;
        for (offset, c) in chars.enumerate() {
            match c {
                '0'...'9' => {
                    numbers[i + 1] = c.to_digit(10).unwrap() as u8;
                    i += 1;
                }
                '.' | '-' | '/' | ' ' => continue,
                _ => return Err(ParseCnpjError::InvalidCharacter(c, offset + 1)),
            };
        }

        // Checks for repeated numbers
        if numbers.iter().all(|&c| c == first_number) {
            return Err(ParseCnpjError::InvalidNumber);
        }

        for i in 0..2 {
            let check_digit = numbers[12 + i];
            let mut remainder = numbers
                .iter()
                // Includes the first check digit in the second iteration
                .take(12 + i)
                // 5, 4, 3, 2, 9, 8, 7, ... 3, 2; and after: 6, 5, 4, 3, 2, 9, 8, 7, ... 3, 2
                .zip((2..10).chain(2..6 + i).rev())
                .map(|(&n, x)| n as u32 * x as u32)
                .sum::<u32>() * 10 % 11;

            if remainder == 10 || remainder == 11 {
                remainder = 0;
            }

            if remainder != check_digit as u32 {
                return Err(ParseCnpjError::InvalidNumber);
            }
        }

        Ok(Self { numbers })
    }
}

#[cfg(feature = "random")]
impl Rand for Cnpj {
    #[inline]
    fn rand<R: Rng>(rng: &mut R) -> Self {
        let mut numbers = [0; 14];
        for number in numbers.iter_mut().take(12) {
            *number = rng.gen_range(0, 9);
        }

        for i in 0..2 {
            let mut check_digit = numbers
                .iter()
                // Includes the first check digit in the second iteration
                .take(12 + i)
                // 5, 4, 3, 2, 9, 8, 7, ... 3, 2; and after: 6, 5, 4, 3, 2, 9, 8, 7, ... 3, 2
                .zip((2..10).chain(2..6 + i).rev())
                .map(|(&n, x)| n as u32 * x as u32)
                .sum::<u32>() * 10 % 11;

            if check_digit == 10 || check_digit == 11 {
                check_digit = 0;
            }

            numbers[12 + i] = check_digit as u8;
        }

        Self { numbers }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_bytes() {
        let a: [u8; 14] = [1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 1, 9, 5];
        let b = Cnpj {
            numbers: [1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 1, 9, 5],
        };

        assert_eq!(a, *b.as_bytes());
    }

    #[cfg(feature = "random")]
    #[test]
    fn test_generate() {
        let a = Cnpj::generate();
        let b = a.to_string().parse::<Cnpj>().unwrap();

        assert_eq!(a, b);
    }

    #[test]
    fn test_cnpj_fmt() {
        let a = "12.345.678/0001-95";
        let b = Cnpj {
            numbers: [1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 1, 9, 5],
        };

        assert_eq!(a, b.to_string());
    }

    #[test]
    fn test_cnpj_from_str() {
        let a = "12.345.678/0001-95".parse::<Cnpj>().unwrap();
        let b = "12345678000195".parse::<Cnpj>().unwrap();
        let c = "12 345 678 0001 95".parse::<Cnpj>().unwrap();

        assert_eq!(a, b);
        assert_eq!(a, c);
        matches!("".parse::<Cnpj>(), Err(ParseCnpjError::Empty));
        matches!(
            "12;345;678/0001-95".parse::<Cnpj>(),
            Err(ParseCnpjError::InvalidCharacter(_, _))
        );
        matches!(
            "12.345.678/0001-96".parse::<Cnpj>(),
            Err(ParseCnpjError::InvalidNumber)
        );
    }
}
