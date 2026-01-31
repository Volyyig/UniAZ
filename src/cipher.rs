/// Structure for performing multi-base encryption and decryption.
pub struct Cipher {
    /// The character set (alphabet) used for the base.
    alphabet: Vec<char>,
    /// The radix (base) of the cipher, equal to the alphabet length.
    radix: u64,
    /// Fixed-size lookup table (up to 256 ASCII chars) for quick character-to-value mapping (O(1)).
    val_map_array: [u64; 256],
}

impl Cipher {
    /// Creates a new Cipher and pre-calculates the character-to-value lookup table.
    pub fn new(pattern: &str) -> Self {
        let alphabet: Vec<char> = pattern.chars().collect();
        let radix = alphabet.len() as u64;

        if radix == 0 {
            panic!("Alphabet cannot be empty");
        }

        // Initialize the array with a sentinel value (u64::MAX) indicating "not in alphabet".
        let mut val_map_array = [u64::MAX; 256];

        // Populate the array by mapping characters to their index value.
        for (i, &c) in alphabet.iter().enumerate() {
            let index = c as usize;
            if index >= 256 {
                panic!("Character set contains non-ASCII characters that exceed the 256 lookup limit.");
            }
            val_map_array[index] = i as u64;
        }

        Cipher { alphabet, radix, val_map_array }
    }

    /// Helper function: Retrieves the index (value) of a character in the alphabet (O(1) lookup).
    fn char_to_val(&self, c: char) -> u64 {
        let index = c as usize;
        if index >= 256 {
            panic!("Non-ASCII character encountered during encryption/decryption.");
        }

        let val = self.val_map_array[index];

        if val == u64::MAX {
            panic!("Input string contains character not defined in the alphabet.");
        }
        val
    }

    /// Core helper function: Calculates the large number modulus based on input digits.
    /// This result is used as a seed for disordering/shifting.
    fn get_seed_mod(&self, digits: &[char], skip_idx: usize, modulus: u64) -> usize {
        if modulus == 0 { return 0; }

        let mut remainder: u64 = 0;
        let mut is_empty = true;

        for (i, &c) in digits.iter().enumerate() {
            if i == skip_idx { continue; }
            is_empty = false;

            // O(1) lookup
            let val = self.char_to_val(c);

            remainder = (remainder * self.radix + val) % modulus;
        }

        if is_empty { 0 } else { remainder as usize }
    }

    /// Generates a disordered replacement table (permutation) based on a seed derived from input digits.
    /// Returns the permutation and a char->index lookup table for O(1) position queries.
    fn disorder(&self, digits: &[char], skip_idx: usize) -> (Vec<char>, [u8; 256]) {
        let mut obj = self.alphabet.clone();

        for i in (1..obj.len()).rev() {
            let current_length = (i + 1) as u64;
            let j = self.get_seed_mod(digits, skip_idx, current_length);
            obj.swap(i, j);
        }

        // Build O(1) char->index lookup for the permutation.
        let mut pos_map = [255u8; 256];
        for (idx, &c) in obj.iter().enumerate() {
            pos_map[c as usize] = idx as u8;
        }
        (obj, pos_map)
    }

    /// Single-iteration forward encryption function.
    fn encrypt_once(&self, input: &str) -> String {
        let mut digit_list: Vec<char> = input.chars().collect();
        let len = digit_list.len();

        for i in 0..len {
            let current_char = digit_list[i];
            let (capacity, pos_map) = self.disorder(&digit_list, i);
            let seed_mod_radix = self.get_seed_mod(&digit_list, i, self.radix);
            let offset = seed_mod_radix as u64 + (i * i) as u64 + 1;

            let pos = pos_map[current_char as usize];
            if pos != 255 {
                let new_pos = ((pos as u64 + offset) % self.radix) as usize;
                digit_list[i] = capacity[new_pos];
            }
        }
        digit_list.into_iter().collect()
    }

    /// Single-iteration backward decryption function.
    fn decrypt_once(&self, input: &str) -> String {
        let mut digit_list: Vec<char> = input.chars().collect();
        let len = digit_list.len();

        for i in (0..len).rev() {
            let current_char = digit_list[i];
            let (mut capacity, mut pos_map) = self.disorder(&digit_list, i);
            capacity.reverse();

            // Rebuild pos_map for reversed permutation.
            for (idx, &c) in capacity.iter().enumerate() {
                pos_map[c as usize] = idx as u8;
            }

            let seed_mod_radix = self.get_seed_mod(&digit_list, i, self.radix);
            let offset = seed_mod_radix as u64 + (i * i) as u64 + 1;

            let pos = pos_map[current_char as usize];
            if pos != 255 {
                let new_pos = ((pos as u64 + offset) % self.radix) as usize;
                digit_list[i] = capacity[new_pos];
            }
        }
        digit_list.into_iter().collect()
    }

