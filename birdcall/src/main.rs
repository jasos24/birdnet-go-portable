use serenity::prelude::*;
use serenity::model::gateway::Ready;
use serenity::model::prelude::interaction::Interaction;
use serenity::model::prelude::Activity;
use std::env;
use std::fs;

mod streamer;
mod commands;

use commands::{
    join::run_join,
    stop::run_stop,
    status::run_status,
    reconnect::run_reconnect,
    setrtmp::run_setrtmp,
    ping::run_ping,
};

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _data: Ready) {
        // Set bot name + avatar
        let bot_name = env::var("BOT_NAME").unwrap_or_else(|_| "BirdCall".into());
        let avatar_path = env::var("BOT_AVATAR_PATH").unwrap_or_else(|_| "assets/tech_finch.png".into());

        if let Ok(bytes) = fs::read(&avatar_path) {
            let _ = ctx.http.edit_current_user(Some(&bot_name), Some(&bytes)).await;
        }

        // Set presence
        ctx.set_activity(Activity::listening("the wild")).await;

        // Global slash commands
        serenity::model::id::CommandId(0)
            .set_global_application_commands(&ctx.http, |commands| {
                commands
                    .create_application_command(|c| c.name("join").description("BirdCall joins your voice channel"))
                    .create_application_command(|c| c.name("stop").description("BirdCall stops streaming and leaves"))
                    .create_application_command(|c| c.name("status").description("Check if BirdCall is streaming"))
                    .create_application_command(|c| c.name("reconnect").description("Restart the audio stream"))
                    .create_application_command(|c| {
                        c.name("setrtmp")
                            .description("Set a new RTMP URL")
                            .create_option(|o| {
                                o.name("url")
                                    .description("New RTMP URL")
                                    .kind(serenity::model::prelude::command::CommandOptionType::String)
                                    .required(true)
                            })
                    })
                    .create_application_command(|c| c.name("ping").description("Check if BirdCall is alive"))
            })
            .await
            .unwrap();

        println!("BirdCall is online");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(cmd) = interaction {
            match cmd.data.name.as_str() {
                "join" => run_join(&ctx, &cmd).await,
                "stop" => run_stop(&ctx, &cmd).await,
                "status" => run_status(&ctx, &cmd).await,
                "reconnect" => run_reconnect(&ctx, &cmd).await,
                "setrtmp" => run_setrtmp(&ctx, &cmd).await,
                "ping" => run_ping(&ctx, &cmd).await,
                _ => {}
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN");

    let mut client = serenity::Client::builder(token, serenity::GatewayIntents::all())
        .event_handler(Handler)
        .register_songbird()
        .await
        .unwrap();

    client.start().await.unwrap();
}
