//! Generates bitflips for byte and UTF-8 strings.
//!
//! Each function returns an iterator that yields each possible output of flipping each bit in the
//! input in turn. [`ascii_bytes`] and [`ascii_str`] essentially return `7 * input.len()`
//! permutations, [`bytes`] returns `8 * input.len()`, and [`utf8`] is... complicated.
//!
//! A very simple example would be:
//!
//! ```
//! for s in bitflip::ascii_str("ab") {
//!     print!("{s} ");
//! }
//! ```
//!
//! Which outputs:
//!
//! ```text
//! `b cb eb ib qb Ab !b ac a` af aj ar aB a"
//! ```
//!
//! This is inspired by — and essentially a direct Rust port of — the Python [`blip`][blip] package
//! by Zack Allen.
//!
//! [blip]: https://pypi.org/project/blip/

/// Flips each bit within the ASCII byte string in turn.
///
/// No check is performed to ensure that the input is actually ASCII. If the input contains bytes
/// with the high bit set, those bits will never be flipped, which probably isn't what you want.
pub fn ascii_bytes(input: &[u8]) -> ByteIterator {
    ByteIterator {
        input: input.to_vec(),
        pos: 0,
        bit: 0,
        max: 7,
    }
}

/// Flips each bit within the ASCII string in turn.
///
/// No check is performed to ensure that the input is actually ASCII. If the input contains bytes
/// with the high bit set, those bits will never be flipped, which probably isn't what you want.
pub fn ascii_str(input: &str) -> StringIterator {
    StringIterator(ascii_bytes(input.as_bytes()))
}

/// Flips each bit within the given byte slice.
///
/// The returned iterator will yield `8 * input.len()` byte vecs.
pub fn bytes(input: &[u8]) -> ByteIterator {
    ByteIterator {
        input: input.to_vec(),
        pos: 0,
        bit: 0,
        max: 8,
    }
}

/// Flips each bit within the given string, then only returns those that are valid UTF-8 in their
/// own right.
pub fn utf8(input: &str) -> StringIterator {
    StringIterator(bytes(input.as_bytes()))
}

/// Iterator returned by functions that yield [`Vec<u8>`].
#[derive(Clone, Debug)]
pub struct ByteIterator {
    input: Vec<u8>,
    pos: usize,
    bit: usize,
    max: usize,
}

impl Iterator for ByteIterator {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.input.len() {
            None
        } else {
            let mut output = self.input.clone();
            output[self.pos] ^= 1 << self.bit;

            self.bit += 1;
            if self.bit >= self.max {
                self.pos += 1;
                self.bit = 0;
            }

            Some(output)
        }
    }
}

/// Iterator returned by functions that yield [`String`].
#[derive(Clone, Debug)]
pub struct StringIterator(ByteIterator);

impl Iterator for StringIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        for next in self.0.by_ref() {
            if let Ok(s) = String::from_utf8(next) {
                return Some(s);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeSet;

    macro_rules! test_bytes {
        ($fn:ident, $input:expr, $expected:expr,) => {
            let results: BTreeSet<Vec<u8>> = $fn($input).collect();
            let expected: BTreeSet<Vec<u8>> =
                $expected.into_iter().map(|bstr| bstr.to_vec()).collect();

            assert_eq!(results, expected);
        };
        ($fn:ident, $input:expr, $expected:expr) => {
            test_bytes!($fn, $input, $expected,)
        };
    }

    macro_rules! test_string {
        ($fn:ident, $input:expr, $expected:expr,) => {
            let results: BTreeSet<String> = $fn($input).collect();
            let expected: BTreeSet<String> = $expected.into_iter().map(String::from).collect();

            assert_eq!(results, expected);
        };
        ($fn:ident, $input:expr, $expected:expr) => {
            test_string!($fn, $input, $expected,)
        };
    }

    #[test]
    fn test_ascii_bytes() {
        test_bytes!(
            ascii_bytes,
            b"abc",
            [
                b"!bc", b"Abc", b"qbc", b"ibc", b"ebc", b"cbc", b"`bc", b"a\"c", b"aBc", b"arc",
                b"ajc", b"afc", b"a`c", b"acc", b"ab#", b"abC", b"abs", b"abk", b"abg", b"aba",
                b"abb",
            ],
        );
    }

    #[test]
    fn test_bytes() {
        test_bytes!(
            bytes,
            b"abc",
            [
                b"!bc", b"Abc", b"qbc", b"ibc", b"ebc", b"cbc", b"`bc", b"a\"c", b"aBc", b"arc",
                b"ajc", b"afc", b"a`c", b"acc", b"ab#", b"abC", b"abs", b"abk", b"abg", b"aba",
                b"abb", b"\xe1bc", b"a\xe2c", b"ab\xe3",
            ]
        );
    }

    #[test]
    fn test_ascii_str() {
        test_string!(
            ascii_str,
            "abc",
            [
                "!bc", "Abc", "qbc", "ibc", "ebc", "cbc", "`bc", "a\"c", "aBc", "arc", "ajc",
                "afc", "a`c", "acc", "ab#", "abC", "abs", "abk", "abg", "aba", "abb",
            ]
        );
    }

    #[test]
    fn test_utf8() {
        test_string!(
            utf8,
            "abc",
            [
                "!bc", "Abc", "qbc", "ibc", "ebc", "cbc", "`bc", "a\"c", "aBc", "arc", "ajc",
                "afc", "a`c", "acc", "ab#", "abC", "abs", "abk", "abg", "aba", "abb",
            ]
        );
    }
}
