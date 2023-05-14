# Restful API

## ユーザ管理

### 新規ユーザ登録

```text
POST /users
```

#### リクエスト (JSON オブジェクト)

| プロパティ名 | データ型 | 意味 |
| --- | --- | --- |
| `username` | `string` | ユーザ名 |
| `password` | `string` | パスワード |

#### レスポンス

| ステータスコード | 意味 |
| --- | --- |
| `201 Created` | ユーザ登録に成功した |

### ログイン

```text
POST /login
```

### ログアウト

```text
DELETE /login
```

## 単語

### 単語を追加する

```text
POST /words
```

### 単語の一覧を取得する

```text
GET /words
```

### 単語ひとつについての詳細を取得する

```text
GET /words/:id
```

### 単語の更新

```text
PATCH /words/:id
```

### 単語の削除

```text
DELETE /words/:id
```
