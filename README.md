# Learning Chinese

中国語学習を支援するアプリケーション

Anthropic Messages API を用いて例文を生成することができます。

実行するには Anthropic の API キーが必要です。

``.env`` :


```text
ANTHROPIC_API_KEY=...
```

```bash
direnv allow

# 指定した単語を用いた例文を３つ生成する
cargo run -- generate "如果"
```
