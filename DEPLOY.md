# Cloudflare Workersデプロイ手順

## 前提条件

- Node.js（v18以上）
- Rust（最新版）
- Cloudflareアカウント

## 1. Wrangler CLIのインストール

```bash
npm install -g wrangler
```

## 2. Cloudflareにログイン

```bash
wrangler login
```

ブラウザが開くので、Cloudflareアカウントで認証してください。

## 3. D1データベースの作成

```bash
# 開発環境用
wrangler d1 create seikin-db

# 本番環境用
wrangler d1 create seikin-db-production
```

実行後、`database_id` が表示されるので、`wrangler.toml` の該当箇所に貼り付けてください：
```toml
database_id = "ここに貼り付け"
```

### D1マイグレーション実行

マイグレーションSQLファイルを作成後：

```bash
# 開発環境
wrangler d1 execute seikin-db --file=./migrations/schema.sql

# 本番環境
wrangler d1 execute seikin-db-production --file=./migrations/schema.sql
```

## 4. KVネームスペースの作成

```bash
# 開発環境用
wrangler kv:namespace create "seikin-kv"

# 本番環境用
wrangler kv:namespace create "seikin-kv" --env production
```

実行後、`id` が表示されるので、`wrangler.toml` の該当箇所に貼り付けてください：
```toml
id = "ここに貼り付け"
```

## 5. シークレット（機密情報）の設定

アプリケーションで必要なシークレットを設定：

```bash
# 開発環境
wrangler secret put API_KEY
wrangler secret put JWT_SECRET

# 本番環境
wrangler secret put API_KEY --env production
wrangler secret put JWT_SECRET --env production
```

コマンド実行後、プロンプトが表示されるので値を入力してください。

## 6. ビルドとデプロイ

### 開発環境へのデプロイ

```bash
cd apiroute
wrangler deploy
```

### 本番環境へのデプロイ

```bash
cd apiroute
wrangler deploy --env production
```

## 7. デプロイ確認

デプロイ後、URLが表示されます：
```
https://seikin-backend.your-subdomain.workers.dev
```

ヘルスチェック：
```bash
curl https://seikin-backend.your-subdomain.workers.dev/health
```

## ローカル開発

Cloudflare Workers環境をローカルでテスト：

```bash
cd apiroute
wrangler dev
```

これにより `http://localhost:8787` でローカルサーバーが起動します。

## トラブルシューティング

### ビルドエラーが出る場合

```bash
# worker-buildを手動でインストール
cargo install worker-build

# キャッシュをクリア
rm -rf build/
wrangler deploy
```

### D1やKVにアクセスできない場合

`wrangler.toml` の `database_id` と `id` が正しく設定されているか確認してください。

### ログの確認

```bash
wrangler tail
```

リアルタイムでWorkerのログを確認できます。

## 環境変数の管理

### 非機密情報（wrangler.toml）

```toml
[vars]
LOG_LEVEL = "INFO"
APP_VERSION = "1.0.0"
```

### 機密情報（wrangler secret）

```bash
wrangler secret put SECRET_NAME
```

## コードから環境変数にアクセス

```rust
#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    // 環境変数の取得
    let log_level = env.var("LOG_LEVEL")?.to_string();

    // シークレットの取得
    let api_key = env.secret("API_KEY")?.to_string();

    // D1データベースへのアクセス
    let db = env.d1("DB")?;

    // KVストアへのアクセス
    let kv = env.kv("KV")?;

    // ...
}
```

## リソース

- [Cloudflare Workers Docs](https://developers.cloudflare.com/workers/)
- [worker-rs GitHub](https://github.com/cloudflare/workers-rs)
- [D1 Documentation](https://developers.cloudflare.com/d1/)
- [Workers KV Documentation](https://developers.cloudflare.com/kv/)
