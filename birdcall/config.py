import os

DISCORD_TOKEN = os.getenv("DISCORD_TOKEN")
STREAM_URL = os.getenv("STREAM_URL")
MEDIAMTX_URL = os.getenv("MEDIAMTX_URL")

if not DISCORD_TOKEN:
    raise RuntimeError("DISCORD_TOKEN missing")
