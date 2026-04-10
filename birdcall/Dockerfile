FROM rust:1.76 as builder

WORKDIR /app

# Copy source
COPY . .

# Build release binary
RUN cargo build --release

# Runtime image
FROM debian:stable-slim

RUN apt-get update && apt-get install -y ffmpeg && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy compiled binary
COPY --from=builder /app/target/release/birdcall /usr/local/bin/birdcall

# Copy assets (avatar)
COPY assets /app/assets

CMD ["birdcall"]
