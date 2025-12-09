use uniaz::*;
use std::io;
use std::io::Write;

fn main() {
    let u: UniAz = UniAz::new();
    loop {
        print!("请输入文本: ");
        io::stdout().flush().unwrap();
        
        let mut text = String::new();
        io::stdin().read_line(&mut text).unwrap();
        
        let mut res = String::new();
        for c in text.trim().chars() {
            let encrypted = u.encrypt(&c);
            res.push_str(&encrypted);
            res.push(' ');
        }
        println!("{}", res);
        if text.trim().is_empty() { break;}
            
    }
    
    
}
