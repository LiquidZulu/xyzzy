#!/usr/bin/env bash
#I run this with `alias ttv="chmod +x ~/Documents/xyzzy/scripts/ttv.sh && ~/Documents/xyzzy/scripts/ttv.sh"`

read -p "Enter vod ID: " id
mkdir -p ~/Videos/ttv/$id/
TwitchDownloaderCLI -m VideoDownload --id $id --ffmpeg-path "/usr/bin/ffmpeg" -o "$HOME/Videos/ttv/$id/$id.mp4"
TwitchDownloaderCLI -m ChatDownload --id $id -o "$HOME/Videos/ttv/$id/$id.json"
TwitchDownloaderCLI -m ChatRender -i "$HOME/Videos/ttv/$id/$id.json" -h 1080 -w 422 --framerate 30 --update-rate 0 --font-size 18 -o "$HOME/Videos/ttv/$id/$id.chat.mp4"
