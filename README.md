# Learning Chinese

中国語学習を支援するアプリケーション

Anthropic Messages API を用いて例文を生成することができます。

## 準備

実行するには Anthropic の API キーが必要です。

``.env`` :


```text
ANTHROPIC_API_KEY=...
```

```bash
direnv allow
```

## 実行

### 例文を生成する

```bash
# 指定した単語を用いた例文を３つ生成する
cargo run -- generate "如果"
```

### gRPC サーバを起動する

```bash
cargo run -- serve

# 別のセッションで実行する
grpcurl -plaintext \
    -import-path ./proto \
    -proto sentence.proto \
    -d '{"word": "但是"}' \
    "[::]:50051" sentence.SentenceService/Generate
```
