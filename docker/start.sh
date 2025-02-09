#!/bin/bash

port="${PORT:-8080}"
directory="${DIRECTORY:-/data/}"

npps4="${NPPS4_ADDRESS:-http://127.0.0.1:51376}"

https=$([ "$HTTPS" = "true" ] && echo "--https" || echo "")

/root/ew/ew --path $directory --port $port --npps4 $npps4 $https --global-android "$ANDROID_GLOBAL"  --japan-android "$ANDROID_JAPAN"  --global-ios "$IOS_GLOBAL"  --japan-ios "$IOS_JAPAN" --assets-url "$ASSET_URL"
