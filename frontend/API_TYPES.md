# API型定義の自動生成

## 概要
バックエンドのOpenAPI仕様から、フロントエンドの型定義を自動生成します。

## セットアップ済み

### 1. インストール済みパッケージ
- `openapi-typescript`: OpenAPIから型定義を生成

### 2. npm スクリプト
```bash
npm run generate-types
```

このコマンドで `types/api.ts` が自動生成されます。

## 使い方

### 型定義の生成
バックエンドサーバーを起動してから:
```bash
npm run generate-types
```

### 型の使用
```typescript
import type { components } from '../types/api';

type User = components['schemas']['GetUserResponse'];
```

または、便利なエイリアスを使用:
```typescript
import type { User, SignInRequest } from '../types';
```

## メリット

1. **型安全性**: バックエンドのAPIと完全に同期
2. **自動更新**: API変更が即座に型エラーとして検知
3. **ドキュメント**: OpenAPIのコメントも型定義に含まれる
4. **開発効率**: 手動での型定義が不要

## ワークフロー

1. バックエンドでAPI変更
2. `npm run generate-types` を実行
3. 型エラーが発生したら修正
4. コミット

## 注意事項

- `types/api.ts` は自動生成ファイルなので直接編集しない
- バックエンドサーバーが起動している必要がある
- API変更後は必ず型生成を実行すること
