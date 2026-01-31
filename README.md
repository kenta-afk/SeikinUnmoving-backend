# Seikin Monorepo

このリポジトリはRust製バックエンドとReact Native (Expo)製フロントエンドのモノレポです。

**主要機能**: MediaPipe Face Meshを使った顔検出による「笑わないゲーム」

## プロジェクト構成

```
.
├── backend/          # Rustバックエンド (API, サービス層)
│   ├── presentation/# メインAPIサーバー (Axum)
│   ├── services/    # ドメインサービス
│   │   ├── userservice/    # ユーザー管理
│   │   ├── gameservice/    # ゲーム進行・判定
│   │   └── videoservice/   # 動画管理
│   └── db/          # データベースマイグレーション
└── frontend/        # React Native (Expo) フロントエンド
```

## 顔検出ゲームについて

### 技術仕様
- **顔検出**: MediaPipe Face Mesh (Web版)
- **笑顔判定**: 顔のランドマーク座標から口角の角度を計算
- **ゲーム時間**: 3分 (180秒)
- **検出間隔**: リアルタイム (約30fps)

## Backend (Rust)

### Prerequisites
- Docker
- Docker Compose

### Running with Docker

`backend/` ディレクトリで以下のコマンドを実行:

**すべてのサービスを起動:**
```bash
cd backend
docker compose up -d
```

**ログを確認:**
```bash
docker compose logs -f apiroute
```

**サービスを停止:**
```bash
docker compose down
```


### Services
- **apiroute** (API Server): http://localhost:8080
- **valkey** (Redis互換): localhost:6379

### Database
- SQLite を使用
- データはDockerボリュームに永続化 (`sqlite_data`)
- マイグレーションファイル: `backend/db/migrations/`

### gameservice
- ゲームセッションの開始・進行・終了API
- 顔の動き（FacePosition）をもとに「動いたら負け」判定
- ゲーム結果の保存・取得

### videoservice
- ゲームで使うYouTube動画の管理API
- 動画の追加・一覧取得・ランダム取得
- 動画情報（URL, タイトル, 長さ, 有効/無効）を管理

### API Documentation
サーバー起動後、以下のURLでAPI仕様を確認できます:
- Swagger UI: http://localhost:8080/swagger-ui/
- OpenAPI JSON: http://localhost:8080/api-docs/openapi.json

## Frontend (React Native / Expo)

### Prerequisites
- Node.js (v18以上推奨)
- npm

### Setup

```bash
cd frontend
npm install --legacy-peer-deps
```

### Running

#### Web (開発用)
```bash
npm run web
```
ブラウザで http://localhost:8081 が開きます

#### iOS (Mac only)
```bash
npm run ios
```

#### Android
```bash
npm run android
```

### 主な機能
- **ユーザー認証**: サインアップ/サインイン
- **顔認識ゲーム**: MediaPipe Face Meshを使用したリアルタイム顔検出
- **ランキング**: ゲーム結果の記録と表示

### 環境設定
`.env` ファイルで API エンドポイントを設定:
```env
EXPO_PUBLIC_API_URL=http://localhost:8080
```

## Setup Instructions

リポジトリをクローンした後:

1. **バックエンドを起動:**
```bash
cd backend
docker compose up -d
```

2. **フロントエンドを起動:**
```bash
cd frontend
npm install --legacy-peer-deps
npm run web
```

3. ブラウザで http://localhost:8081 にアクセス

### 開発時のTips
- フロントエンドのコード変更は自動でリロードされます
- バックエンドのログは `docker compose logs -f apiroute` で確認できます
- APIの型定義を更新する場合: `cd frontend && npm run generate-types`