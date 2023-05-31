# 開発

## 必要なソフトウェア

- asdf
- rustup
- Docker

## asdf で管理されている処理系・開発ツールをインストールする

- deno
- pnpm

```bash
$ asdf install
```

## 依存 npm パッケージをインストールする

```bash
$ pnpm i
```

## GraphQL スキーマをもとに TypeScript 型定義を生成する

先に依存 npm パッケージをインストールしておいてください。

`/resolvers-types.d.ts` が生成されます。

```bash
$ pnpm exec graphql-codegen
```

## BFF (Backend For Frontend) の起動

先に GraphQL スキーマから TypeScript 型定義 ( `/resolvers-types.d.ts` ) を生成しておいてください。

```bash
$ cd bff
$ deno run --allow-read --allow-net main.ts
```

<!--
## テスト用 SMTP サーバと Redis の起動

```bash
$ docker compose up
```

## アプリケーションサーバの起動

```bash
$ RUST_LOG=debug cargo run
```
-->
