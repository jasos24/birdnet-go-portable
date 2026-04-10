use serenity::prelude::*;
use serenity::model::prelude::*;

pub async fn run(ctx: &Context, cmd: &ApplicationCommandInteraction) {
    cmd.create_interaction_response(&ctx.http, |r| {
        r.interaction_response_data(|d| d.content("🫧 BirdCall is awake and listening — all systems good."))
    })
    .await
    .ok();
}
