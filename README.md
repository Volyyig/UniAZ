# UniAz 🔐

UniAz is a small Rust crate that provides a simple, Unicode-aware way to
encode a single Unicode character into the alphabet and recover it back.
It combines a base converter and a light-weight cipher permutation to produce
obfuscated string representations of characters. It is intended to provide a readable, mappable, and easy-to-remember alternative representation for non‑ASCII characters in situations where they might otherwise become garbled.

Key features
- 🔁 Encrypt and decrypt individual Unicode characters via `UniAz`.
- 🧩 Uses the normal latin alphabet in ascii(A–Z).
- ⚡ Lightweight: single-file API for easy integration and testing.

Installation
```bat
cargo add uniaz
```

Quick example

```rust
use uniaz::UniAz;

fn main() {
    let uni = UniAz::new(); // default uses "abcdefghijklmnopqrstuvwxyz"

    let encrypted = uni.encrypt(&'你');        // -> String (obfuscated)
    let decrypted = uni.decrypt(&encrypted);  // -> '你'

    println!("encrypted={}", encrypted);
    assert_eq!(decrypted, '你');
}
```

API (important)
- `UniAz::new()` — create an instance.
- `UniAz::encrypt(&char) -> String` — convert a char to an encrypted string.
- `UniAz::decrypt(&str) -> char` — recover the original char from an encrypted string.

Docs & tests
- Generate and open the API docs:

```bat
cargo doc --open
```

License
- MIT OR Apache-2.0.

Enjoy! 🚀

