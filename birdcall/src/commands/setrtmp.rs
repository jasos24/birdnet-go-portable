use serenity::prelude::*;
use serenity::model::prelude::*;
use crate::rtmp;

pub async fn run(ctx: &Context, cmd: &ApplicationCommandInteraction) {
    let new_url = cmd.data.options[0].value.as_str().unwrap();

    rtmp::set_rtmp_url(new_url);

    cmd.create_interaction_response(&ctx.http, |r| {
        r.interaction_response_data(|d| d.content("📡 BirdCall updated the RTMP destination."))
    })
    .await
    .ok();
}
