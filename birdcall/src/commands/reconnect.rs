use serenity::prelude::*;
use serenity::model::prelude::*;
use std::env;
use crate::streamer;

pub async fn run_reconnect(ctx: &Context, cmd: &ApplicationCommandInteraction) {
    let rtmp_url = env::var("RTMP_URL").unwrap();

    tokio::spawn(async move {
        streamer::start_stream(rtmp_url).await;
    });

    cmd.create_interaction_response(&ctx.http, |r| {
        r.interaction_response_data(|m| {
            m.content("🔄 BirdCall flapped its wings and refreshed the audio stream.")
        })
    }).await.ok();
}
