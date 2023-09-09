use serenity::{
    client::Context,
    framework::standard::CommandResult,
    model::application::interaction::application_command::ApplicationCommandInteraction,
};
use serenity::model::prelude::InteractionResponseType;

use crate::{
    utils::{
        constants::SPTFY_ERR_MSG_CASE_1,
        spotify::SpotifyActivity,
        discordhelpers::embeds::build_fail_embed,
    },
};
use crate::slashcmds::parse;

pub async fn track(ctx: &Context, msg: &ApplicationCommandInteraction) -> CommandResult {
    let user = parse(msg.clone());
    let mut activity = SpotifyActivity::new(&ctx, &msg, &user.id).await;

    if !activity.listening().await {
        let emb = build_fail_embed(&user, SPTFY_ERR_MSG_CASE_1);
        msg.create_interaction_response(&ctx.http, |resp| {
            resp.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|data| data.add_embed(emb).ephemeral(true))
        }).await?;
    } else {
        msg.create_interaction_response(&ctx.http, |resp| {
            let content = activity.get_track_url();
            resp.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|data| data.content(content))
        }).await?;
    }

    Ok(())
}