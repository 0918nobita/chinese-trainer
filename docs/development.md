# 開発

## 必要なソフトウェア

- `rustup` で Rust コンパイラと `cargo` をインストールしておく
- Docker を導入し `docker compose` コマンドが使えるようにしておく

## テスト用 SMTP サーバの起動

```bash
$ docker compose up
```

## アプリケーションサーバの起動

```bash
$ RUST_LOG=debug cargo run
```
