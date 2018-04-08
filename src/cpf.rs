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

/// An error which can be returned when parsing an CPF/ICN number.
#[derive(Fail, Debug)]
pub enum ParseCpfError {
    #[fail(display = "Empty string.")]
    Empty,
    #[fail(display = "Invalid character `{}` at offset {}.", _0, _1)]
    InvalidCharacter(char, usize),
    #[fail(display = "Invalid CPF number.")]
    InvalidNumber,
}

/// A valid CPF/ICN number. Parsing recognizes numbers with or without separators (dot, minus,
/// slash, and space).
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cpf {
    numbers: [u8; 11],
}

impl Cpf {
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
    /// use brids::Cpf;
    ///
    /// let cpf = Cpf::generate();
    /// ```
    #[cfg(feature = "random")]
    #[inline]
    pub fn generate() -> Self {
        thread_rng().gen()
    }
}

impl fmt::Debug for Cpf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cpf(\"{}\")", self.to_string())
    }
}

impl fmt::Display for Cpf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, number) in self.numbers.iter().enumerate() {
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

        // Must start with a number
        let mut chars = s.chars();
        let first_number = match chars.next() {
            Some(c) => match c {
                '0'...'9' => c.to_digit(10).unwrap() as u8,
                _ => return Err(ParseCpfError::InvalidCharacter(c, 0)),
            },
            None => return Err(ParseCpfError::Empty),
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
                _ => return Err(ParseCpfError::InvalidCharacter(c, offset + 1)),
            };
        }

        // Checks for repeated numbers
        if numbers.iter().all(|&c| c == first_number) {
            return Err(ParseCpfError::InvalidNumber);
        }

        for i in 0..2 {
            let check_digit = numbers[9 + i];
            let mut remainder = numbers
                .iter()
                // Includes the first check digit in the second iteration
                .take(9 + i)
                // 10, 9, 8, ... 3, 2; and after: 11, 10, 9, 8, ... 3, 2
                .zip((2..11 + i).rev())
                .map(|(&n, x)| n as u32 * x as u32)
                .sum::<u32>() * 10 % 11;

            if remainder == 10 || remainder == 11 {
                remainder = 0;
            }

            if remainder != check_digit as u32 {
                return Err(ParseCpfError::InvalidNumber);
            }
        }

        Ok(Self { numbers })
    }
}

#[cfg(feature = "random")]
impl Rand for Cpf {
    #[inline]
    fn rand<R: Rng>(rng: &mut R) -> Self {
        let mut numbers = [0; 11];
        for number in numbers.iter_mut().take(9) {
            *number = rng.gen_range(0, 9);
        }

        for i in 0..2 {
            let mut check_digit = numbers
                .iter()
                // Includes the first check digit in the second iteration
                .take(9 + i)
                // 10, 9, 8, ... 3, 2; and after: 11, 10, 9, 8, ... 3, 2
                .zip((2..11 + i).rev())
                .map(|(&n, x)| n as u32 * x as u32)
                .sum::<u32>() * 10 % 11;

            if check_digit == 10 || check_digit == 11 {
                check_digit = 0;
            }

            numbers[9 + i] = check_digit as u8;
        }

        Self { numbers }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_bytes() {
        let a: [u8; 11] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 9];
        let b = Cpf {
            numbers: [1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 9],
        };

        assert_eq!(a, *b.as_bytes());
    }

    #[cfg(feature = "random")]
    #[test]
    fn test_generate() {
        let a = Cpf::generate();
        let b = a.to_string().parse::<Cpf>().unwrap();

        assert_eq!(a, b);
    }

    #[test]
    fn test_cpf_fmt() {
        let a = "123.456.789-09";
        let b = Cpf {
            numbers: [1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 9],
        };

        assert_eq!(a, b.to_string());
    }

    #[test]
    fn test_cpf_from_str() {
        let a = "123.456.789-09".parse::<Cpf>().unwrap();
        let b = "123.456.789/09".parse::<Cpf>().unwrap();
        let c = "12345678909".parse::<Cpf>().unwrap();
        let d = "123 456 789 09".parse::<Cpf>().unwrap();

        assert_eq!(a, b);
        assert_eq!(a, c);
        assert_eq!(a, d);
        matches!("".parse::<Cpf>(), Err(ParseCpfError::Empty));
        matches!(
            "123;456;789/09".parse::<Cpf>(),
            Err(ParseCpfError::InvalidCharacter(_, _))
        );
        matches!(
            "123.456.789-10".parse::<Cpf>(),
            Err(ParseCpfError::InvalidNumber)
        );
    }
}
