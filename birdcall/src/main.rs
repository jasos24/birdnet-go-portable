use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::builder::*;
use songbird::SerenityInit;

use std::env;
use std::fs;

mod commands;
mod pipeline;
mod rtmp;
mod audio;
mod util;

use commands::*;

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _data: Ready) {
        // BirdCall branding
        let bot_name = env::var("BOT_NAME").unwrap_or("BirdCall".into());
        let avatar_path = env::var("BOT_AVATAR_PATH").unwrap_or("assets/tech_finch.png".into());

        if let Ok(bytes) = fs::read(&avatar_path) {
            let _ = ctx.http.edit_profile(&EditProfile::new().username(bot_name).avatar(bytes)).await;
        }

        ctx.set_presence(
            Some(Activity::listening("the wild")),
            OnlineStatus::Online,
        );

        // Slash commands
        let commands = vec![
            CreateApplicationCommand::default()
                .name("join")
                .description("Join your voice channel"),
            CreateApplicationCommand::default()
                .name("stop")
                .description("Stop streaming and leave"),
            CreateApplicationCommand::default()
                .name("status")
                .description("Check stream status"),
            CreateApplicationCommand::default()
                .name("reconnect")
                .description("Restart FFmpeg"),
            CreateApplicationCommand::default()
                .name("setrtmp")
                .description("Set RTMP URL")
                .create_option(|o| {
                    o.name("url")
                        .description("New RTMP URL")
                        .kind(CommandOptionType::String)
                        .required(true)
                }),
            CreateApplicationCommand::default()
                .name("ping")
                .description("Healthcheck"),
        ];

        Command::set_global_application_commands(&ctx.http, |c| {
            for cmd in commands {
                c.create_application_command(|cc| {
                    *cc = cmd.clone();
                    cc
                });
            }
            c
        })
        .await
        .unwrap();

        println!("🦜 BirdCall is online.");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(cmd) = interaction {
            match cmd.data.name.as_str() {
                "join" => join::run(&ctx, &cmd).await,
                "stop" => stop::run(&ctx, &cmd).await,
                "status" => status::run(&ctx, &cmd).await,
                "reconnect" => reconnect::run(&ctx, &cmd).await,
                "setrtmp" => setrtmp::run(&ctx, &cmd).await,
                "ping" => ping::run(&ctx, &cmd).await,
                _ => {}
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .register_songbird()
        .await
        .unwrap();

    client.start().await.unwrap();
}
