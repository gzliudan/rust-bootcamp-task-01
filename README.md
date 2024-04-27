# Rust cli tool

## 作业一

阅读 chacha20poly1305 文档，了解其使用方法并构建 CLI 对输入文本进行加密/解密

```bash
# 加密并输出 base64
rcli text encrypt -key "xxx"

# base64 > binary> 解密文本
rcli text decrypt -key "XXX"
```

## 作业二

json web token(jwt) 在用户验证领域经常被用到。请构建一个 CLI 来为给定 sub/aud/exp/… 生成一个 jwt。要求生成的 jwt 可以通过 jwt.io 的验证。

```bash
rcli jwt sign --sub acme --aud device1 --exp 14d
rcli jwt verify -t
```
