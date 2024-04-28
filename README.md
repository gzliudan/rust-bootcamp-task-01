# Rust cli tool

## 作业一

阅读 chacha20poly1305 文档，了解其使用方法并构建 CLI 对输入文本进行加密/解密

### 创建密钥

```bash
cargo run -- text gen-key
```

输出：

```text
text key: 0x6BCF8B317F430090825D8ECB7F536E9C7EEAB55E489DF22778271D5CFA83B873
```

### 加密文本

输出base64

```bash
cargo run -- text encrypt --key 0x6BCF8B317F430090825D8ECB7F536E9C7EEAB55E489DF22778271D5CFA83B873 hello
```

输出：

```text
encrypt result: 7ZJvIUQaYDLgqCqb/XT/DQ78WYlPzSalpYP7k67apjLE
```

### 解密文本

输入base64

```bash
cargo run -- text decrypt --key 0x6BCF8B317F430090825D8ECB7F536E9C7EEAB55E489DF22778271D5CFA83B873 7ZJvIUQaYDLgqCqb/XT/DQ78WYlPzSalpYP7k67apjLE
```

输出：

```text
decrypt result: hello
```

## 作业二

json web token(jwt) 在用户验证领域经常被用到。请构建一个 CLI 来为给定 sub/aud/exp/… 生成一个 jwt。要求生成的 jwt 可以通过 jwt.io 的验证。

### 编码

```bash
cargo run -- jwt sign --aud aud --sub sub --exp 14d
```

输出：

```text
sign result: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJhdWQiOiJhdWQiLCJzdWIiOiJzdWIiLCJleHAiOjE3MTU1MDI2NzB9.LrOP7B0Cg6g7bLh-E2Qd0MjFT34Yqiy9OXSlCtuVwgE
```

### 解码

```bash
cargo run -- jwt verify eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJhdWQiOiJhdWQiLCJzdWIiOiJzdWIiLCJleHAiOjE3MTU1MDI2NzB9.LrOP7B0Cg6g7bLh-E2Qd0MjFT34Yqiy9OXSlCtuVwgE
```

输出：

```text
decoded token: TokenData { header: Header { typ: Some("JWT"), alg: HS256, cty: None, jku: None, jwk: None, kid: None, x5u: None, x5c: None, x5t: None, x5t_s256: None }, claims: Claims { aud: "aud", sub: "sub", exp: 1715502670 } }
verify result: true
```
