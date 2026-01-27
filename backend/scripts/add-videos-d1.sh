#!/bin/bash

# D1データベースに動画を追加するスクリプト
# 使い方: ./add-videos-d1.sh

DATABASE_ID="7024845a-2b00-4cb5-a0a3-87fc7dead454"

# 動画のYouTube URLをここに追加してください
# 形式: "youtube_url|title|duration_seconds"
VIDEOS=(
    "https://youtube.com/shorts/3viDm5oPh_s?si=xSUdHu9qWeBUpUDo|セイキン面白動画1|180"
    "https://youtube.com/shorts/6Bm3mj8EeGM?si=mybMXeVc--lyHn1U|セイキン面白動画2|180"
    "https://youtube.com/shorts/QGv3FBmADnU?si=aLrJsBV1moE7b5xg|セイキン面白動画3|180"
)

echo "セイキンさんの動画をD1データベースに追加します..."
echo "Database ID: $DATABASE_ID"
echo ""

for video in "${VIDEOS[@]}"; do
    IFS='|' read -r url title duration <<< "$video"
    
    # UUIDを生成（小文字）
    id=$(uuidgen | tr '[:upper:]' '[:lower:]')
    
    # 現在時刻を取得（ISO8601形式）
    created_at=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
    
    echo "追加中: $title"
    echo "  URL: $url"
    echo "  ID: $id"
    
    # D1にデータを挿入
    cd /Users/doikentarou/seikin-backend/backend/api
    npx wrangler d1 execute seikin-db --remote --command="INSERT INTO videos (id, youtube_url, title, duration_seconds, is_active, created_at) VALUES ('$id', '$url', '$title', $duration, 1, '$created_at');"
    
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
cd /Users/doikentarou/seikin-backend/backend/api
npx wrangler d1 execute seikin-db --remote --command="SELECT title, youtube_url FROM videos WHERE is_active = 1;"
