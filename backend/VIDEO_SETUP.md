# セイキン動画データベース管理ガイド

## 動画を追加する方法

### 方法1: シェルスクリプトを使う（簡単）

1. `backend/scripts/add-videos.sh` を開く
2. `VIDEOS=()` 配列に動画URLを追加:

```bash
VIDEOS=(
    "https://www.youtube.com/watch?v=実際のYouTube動画ID|セイキン面白動画1|180"
    "https://www.youtube.com/watch?v=別の動画ID|セイキン爆笑動画|180"
    # 動画を追加する場合はここに書く
)
```

3. スクリプトを実行:

```bash
cd backend
./scripts/add-videos.sh
```

### 方法2: curlコマンドを使う（API経由）

サーバーを起動してから:

```bash
curl -X POST http://localhost:8080/api/videos \
  -H "Content-Type: application/json" \
  -d '{
    "youtube_url": "https://www.youtube.com/watch?v=実際のYouTube動画ID",
    "title": "セイキン面白動画",
    "duration_seconds": 180
  }'
```

### 方法3: 直接SQLiteに追加

```bash
cd backend
sqlite3 db/data/seikin.db

INSERT INTO videos (id, youtube_url, title, duration_seconds, is_active, created_at)
VALUES (
    lower(hex(randomblob(16))),
    'https://www.youtube.com/watch?v=YOUR_VIDEO_ID',
    'セイキン面白動画',
    180,
    1,
    datetime('now')
);
```

## 動画一覧を確認

```bash
cd backend
sqlite3 db/data/seikin.db "SELECT title, youtube_url FROM videos WHERE is_active = 1;"
```

## YouTubeショート動画のURLについて

- 通常の動画: `https://www.youtube.com/watch?v=VIDEO_ID`
- ショート動画: `https://www.youtube.com/shorts/VIDEO_ID`

どちらの形式でもOKです！

## 注意事項

- **著作権**: セイキンさんの公式動画を使用してください
- **duration_seconds**: 180秒（3分）が標準
- **is_active**: 1 = 有効、0 = 無効（ランダム選択から除外）

## APIエンドポイント

- `POST /api/videos` - 動画を追加
- `GET /api/videos` - 全動画を取得
- `GET /api/videos/random` - ランダムに1つの動画を取得（ゲーム用）

## 次のステップ

1. [add-videos.sh](scripts/add-videos.sh) に実際のYouTube URLを追加
2. スクリプトを実行
3. サーバーを起動してゲームをプレイ！
