# API - Cloudflare Workers

本番環境用のAPIサーバー。Cloudflare Workersで動作します。

## 構造

```
api/
├── src/
│   ├── lib.rs              # エントリーポイント
│   └── routes/
│       ├── mod.rs          # ルート管理
│       └── health.rs       # ヘルスチェック
├── Cargo.toml
├── wrangler.toml
└── README.md
```

## 新しいルートの追加方法

### 1. ルートファイルを作成

```rust
// src/routes/users.rs
use worker::*;

pub fn register(router: Router) -> Router {
    router
        .get("/api/users", get_users)
        .post("/api/users", create_user)
}

async fn get_users(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    Response::ok("Users list")
}

async fn create_user(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    Response::ok("User created")
}
```

### 2. mod.rs に追加

```rust
// src/routes/mod.rs
pub mod health;
pub mod users;  // 追加

use worker::Router;

pub fn register_routes(router: Router) -> Router {
    let router = health::register(router);
    let router = users::register(router);  // 追加
    router
}
```

## ローカル開発

```bash
cd api
wrangler dev
```

http://localhost:8787 でアクセス可能

## デプロイ

```bash
# 本番環境にデプロイ
wrangler deploy --env production

# または、mainブランチにpushすれば自動デプロイ
git push origin main
```

## エンドポイント

| メソッド | パス | 説明 |
|---------|------|------|
| GET | `/health` | ヘルスチェック |

## 環境変数

- `LOG_LEVEL`: ログレベル（INFO, DEBUG, ERROR）

## 参考資料

- [Cloudflare Workers Docs](https://developers.cloudflare.com/workers/)
- [worker-rs](https://github.com/cloudflare/workers-rs)