    /// Encrypts the input string for a specified number of iterations.
    pub fn encrypt(&self, input: &str, iteration: usize) -> String {
        let mut res = String::from(input);
        for _ in 0..iteration {
            res = self.encrypt_once(&res);
        }
        res
    }

    /// Decrypts the input string for a specified number of iterations.
    pub fn decrypt(&self, input: &str, iteration: usize) -> String {
        let mut res = String::from(input);
        for _ in 0..iteration {
            res = self.decrypt_once(&res);
        }
        res
    }
}

// ---------------- Test Section ----------------

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn test_decimal() {
        // Standard decimal test
        let cipher = Cipher::new("0123456789");
        let original = "1234567890";
        let encrypted = cipher.encrypt_once(original);
        let decrypted = cipher.decrypt_once(&encrypted);

        println!("Decimal - Original: {}", original);
        println!("Decimal - Encrypted: {}", encrypted);
        println!("Decimal - Decrypted: {}", decrypted);

        assert_eq!(original, &decrypted);
        assert_ne!(original, &encrypted);
    }

    #[test]
    fn test_hex() {
        // Hexadecimal test
        let cipher = Cipher::new("0123456789ABCDEF");
        let original = "A1F90";
        let encrypted = cipher.encrypt_once(original);
        let decrypted = cipher.decrypt_once(&encrypted);

        println!("Hex - Original: {}", original);
        println!("Hex - Encrypted: {}", encrypted);

        assert_eq!(original, &decrypted);
    }

    #[test]
    fn test_base62() {
        // Base62 (Digits + Lowercase + Uppercase)
        let base62_pattern = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let cipher = Cipher::new(base62_pattern);

        let original = "HelloRust2025";
        let encrypted = cipher.encrypt_once(original);
        let decrypted = cipher.decrypt_once(&encrypted);

        println!("Base62 - Original: {}", original);
        println!("Base62 - Encrypted: {}", encrypted);

        assert_eq!(original, &decrypted);
    }

    #[test]
    fn test_binary() {
        // Binary test (only 0 and 1)
        let cipher = Cipher::new("01");
        let original = "110101011100";
        let encrypted = cipher.encrypt_once(original);
        let decrypted = cipher.decrypt_once(&encrypted);
        println!("Binary - Original: {}", original);
        println!("Binary - Encrypted: {}", encrypted);
        assert_eq!(original, &decrypted);
    }

    #[test]
    fn test_distribution() {
        use std::collections::HashMap;
        let mut flag = HashMap::new();
        flag.insert('0', 0);
        flag.insert('1', 0);
        flag.insert('2', 0);
        flag.insert('3', 0);
        flag.insert('4', 0);
        flag.insert('5', 0);
        flag.insert('6', 0);
        flag.insert('7', 0);
        flag.insert('8', 0);
        flag.insert('9', 0);

        let c = Cipher::new("0123456789");

        for i in 0..50000 {
            let mut res = format!("{i:05}");
            for _ in 0..2 {
                res = c.encrypt_once(&res);
            }
            print!("{}, ", u32::from_str(&res).unwrap());
            let initial = res.chars().next().unwrap();
            *flag.get_mut(&initial).unwrap() += 1;
        }
        let avg = flag.values().sum::<u32>() / 10;

        println!();
        println!("{:?}", flag);
        println!("{:?}", avg);
    }

    #[test]
    fn test_alphabet() {
        use anybase::Converter;
        let alphabet = "abcdefghijklmnopqrstuvwxyz";
        let cipher = Cipher::new(alphabet);
        let converter = Converter::new("0123456789", alphabet);
        for i in "你好世界".chars() {
            let original = converter.convert(&(i as u32).to_string()).unwrap();
            println!("{} {:?}", i, original);
            let encrypted = cipher.encrypt_once(&original);
            let encrypted = cipher.encrypt_once(&encrypted);
            let decrypted = cipher.decrypt_once(&encrypted);
            let decrypted = cipher.decrypt_once(&decrypted);
            println!("Alphabet - Original: {}", original);
            println!("Alphabet - Encrypted: {}", encrypted);
            println!("Alphabet - Decrypted: {}", decrypted);
        }
    }

    #[test]
    fn test_alphabet_iteration() {
        use anybase::Converter;
        let alphabet = "abcdefghijklmnopqrstuvwxyz";
        let cipher = Cipher::new(alphabet);
        let converter = Converter::new("0123456789", alphabet);
        for i in "你好世界".chars() {
            let original = converter.convert(&(i as u32).to_string()).unwrap();
            println!("{} {:?}", i, original);
            let encrypted = cipher.encrypt(&original, 2);
            let decrypted = cipher.decrypt(&encrypted, 2);
            println!("Alphabet - Original: {}", original);
            println!("Alphabet - Encrypted: {}", encrypted);
            println!("Alphabet - Decrypted: {}", decrypted);
        }
    }
}