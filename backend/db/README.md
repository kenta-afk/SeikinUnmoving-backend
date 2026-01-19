# Database Migrations

このディレクトリはSQLiteデータベースのマイグレーション管理用です。
開発環境ではSQLite、本番環境ではCloudflare D1を使用します。

## 構成

```
db/
├── migrations/
│   ├── 20241115000001_create_users_table.up.sql    # マイグレーション実行
│   └── 20241115000001_create_users_table.down.sql  # ロールバック
└── README.md
```

## 開発環境での使用方法

### Docker Composeで自動実行

```bash
cd /Users/doikentarou/seikin-backend
docker-compose up
```

起動時に自動的にマイグレーションが実行されます。

### 手動でマイグレーション実行

```bash
# sqlx-cliのインストール（初回のみ）
cargo install sqlx-cli --no-default-features --features sqlite

# マイグレーション実行
export DATABASE_URL="sqlite:///data/seikin.db"
sqlx migrate run

# マイグレーション状態の確認
sqlx migrate info

# ロールバック（最後のマイグレーションを取り消す）
sqlx migrate revert
```

## 新しいマイグレーションの作成

```bash
# 新しいマイグレーションファイルを作成
sqlx migrate add <migration_name>

# 例: gamesテーブルを作成
sqlx migrate add create_games_table
```

これにより以下のファイルが生成されます：
- `migrations/<timestamp>_create_games_table.up.sql` - マイグレーション
- `migrations/<timestamp>_create_games_table.down.sql` - ロールバック

## Cloudflare D1へのデプロイ

本番環境（Cloudflare Workers + D1）では、`wrangler`コマンドを使用します。

### D1データベースの作成

```bash
cd api
wrangler d1 create seikin-db
```

生成された`database_id`を`api/wrangler.toml`に追加：

```toml
[[d1_databases]]
binding = "DB"
database_name = "seikin-db"
database_id = "your-database-id"  # ここに設定
```

### マイグレーションの実行

```bash
# ローカルD1にマイグレーション実行
wrangler d1 execute seikin-db --local --file=../db/migrations/20241115000001_create_users_table.up.sql

# 本番D1にマイグレーション実行
wrangler d1 execute seikin-db --remote --file=../db/migrations/20241115000001_create_users_table.up.sql
```

### 複数のマイグレーションを一括実行

```bash
# すべてのマイグレーションを本番環境に適用
for file in ../db/migrations/*.up.sql; do
  wrangler d1 execute seikin-db --remote --file="$file"
done
```

## データベーススキーマ

### users テーブル

| カラム名 | 型 | 制約 | 説明 |
|---------|---|------|------|
| id | TEXT | PRIMARY KEY | UUID v7 |
| username | TEXT | NOT NULL, UNIQUE | ユーザー名 |
| email | TEXT | NOT NULL, UNIQUE | メールアドレス |
| password | TEXT | NOT NULL | ハッシュ化されたパスワード |
| created_at | TEXT | NOT NULL | 作成日時（ISO8601） |
| updated_at | TEXT | NOT NULL | 更新日時（ISO8601） |

インデックス:
- `idx_users_email` on `email`
- `idx_users_username` on `username`

## トラブルシューティング

### マイグレーションが失敗する

```bash
# マイグレーション履歴をリセット（開発環境のみ）
rm /data/seikin.db
docker-compose down -v
docker-compose up --build
```

### SQLiteとD1の互換性

SQLiteとCloudflare D1はほぼ互換性がありますが、以下の点に注意：

- 日付型は`TEXT`として`ISO8601`形式で保存
- UUIDは`TEXT`として保存
- `AUTOINCREMENT`の代わりに手動でUUID生成を推奨

## 参考資料

- [SQLx CLI Documentation](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli)
- [Cloudflare D1 Documentation](https://developers.cloudflare.com/d1/)
- [SQLite Documentation](https://www.sqlite.org/docs.html)
