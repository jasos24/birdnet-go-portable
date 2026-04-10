use serenity::prelude::*;
use serenity::model::prelude::*;
use songbird::SerenityInit;

pub async fn run(ctx: &Context, cmd: &ApplicationCommandInteraction) {
    let guild_id = cmd.guild_id.unwrap();

    let manager = songbird::get(ctx).await.unwrap().clone();
    let _ = manager.remove(guild_id).await;

    cmd.create_interaction_response(&ctx.http, |r| {
        r.interaction_response_data(|d| d.content("🌙 BirdCall glides away. Stream ended."))
    })
    .await
    .ok();
}
