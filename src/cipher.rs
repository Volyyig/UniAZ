/// 用于执行多进制加密和解密的结构体
pub struct Cipher {
    alphabet: Vec<char>,
    radix: u64,
    // 【优化点】使用固定大小数组作为查找表。
    // ASCII 字符集只有 128 或 256 个字符，使用 256 元素足以覆盖常用编码。
    val_map_array: [u64; 256],
}

impl Cipher {
    /// 创建一个新的加密器，并预计算查找表
    pub fn new(pattern: &str) -> Self {
        let alphabet: Vec<char> = pattern.chars().collect();
        let radix = alphabet.len() as u64;

        if radix == 0 {
            panic!("Alphabet cannot be empty");
        }

        // 1. 初始化数组：用一个不可能的哨兵值（如 u64::MAX）填充，表示“不在字母表中”。
        let mut val_map_array = [u64::MAX; 256];

        // 2. 填充数组：将字母表中的字符映射到它们的索引值。
        for (i, &c) in alphabet.iter().enumerate() {
            let index = c as usize;
            if index >= 256 {
                panic!("Character set contains non-ASCII characters that exceed the 256 lookup limit.");
            }
            val_map_array[index] = i as u64;
        }

        Cipher { alphabet, radix, val_map_array }
    }

    /// 辅助函数：查找字符在字符集中的索引 (O(1) 查找)
    /// 【优化点】直接使用数组索引访问，性能最高。
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

    // ----------------------------------------------------
    // 以下函数逻辑不变，内部调用 char_to_val 时的性能是 O(1)
    // ----------------------------------------------------

    /// 核心辅助函数：计算大数取模
    fn get_seed_mod(&self, digits: &[char], skip_idx: usize, modulus: u64) -> usize {
        if modulus == 0 { return 0; }

        let mut remainder: u64 = 0;
        let mut is_empty = true;

        for (i, &c) in digits.iter().enumerate() {
            if i == skip_idx { continue; }
            is_empty = false;

            // O(1) 查找
            let val = self.char_to_val(c);

            remainder = (remainder * self.radix + val) % modulus;
        }

        if is_empty { 0 } else { remainder as usize }
    }

    /// 生成乱序用的替换表 (基于 Fisher-Yates 变体的 O(N) 逻辑)
    fn disorder(&self, digits: &[char], skip_idx: usize) -> Vec<char> {
        let mut obj = self.alphabet.clone();

        for i in (1..obj.len()).rev() {
            let current_length = (i + 1) as u64;
            let j = self.get_seed_mod(digits, skip_idx, current_length);
            obj.swap(i, j);
        }
        obj
    }

    /// 通用加密函数
    fn encrypt_once(&self, input: &str) -> String {
        let mut digit_list: Vec<char> = input.chars().collect();
        let len = digit_list.len();

        for i in 0..len {
            let current_char = digit_list[i];
            let capacity = self.disorder(&digit_list, i);
            let seed_mod_radix = self.get_seed_mod(&digit_list, i, self.radix);
            let offset = seed_mod_radix + i.pow(2) + 1;

            if let Some(pos) = capacity.iter().position(|&x| x == current_char) {
                let new_pos = (pos + offset) % capacity.len();
                digit_list[i] = capacity[new_pos];
            }
        }
        digit_list.into_iter().collect()
    }

    /// 通用解密函数
    fn decrypt_once(&self, input: &str) -> String {
        let mut digit_list: Vec<char> = input.chars().collect();
        let len = digit_list.len();

        for i in (0..len).rev() {
            let current_char = digit_list[i];
            let mut capacity = self.disorder(&digit_list, i);
            capacity.reverse();

            let seed_mod_radix = self.get_seed_mod(&digit_list, i, self.radix);
            let offset = seed_mod_radix + i.pow(2) + 1;

            if let Some(pos) = capacity.iter().position(|&x| x == current_char) {
                let new_pos = (pos + offset) % capacity.len();
                digit_list[i] = capacity[new_pos];
            }
        }
        digit_list.into_iter().collect()
    }

    pub fn encrypt(&self, input: &str, iteration: usize) -> String {
        let mut res = String::from(input);
        for _ in 0..iteration {
            res = self.encrypt_once(&res);
        }
        res
    }

    pub fn decrypt(&self, input: &str, iteration: usize) -> String {
        let mut res = String::from(input);
        for _ in 0..iteration {
            res = self.decrypt_once(&res);
        }
        res
    }
}

// ---------------- 测试部分 ----------------

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn test_decimal() {
        // 传统的十进制测试
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
        // 十六进制测试
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
        // Base62 (数字 + 小写 + 大写)
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
        // 二进制测试 (只有 0 和 1)
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
        for i in "你好世界 鼻".chars() {
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
        for i in "你好世界 鼻".chars() {
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