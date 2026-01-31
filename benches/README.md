# UniAZ 基准测试

本文档介绍如何运行和理解 UniAZ 项目的性能基准测试。

## 运行基准测试

### 运行所有基准测试

```bash
cargo bench
```

### 运行特定的基准测试

```bash
# 只运行单字符加密基准测试
cargo bench -- encrypt_char

# 只运行字符串解密基准测试
cargo bench -- decrypt_str

# 只运行往返（加密+解密）基准测试
cargo bench -- roundtrip
```

## 基准测试概览

基准测试套件包含以下测试组：

### 1. 初始化测试 (`bench_initialization`)
- 测试 `UniAz::new()` 的性能
- 衡量创建新实例的开销

### 2. 单字符加密 (`bench_encrypt_char`)
测试不同类型字符的加密性能：
- ASCII 大写字母 (A)
- ASCII 小写字母 (a)
- ASCII 数字 (0)
- 中文字符 (你)
- Emoji (😀)
- 特殊符号 (€)

### 3. 单字符解密 (`bench_decrypt_char`)
测试不同类型字符的解密性能（与加密测试相同的字符集）

### 4. 字符串加密 (`bench_encrypt_str`)
测试不同长度和类型的字符串加密：
- 短 ASCII 文本 ("Hello")
- 中等 ASCII 文本 ("Hello, World!")
- 长 ASCII 文本 ("The quick brown fox...")
- 短中文文本 ("你好")
- 中等中文文本 ("你好世界，欢迎使用UniAZ！")
- Emoji 序列 ("😀😁😂🤣😃😄😅😆😉😊")
- 混合内容 ("Mixed 混合 text 文本 123 😀")

### 5. 字符串解密 (`bench_decrypt_str`)
测试不同长度和类型的字符串解密（与加密测试相同的字符串）

### 6. 往返操作 (`bench_roundtrip`)
- 单字符往返（加密 + 解密）
- 字符串往返（加密 + 解密）

## 理解基准测试结果

Criterion 会输出详细的统计信息：

```
encrypt_char/Chinese character
                        time:   [3.5000 µs 3.5150 µs 3.5300 µs]
```

- **time**: 表示操作的平均执行时间
- 三个值分别是：下界、估计值、上界（95% 置信区间）
- **µs**: 微秒（1 µs = 0.001 毫秒）

### 典型性能指标

您可以期望看到类似的性能范围：
- 单字符加密：约 3-5 µs
- 单字符解密：约 4-6 µs
- 字符串加密（中等长度）：约 20-50 µs
- 字符串解密（中等长度）：约 20-60 µs

## 基准测试报告

Criterion 会在 `target/criterion` 目录下生成详细的 HTML 报告，包括：
- 详细的统计数据
- 性能图表
- 历史趋势（用于跟踪性能回归）

要查看报告，在浏览器中打开：
```
target/criterion/report/index.html
```

## 性能优化建议

如果您在优化代码性能，可以：

1. **运行基准测试建立基线**
   ```bash
   cargo bench
   ```

2. **进行代码更改**

3. **再次运行基准测试进行比较**
   ```bash
   cargo bench
   ```

Criterion 会自动比较新旧结果并显示性能变化。

## 自定义基准测试

要添加新的基准测试，编辑 `benches/uniaz_bench.rs` 文件：

```rust
fn my_custom_bench(c: &mut Criterion) {
    let uni_az = UniAz::new();
    
    c.bench_function("my_test", |b| {
        b.iter(|| {
            // 您的测试代码
        });
    });
}

// 然后将其添加到 criterion_group!
criterion_group!(
    benches,
    // ... 其他基准测试 ...
    my_custom_bench
);
```

## 更多信息

- [Criterion.rs 官方文档](https://bheisler.github.io/criterion.rs/book/)
- [Rust 性能测试指南](https://doc.rust-lang.org/book/ch11-00-testing.html)
