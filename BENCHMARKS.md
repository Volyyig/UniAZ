# 基准测试快速指南

## 运行基准测试

```bash
# 运行所有基准测试
cargo bench

# 运行特定测试
cargo bench -- encrypt_char
cargo bench -- decrypt_str
cargo bench -- roundtrip
```

## 查看结果

基准测试完成后，可以在以下位置查看详细的 HTML 报告：
```
target/criterion/report/index.html
```

## 测试覆盖

✅ 单字符加密/解密（ASCII、中文、Emoji）
✅ 字符串加密/解密（不同长度）
✅ 往返操作
✅ 初始化性能

详细文档请查看 `benches/README.md`
