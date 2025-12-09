use std::str::FromStr;
use crate::cipher::Cipher;
use anybase::Converter;
mod cipher;

pub struct UniAz {
    converter: Converter<'static>,
    rev_converter: Converter<'static>,
    cipher: Cipher,
}

impl UniAz {
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
    
    pub fn encrypt(&self, plain: &char) -> String {
        let converted = self.converter.convert(&(*plain as u32).to_string()).unwrap();
        self.cipher.encrypt(&converted, 2)
    }
    
    pub fn decrypt(&self, cipher: &str) -> char {
        let decrypted = self.cipher.decrypt(&cipher, 2);
        char::from_u32(u32::from_str(&self.rev_converter.convert(&decrypted).unwrap()).unwrap()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::UniAz;

    #[test]
    fn it_works() {
        let u =  UniAz::new();
        let e = u.encrypt(&'你');
        let p = u.decrypt(&e);
        println!("{:?}", e);
        println!("{:?}", p);
    }
}