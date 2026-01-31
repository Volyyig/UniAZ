use std::io::{self, Write};
use uniaz::UniAz;

fn main() {
    let u = UniAz::new();
    loop {
        print!("请输入文本: ");
        io::stdout().flush().unwrap();

        let mut text = String::new();
        io::stdin().read_line(&mut text).unwrap();
        let text = text.trim();

        if text.is_empty() {
            break;
        }

        let encrypted = u.encrypt_str(text);
        println!("{encrypted}");
    }
}
