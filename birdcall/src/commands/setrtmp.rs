use serenity::prelude::*;
use serenity::model::prelude::*;
use std::env;
use std::fs;

pub async fn run_setrtmp(ctx: &Context, cmd: &ApplicationCommandInteraction) {
    let new_url = cmd.data.options[0].value.as_ref().unwrap().as_str().unwrap();

    let old = env::var("RTMP_URL").unwrap_or_default();
    let env_path = ".env";

    if let Ok(mut env_file) = fs::read_to_string(env_path) {
        if !old.is_empty() {
            env_file = env_file.replace(
                &format!("RTMP_URL={}", old),
                &format!("RTMP_URL={}", new_url),
            );
        } else {
            env_file.push_str(&format!("\nRTMP_URL={}", new_url));
        }
        let _ = fs::write(env_path, env_file);
    }

    cmd.create_interaction_response(&ctx.http, |r| {
        r.interaction_response_data(|m| {
            m.content("📡 BirdCall has locked onto a new stream destination.")
        })
    }).await.ok();
}
