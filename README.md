# BirdNET iPhone Mic PWA

This stack captures iPhone Safari mic audio in a PWA and streams it to BirdNET Go via a WebRTC receiver that writes 48k mono PCM into a FIFO. A host `ffmpeg` process plays that FIFO into an ALSA loopback device for BirdNET Go to read.

## Quick Start (Linux host)

1. Start containers:

```bash
mkdir -p audio

# Set the BirdNET Go image you use (defaults to GHCR latest)
cp .env.example .env
# Edit .env and set BIRDNET_GO_IMAGE if you want a specific tag

docker compose up -d --build
```

4. Create the FIFO (if not created by the container) and start the ALSA bridge:

```bash
mkfifo -m 666 ./audio/in.pcm
./scripts/load_loopback.sh
./scripts/pipe_to_alsa.sh ./audio/in.pcm hw:Loopback,0,0
```

5. Configure BirdNET Go to read from `hw:Loopback,1,0`.

## BirdNET Go Compose Integration

`docker-compose.yml` includes a `birdnet_go` service that shares `/dev/snd` with the host and expects:

- Image set in `.env` via `BIRDNET_GO_IMAGE`
- Config directory at `./birdnet-go/config`
- Data directory at `./birdnet-go/data`

Set the input device in your BirdNET Go config to `hw:Loopback,1,0`.

## Files

- `app/app.py`: Flask + aiortc WebRTC receiver
- `app/templates/index.html`: PWA UI
- `app/static/main.js`: mic capture + WebRTC
- `docker-compose.yml`: app + Caddy proxy
- `Caddyfile`: TLS reverse proxy (edit hostname + cert paths)
- `scripts/pipe_to_alsa.sh`: FIFO -> ALSA loopback playback

## Notes

- Keep the iPhone screen awake while streaming.
- Safari requires HTTPS for mic permissions. You will need to serve the Flask app behind HTTPS (reverse proxy or tunnel).
- If you want STUN or WAN access, add ICE servers in `app/static/main.js`.
