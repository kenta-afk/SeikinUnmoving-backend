#!/bin/bash

# Docker環境のvideosテーブルに動画を追加するスクリプト

CONTAINER_NAME="seikin-apiroute"
API_URL="http://localhost:8080"

# 動画のYouTube URLをここに追加してください
# 形式: "youtube_url|title|duration_seconds"
VIDEOS=(
    "https://youtube.com/shorts/3viDm5oPh_s?si=xSUdHu9qWeBUpUDo|セイキン面白動画1|180"
    "https://youtube.com/shorts/6Bm3mj8EeGM?si=mybMXeVc--lyHn1U|セイキン面白動画2|180"
    "https://youtube.com/shorts/QGv3FBmADnU?si=aLrJsBV1moE7b5xg|セイキン面白動画3|180"
)

echo "Docker環境のセイキンさんの動画をAPIで追加します..."
echo ""

for video in "${VIDEOS[@]}"; do
    IFS='|' read -r url title duration <<< "$video"
    
    echo "追加中: $title"
    echo "  URL: $url"
    
    # APIで動画を追加
    response=$(curl -s -X POST "$API_URL/api/videos" \
        -H "Content-Type: application/json" \
        -d "{\"youtube_url\":\"$url\",\"title\":\"$title\",\"duration_seconds\":$duration}")
    
    if [ $? -eq 0 ]; then
        echo "  ✓ 追加成功"
    else
        echo "  ✗ 追加失敗: $response"
    fi
    echo ""
done

echo "完了！"
echo ""
echo "追加された動画を確認:"
curl -s "$API_URL/api/videos" | python3 -m json.tool
