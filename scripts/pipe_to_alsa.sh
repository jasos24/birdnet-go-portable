#!/usr/bin/env bash
set -euo pipefail

FIFO_PATH=${1:-"./audio/in.pcm"}
DEVICE=${2:-"hw:Loopback,0,0"}

ffmpeg -hide_banner -loglevel warning \
  -f s16le -ar 48000 -ac 1 -i "$FIFO_PATH" \
  -f alsa "$DEVICE"
