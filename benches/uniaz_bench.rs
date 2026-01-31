use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use uniaz::UniAz;

/// Benchmark for single character encryption
fn bench_encrypt_char(c: &mut Criterion) {
    let uni_az = UniAz::new();

    let mut group = c.benchmark_group("encrypt_char");

    // Test different types of characters
    let test_chars = vec![
        ('A', "ASCII uppercase"),
        ('a', "ASCII lowercase"),
        ('0', "ASCII digit"),
        ('你', "Chinese character"),
        ('😀', "Emoji"),
        ('€', "Special symbol"),
    ];

    for (ch, desc) in test_chars {
        group.bench_with_input(BenchmarkId::from_parameter(desc), &ch, |b, &ch| {
            b.iter(|| uni_az.encrypt(black_box(ch)));
        });
    }

    group.finish();
}

/// Benchmark for single character decryption
fn bench_decrypt_char(c: &mut Criterion) {
    let uni_az = UniAz::new();

    let mut group = c.benchmark_group("decrypt_char");

    // Pre-encrypt test characters
    let test_chars = vec![
        ('A', "ASCII uppercase"),
        ('a', "ASCII lowercase"),
        ('0', "ASCII digit"),
        ('你', "Chinese character"),
        ('😀', "Emoji"),
        ('€', "Special symbol"),
    ];

    for (ch, desc) in test_chars {
        let encrypted = uni_az.encrypt(ch);
        group.bench_with_input(
            BenchmarkId::from_parameter(desc),
            &encrypted,
            |b, encrypted| {
                b.iter(|| uni_az.decrypt(black_box(encrypted)).unwrap());
            },
        );
    }

    group.finish();
}

/// Benchmark for string encryption with various lengths
fn bench_encrypt_str(c: &mut Criterion) {
    let uni_az = UniAz::new();

    let mut group = c.benchmark_group("encrypt_str");

    let test_strings = vec![
        ("Hello", "Short ASCII"),
        ("Hello, World!", "Medium ASCII"),
        ("The quick brown fox jumps over the lazy dog", "Long ASCII"),
        ("你好", "Short Chinese"),
        ("你好世界，欢迎使用UniAZ！", "Medium Chinese"),
        ("😀😁😂🤣😃😄😅😆😉😊", "Emoji sequence"),
        ("Mixed 混合 text 文本 123 😀", "Mixed content"),
    ];

    for (text, desc) in test_strings {
        group.bench_with_input(BenchmarkId::from_parameter(desc), &text, |b, &text| {
            b.iter(|| uni_az.encrypt_str(black_box(text)));
        });
    }

    group.finish();
}

/// Benchmark for string decryption with various lengths
fn bench_decrypt_str(c: &mut Criterion) {
    let uni_az = UniAz::new();

    let mut group = c.benchmark_group("decrypt_str");

    let test_strings = vec![
        ("Hello", "Short ASCII"),
        ("Hello, World!", "Medium ASCII"),
        ("The quick brown fox jumps over the lazy dog", "Long ASCII"),
        ("你好", "Short Chinese"),
        ("你好世界，欢迎使用UniAZ！", "Medium Chinese"),
        ("😀😁😂🤣😃😄😅😆😉😊", "Emoji sequence"),
        ("Mixed 混合 text 文本 123 😀", "Mixed content"),
    ];

    for (text, desc) in test_strings {
        let encrypted = uni_az.encrypt_str(text);
        group.bench_with_input(
            BenchmarkId::from_parameter(desc),
            &encrypted,
            |b, encrypted| {
                b.iter(|| uni_az.decrypt_str(black_box(encrypted)).unwrap());
            },
        );
    }

    group.finish();
}

/// Benchmark for round-trip (encrypt + decrypt) operations
fn bench_roundtrip(c: &mut Criterion) {
    let uni_az = UniAz::new();

    let mut group = c.benchmark_group("roundtrip");

    // Test character roundtrip
    group.bench_function("char_roundtrip", |b| {
        b.iter(|| {
            let ch = black_box('你');
            let encrypted = uni_az.encrypt(ch);
            uni_az.decrypt(&encrypted).unwrap()
        });
    });

    // Test string roundtrip
    group.bench_function("string_roundtrip", |b| {
        b.iter(|| {
            let text = black_box("你好世界");
            let encrypted = uni_az.encrypt_str(text);
            uni_az.decrypt_str(&encrypted).unwrap()
        });
    });

    group.finish();
}

/// Benchmark for UniAz initialization
fn bench_initialization(c: &mut Criterion) {
    c.bench_function("UniAz::new", |b| {
        b.iter(|| UniAz::new());
    });
}

criterion_group!(
    benches,
    bench_initialization,
    bench_encrypt_char,
    bench_decrypt_char,
    bench_encrypt_str,
    bench_decrypt_str,
    bench_roundtrip
);
criterion_main!(benches);
