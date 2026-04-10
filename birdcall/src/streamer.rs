use std::process::Command;

pub async fn start_stream(rtmp_url: String) {
    Command::new("ffmpeg")
        .args([
            "-f", "s16le",
            "-ar", "48000",
            "-ac", "2",
            "-i", "pipe:0",
            "-c:a", "aac",
            "-b:a", "128k",
            "-f", "flv",
            &rtmp_url,
        ])
        .spawn()
        .expect("Failed to start ffmpeg");
}
