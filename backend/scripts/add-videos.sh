#!/bin/bash

# セイキンさんの動画をデータベースに追加するスクリプト
# 使い方: ./add-videos.sh

DB_PATH="/Users/doikentarou/seikin-backend/backend/db/data/seikin.db"

# 動画のYouTube URLをここに追加してください
# 形式: "youtube_url|title|duration_seconds"
VIDEOS=(
    "https://youtube.com/shorts/3viDm5oPh_s?si=xSUdHu9qWeBUpUDo|セイキン面白動画1|180"
    "https://youtube.com/shorts/6Bm3mj8EeGM?si=mybMXeVc--lyHn1U|セイキン面白動画2|180"
    "https://youtube.com/shorts/QGv3FBmADnU?si=aLrJsBV1moE7b5xg|セイキン面白動画3|180"
    # ここに動画を追加してください
    # "https://www.youtube.com/watch?v=実際のID|タイトル|秒数"
)

echo "セイキンさんの動画をデータベースに追加します..."
echo ""

for video in "${VIDEOS[@]}"; do
    IFS='|' read -r url title duration <<< "$video"
    
    # UUIDを生成
    id=$(uuidgen | tr '[:upper:]' '[:lower:]')
    
    # 現在時刻を取得（ISO8601形式）
    created_at=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
    
    echo "追加中: $title"
    echo "  URL: $url"
    
    # SQLite にデータを挿入
    sqlite3 "$DB_PATH" <<EOF
INSERT INTO videos (id, youtube_url, title, duration_seconds, is_active, created_at)
VALUES ('$id', '$url', '$title', $duration, 1, '$created_at');
EOF
    
    if [ $? -eq 0 ]; then
        echo "  ✓ 追加成功"
    else
        echo "  ✗ 追加失敗"
    fi
    echo ""
done

echo "完了！"
echo ""
echo "追加された動画を確認:"
sqlite3 "$DB_PATH" "SELECT title, youtube_url FROM videos WHERE is_active = 1;"
