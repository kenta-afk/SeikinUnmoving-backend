# デプロイメントガイド

## サインアップが動作しない問題の修正

### 実施した変更

#### 1. バックエンド（Cloudflare Workers）のCORS設定追加
- `backend/api/src/lib.rs`: レスポンスにCORSヘッダーを追加
- `backend/api/src/routes/mod.rs`: OPTIONSリクエスト（CORSプリフライト）のハンドラーを追加

#### 2. フロントエンドのAPI URL設定を環境変数化
- `frontend/services/api.ts`: ハードコーディングされたlocalhostを環境変数 `EXPO_PUBLIC_API_URL` に変更

#### 3. GitHub Actionsワークフローの更新
- `.github/workflows/frontend-cd.yml`: ビルド時に環境変数を設定

#### 4. wrangler.tomlの設定修正
- `backend/api/wrangler.toml`: 本番環境のD1データベース設定を追加

### 次のステップ

#### 1. GitHub Secretsの設定
GitHubリポジトリの Settings → Secrets and variables → Actions で以下を設定してください：

```
API_URL: https://seikin-backend-production.{あなたのアカウント}.workers.dev
```

実際のWorkerのURLは以下のコマンドで確認できます：
```bash
cd backend/api
npx wrangler deployments list --env production
```

または、Cloudflareダッシュボードの Workers & Pages セクションから確認してください。

#### 2. バックエンドのデプロイ

```bash
cd backend/api
npx wrangler deploy --env production
```

#### 3. フロントエンドのデプロイ

変更をコミット＆プッシュすると自動的にデプロイされます：

```bash
git add .
git commit -m "fix: CORS設定とAPI URL環境変数の追加"
git push
```

#### 4. 動作確認

1. Cloudflare PagesのURLにアクセス（例: `https://seikin-frontend.pages.dev`）
2. サインアップ画面でユーザー登録を試行
3. ブラウザの開発者ツールでネットワークタブとコンソールを確認

### トラブルシューティング

#### CORSエラーが出る場合
- バックエンドが正しくデプロイされているか確認
- Workersのログを確認: `npx wrangler tail --env production`

#### API URLが正しく設定されていない場合
- GitHub Secretsの `API_URL` が正しく設定されているか確認
- フロントエンドを再デプロイ

#### D1データベースエラー
- D1データベースのマイグレーションが実行されているか確認:
```bash
cd backend/api
npx wrangler d1 execute seikin-db --env production --file=../../db/migrations/20241115000001_create_users_table.up.sql
npx wrangler d1 execute seikin-db --env production --file=../../db/migrations/20241118000001_create_clients_table.up.sql
```

### 重要なURL

- **Cloudflare Workers Dashboard**: https://dash.cloudflare.com/
- **Cloudflare Pages Dashboard**: https://dash.cloudflare.com/pages
- **GitHub Actions**: https://github.com/YOUR_USERNAME/seikin-backend/actions

### ローカル開発環境

ローカルで開発する場合は、環境変数を設定する必要はありません（デフォルトで `http://localhost:8080` を使用）。

```bash
# バックエンド
cd backend/presentation
cargo run

# フロントエンド
cd frontend
npm start
```
