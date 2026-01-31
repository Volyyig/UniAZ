//! UniAZ - A Unicode encryption library
//!
//! This crate provides functionality to unify arbitrary Unicode characters into a-z.
//!
//! # Examples
//!
//! ```
//! use uniaz::UniAz;
//!
//! let uni_az = UniAz::new();
//! let encrypted = uni_az.encrypt('你');
//! let decrypted = uni_az.decrypt(&encrypted).unwrap();
//! assert_eq!(decrypted, '你');
//! ```

use std::str::FromStr;
use crate::cipher::Cipher;
use anybase::Converter;
mod cipher;

/// Main interface for Unicode character encryption and decryption
///
/// The `UniAz` struct provides a high-level API for encrypting individual Unicode
/// characters and decrypting them back to their original form.
///
/// Internally, it uses:
/// - A converter to transform Unicode code points to a custom base representation
/// - A cipher with a predefined alphabet for encryption
/// - A reverse converter to transform back to Unicode characters
pub struct UniAz {
    converter: Converter<'static>,
    rev_converter: Converter<'static>,
    cipher: Cipher,
}

impl UniAz {
    /// Creates a new `UniAz` instance with the default configuration
    ///
    /// Uses the Latin alphabet (`abcdefghijklmnopqrstuvwxyz`) as the internal
    /// cipher alphabet and base-10 numbers as the source representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use uniaz::UniAz;
    ///
    /// let uni_az = UniAz::new();
    /// ```
    pub fn new() -> Self {
        const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
        let converter = Converter::new("0123456789", ALPHABET);
        let rev_converter = converter.inverse();

        UniAz {
            converter,
            rev_converter,
            cipher: Cipher::new(ALPHABET),
        }
    }
    
    /// Encrypts a single Unicode character
    ///
    /// Takes a character, converts it to its Unicode code point, represents
    /// that number in the internal alphabet base, then encrypts it twice.
    ///
    /// # Arguments
    ///
    /// * `plain` - The character to encrypt
    ///
    /// # Returns
    ///
    /// A `String` containing the encrypted representation of the character
    ///
    /// # Examples
    ///
    /// ```
    /// use uniaz::UniAz;
    ///
    /// let uni_az = UniAz::new();
    /// let encrypted = uni_az.encrypt('A');
    /// ```
    pub fn encrypt(&self, plain: char) -> String {
        let numeric = (plain as u32).to_string();
        let converted = self
            .converter
            .convert(&numeric)
            .expect("converter: valid decimal string for Unicode codepoint");
        self.cipher.encrypt(&converted, 2)
    }
    
    /// Decrypts an encrypted string back to a Unicode character
    ///
    /// Takes an encrypted string, decrypts it twice, converts the result
    /// from the internal alphabet base back to a number, and interprets
    /// that number as a Unicode code point.
    ///
    /// # Arguments
    ///
    /// * `cipher` - A reference to the encrypted string (must contain only a-z)
    ///
    /// # Returns
    ///
    /// `Ok(char)` with the original character, or `Err(DecryptError)` if the
    /// cipher text is invalid, corrupted, or tampered with.
    ///
    /// # Examples
    ///
    /// ```
    /// use uniaz::UniAz;
    ///
    /// let uni_az = UniAz::new();
    /// let encrypted = uni_az.encrypt('A');
    /// let decrypted = uni_az.decrypt(&encrypted).unwrap();
    /// assert_eq!(decrypted, 'A');
    /// ```
    pub fn decrypt(&self, cipher: &str) -> Result<char, DecryptError> {
        if !cipher.chars().all(|c| c.is_ascii_lowercase()) {
            return Err(DecryptError::InvalidCipherText);
        }
        let decrypted = self.cipher.decrypt(cipher, 2);
        let numeric = self
            .rev_converter
            .convert(&decrypted)
            .map_err(|_| DecryptError::InvalidToken)?;
        let cp = u32::from_str(&numeric).map_err(|_| DecryptError::InvalidToken)?;
        char::from_u32(cp).ok_or(DecryptError::InvalidCodepoint)
    }

    /// Encrypts a string by encrypting each character and joining with spaces.
    ///
    /// # Examples
    ///
    /// ```
    /// use uniaz::UniAz;
    ///
    /// let uni_az = UniAz::new();
    /// let encrypted = uni_az.encrypt_str("你好");
    /// let decrypted = uni_az.decrypt_str(&encrypted).unwrap();
    /// assert_eq!(decrypted, "你好");
    /// ```
    pub fn encrypt_str(&self, text: &str) -> String {
        text.chars()
            .map(|c| self.encrypt(c))
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Decrypts a string that was encrypted with [`encrypt_str`](Self::encrypt_str).
    ///
    /// Expects space-separated encrypted tokens. Returns an error if any token is invalid.
    pub fn decrypt_str(&self, text: &str) -> Result<String, DecryptError> {
        let mut result = String::new();
        for token in text.split_whitespace() {
            if token.is_empty() {
                continue;
            }
            let c = self.decrypt(token)?;
            result.push(c);
        }
        Ok(result)
    }
}

impl Default for UniAz {
    fn default() -> Self {
        Self::new()
    }
}

/// Error type for decryption failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecryptError {
    /// The cipher text contains invalid characters (expects only a-z).
    InvalidCipherText,
    /// The encrypted token could not be decoded (corrupted or tampered).
    InvalidToken,
    /// The decoded value is not a valid Unicode codepoint.
    InvalidCodepoint,
}

impl std::fmt::Display for DecryptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecryptError::InvalidCipherText => write!(f, "cipher text must contain only a-z"),
            DecryptError::InvalidToken => write!(f, "invalid or corrupted cipher token"),
            DecryptError::InvalidCodepoint => write!(f, "decoded value is not a valid Unicode codepoint"),
        }
    }
}

impl std::error::Error for DecryptError {}

#[cfg(test)]
mod tests {
    use crate::UniAz;

    #[test]
    fn test_char() {
        let u = UniAz::new();
        let e = u.encrypt('你');
        let p = u.decrypt(&e).unwrap();
        assert_eq!(p, '你');
    }

    #[test]
    fn test_emoji() {
        let u = UniAz::new();
        let e = u.encrypt('😀');
        let p = u.decrypt(&e).unwrap();
        assert_eq!(p, '😀');
    }

    #[test]
    fn test_decrypt_invalid() {
        let u = UniAz::new();
        assert!(u.decrypt("!!").is_err());
        assert!(u.decrypt("ab12").is_err());
    }

    #[test]
    fn test_encrypt_decrypt_str() {
        let u = UniAz::new();
        let encrypted = u.encrypt_str("你好世界");
        let decrypted = u.decrypt_str(&encrypted).unwrap();
        assert_eq!(decrypted, "你好世界");
    }

    #[test]
    #[ignore = "takes ~minutes to run all 1.1M codepoints"]
    fn test_all_codepoints() {
        let u = UniAz::new();
        for i in 0..=0x10FFFF {
            if let Some(c) = char::from_u32(i) {
                let e = u.encrypt(c);
                let p = u.decrypt(&e).unwrap();
                assert_eq!(c, p);
            }
        }
    }
}