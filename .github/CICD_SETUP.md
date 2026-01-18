# CI/CD セットアップガイド

このプロジェクトではGitHub Actionsを使用してCI/CDを行います。

## 📁 ワークフローファイル

- **ci.yml**: プルリクエストとmainブランチへのプッシュ時にCI実行（Lint、テスト）
- **backend-cd.yml**: バックエンドのCloudflare Workersへのデプロイ
- **frontend-cd.yml**: フロントエンドのEAS BuildとCloudflare Pagesへのデプロイ

## 🔐 必要なシークレット設定

GitHub Repositoryの Settings > Secrets and variables > Actions で以下のシークレットを設定してください：

### バックエンド用
- `CLOUDFLARE_API_TOKEN`: Cloudflare API Token
  - Cloudflareダッシュボード > My Profile > API Tokens から生成
  - Permissions: `Account.Cloudflare Workers Scripts:Edit`

### フロントエンド用
- `EXPO_TOKEN`: Expo Access Token
  - https://expo.dev/accounts/[account]/settings/access-tokens から生成
- `CLOUDFLARE_API_TOKEN`: 上記と同じ（Web版デプロイ用）
- `CLOUDFLARE_ACCOUNT_ID`: CloudflareアカウントID
  - Cloudflareダッシュボードの右側に表示されています

## 🚀 バックエンドデプロイ

### 自動デプロイ
`backend/`配下のファイルをmainブランチにプッシュすると自動的にデプロイされます。

### 手動デプロイ
```bash
cd backend/api
wrangler deploy --env production
```

### 初回セットアップ
```bash
# Cloudflare Workers用のAPIトークンでログイン
cd backend/api
wrangler login

# シークレットの設定（必要に応じて）
wrangler secret put JWT_SECRET
wrangler secret put DATABASE_URL
```

## 📱 フロントエンドデプロイ

### EAS Build（iOS/Android）

#### 初回セットアップ
```bash
cd frontend

# EASアカウントにログイン
npx eas login

# EASプロジェクトを初期化
npx eas build:configure

# eas.jsonで設定を確認・編集
# - iOS: Apple Developer情報を設定
# - Android: Keystoreを設定
```

#### ビルド実行
```bash
# 開発ビルド
npx eas build --platform ios --profile development
npx eas build --platform android --profile development

# プレビュービルド
npx eas build --platform ios --profile preview
npx eas build --platform android --profile preview

# 本番ビルド（自動デプロイ時も実行される）
npx eas build --platform ios --profile production
npx eas build --platform android --profile production
```

#### ストアへの提出
```bash
# iOS App Store
npx eas submit --platform ios

# Google Play Store
npx eas submit --platform android
```

### Web版デプロイ（Cloudflare Pages）

`frontend/`配下のファイルをmainブランチにプッシュすると、自動的にWebビルドが作成され、Cloudflare Pagesにデプロイされます。

#### 初回セットアップ
1. Cloudflareダッシュボードで新しいPagesプロジェクトを作成
2. プロジェクト名を `seikin-frontend` に設定
3. GitHubリポジトリには接続せず、Direct Upload方式を選択

## 🔄 CI実行タイミング

### CI（ci.yml）
- すべてのプルリクエスト
- mainブランチへのプッシュ

### Backend CD（backend-cd.yml）
- `backend/`配下のファイルがmainブランチにプッシュされた時のみ

### Frontend CD（frontend-cd.yml）
- `frontend/`配下のファイルがmainブランチにプッシュされた時のみ

## 🛠️ トラブルシューティング

### バックエンドデプロイが失敗する場合
- `CLOUDFLARE_API_TOKEN`が正しく設定されているか確認
- `backend/api/wrangler.toml`の設定を確認
- ローカルで `wrangler deploy --dry-run` を実行して問題を特定

### フロントエンドビルドが失敗する場合
- `EXPO_TOKEN`が正しく設定されているか確認
- `frontend/eas.json`の設定（Apple ID、Team IDなど）を確認
- ローカルで `npx eas build --platform ios --profile preview` を実行して問題を特定

### CIが失敗する場合
- ローカルで `cargo fmt --check` と `cargo clippy` を実行
- フロントエンドで `npm run lint` と `npx tsc --noEmit` を実行

## 📝 eas.json の編集が必要な項目

[frontend/eas.json](../frontend/eas.json) の以下の項目を実際の値に置き換えてください：

```json
{
  "submit": {
    "production": {
      "ios": {
        "appleId": "your-apple-id@example.com",  // ← 実際のApple IDに変更
        "ascAppId": "1234567890",                // ← App Store Connectのアプリ IDに変更
        "appleTeamId": "ABCDE12345"              // ← Apple Developer Team IDに変更
      },
      "android": {
        "serviceAccountKeyPath": "./service-account-key.json"  // ← Google Playのサービスアカウントキーのパス
      }
    }
  }
}
```

## 🌐 デプロイ先URL

- **バックエンド（Cloudflare Workers）**: `https://seikin-backend-production.workers.dev`
- **フロントエンド（Cloudflare Pages）**: `https://seikin-frontend.pages.dev`
- **iOS App**: App Store（EAS Submitで提出後）
- **Android App**: Google Play Store（EAS Submitで提出後）
