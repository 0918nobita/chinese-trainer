# Learning Chinese

中国語学習を支援するアプリケーション

Anthropic Messages API を用いて例文を生成することができます。

## 準備

- [Rust](https://rustup.rs/)
- [direnv](https://github.com/direnv/direnv)
- [asdf](https://asdf-vm.com/) or [mise](https://mise.jdx.dev/)
- [gRPC UI](https://github.com/fullstorydev/grpcui)
- Anthropic の API キー

``.env`` :


```text
ANTHROPIC_API_KEY=...
```

```bash
direnv allow
```

## 例文を生成する

```bash
# 指定した単語を用いた例文を３つ生成する
cargo run -- generate "如果"
```

## Next.js, gRPC サーバを起動する

```bash
makers serve
```

## gRPC Web UI を起動する

ブラウザ上で gRPC サーバの動作確認ができます。

```
makers grpcui
```
