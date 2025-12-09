//! UniAZ - A Unicode encryption library
//!
//! This crate provides functionality to unify the arbitrarily Unicode character into A-Z
//! 
//!
//! # Examples
//!
//! ```
//! use uniaz::UniAz;
//!
//! let uni_az = UniAz::new();
//! let encrypted = uni_az.encrypt(&'你');
//! let decrypted = uni_az.decrypt(&encrypted);
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
            cipher: Cipher::new(ALPHABET)
        }
    }
    
    /// Encrypts a single Unicode character
    ///
    /// Takes a character, converts it to its Unicode code point, represents
    /// that number in the internal alphabet base, then encrypts it twice.
    ///
    /// # Arguments
    ///
    /// * `plain` - A reference to the character to encrypt
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
    /// let encrypted = uni_az.encrypt(&'A');
    /// ```
    pub fn encrypt(&self, plain: &char) -> String {
        let converted = self.converter.convert(&(*plain as u32).to_string()).unwrap();
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
    /// * `cipher` - A reference to the encrypted string
    ///
    /// # Returns
    ///
    /// The original Unicode character
    ///
    /// # Examples
    ///
    /// ```
    /// use uniaz::UniAz;
    ///
    /// let uni_az = UniAz::new();
    /// let encrypted = uni_az.encrypt(&'A');
    /// let decrypted = uni_az.decrypt(&encrypted);
    /// assert_eq!(decrypted, 'A');
    /// ```
    pub fn decrypt(&self, cipher: &str) -> char {
        let decrypted = self.cipher.decrypt(&cipher, 2);
        char::from_u32(u32::from_str(&self.rev_converter.convert(&decrypted).unwrap()).unwrap()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::UniAz;

    #[test]
    fn test_char() {
        let u = UniAz::new();
        let e = u.encrypt(&'你');
        let p = u.decrypt(&e);
        println!("{:?}", e);
        println!("{:?}", p);
    }

    #[test]
    fn test_emoji() {
        let u = UniAz::new();
        let e = u.encrypt(&'😀');
        let p = u.decrypt(&e);
        println!("{:?}", e);
        println!("{:?}", p);
    }
}