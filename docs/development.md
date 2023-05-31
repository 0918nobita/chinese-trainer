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

```bash
$ pnpm exec graphql-codegen
```

## BFF (Backend For Frontend) の起動

先に GraphQL スキーマから TypeScript 型定義を生成しておいてください。

```bash
$ cd bff
$ deno run --allow-read --allow-net main.ts
```

## フロントエンドの開発サーバの起動

```bash
$ pnpm exec vite
```

## フロントエンドのテスト

```bash
$ pnpm exec vitest
```

[Vitest UI](https://vitest.dev/guide/ui.html) を利用する場合：

```bash
$ pnpm exec vitest --ui
```

## BFF に関するコード生成

`/protos` ディレクトリ内の `.proto` (Protocol Buffers) ファイルをもとに、`/bff/generated` 以下に gRPC クライアント (Node.js) 用の TypeScript コードを生成します。

```bash
$ cd bff
$ ./codegen
```

## BFF の起動

先に BFF に関するコード生成を実行しておいてください。

```bash
$ cd bff
$ pnpm exec ts-node main.ts
```

## テスト用 SMTP サーバと Redis の起動

```bash
$ docker compose up
```

## アプリケーションサーバの起動

```bash
$ cd backend
$ RUST_LOG=debug cargo run
```
