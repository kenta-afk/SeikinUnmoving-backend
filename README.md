# Seikin Monorepo

このリポジトリはRust製バックエンドとReact Native (Expo)製フロントエンドのモノレポです。

## プロジェクト構成

```
.
├── backend/          # Rustバックエンド (API, サービス層)
│   ├── api/         # Cloudflare Workers用API
│   ├── presentation/# メインAPIサーバー
│   ├── services/    # ドメインサービス
│   └── db/          # データベースマイグレーション
└── frontend/        # React Native (Expo) フロントエンド
```

## Backend (Rust)

### Prerequisites
- Docker
- Docker Compose
- Rust (cargo)

### Running with Docker

`backend/` ディレクトリで以下のコマンドを実行:

Start all services (apiroute, Valkey 9.0):
```bash
cd backend
docker compose up -d
```

Stop all services:
```bash
docker compose down
```

View logs:
```bash
docker compose logs -f
```

### Services
- apiroute: http://localhost:8080
- Valkey: localhost:6379

### Database
This project uses SQLite as the database. The database file is stored in a Docker volume and will persist across container restarts.

## Frontend (React Native / Expo)

### Prerequisites
- Node.js (v18以上推奨)
- npm or yarn

### Setup

```bash
cd frontend
npm install
```

### Running

#### Web
```bash
npm run web
```

#### iOS (Mac only)
```bash
npm run ios
```

#### Android
```bash
npm run android
```

### Development
Expoの開発サーバーが起動し、Web、iOS、Androidで同じコードベースを実行できます。

## Setup Instructions

If you clone this repository, please command `$lefthook install` in the backend directory.