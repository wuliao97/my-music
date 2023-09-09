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
        discordhelpers::embeds::{
            spotify_embed,
            build_fail_embed,
        },
    },
};
use crate::quote;
use crate::slashcmds::parse;


pub async fn listening(ctx: &Context, msg: &ApplicationCommandInteraction) -> CommandResult {
    let user = parse(msg.clone());
    let mut spotify_activity = SpotifyActivity::new(&ctx, &msg, &user.id).await;
    let mut embed = spotify_embed(&user);


    if !spotify_activity.listening().await {
        embed = build_fail_embed(&user, SPTFY_ERR_MSG_CASE_1);
    } else {
        embed.field("Title", quote!(spotify_activity.get_title()), false);
        embed.field("by", quote!(spotify_activity.get_artists()), false);
        embed.field("on", quote!(spotify_activity.get_album()), false);
        embed.footer(|f| f.text(spotify_activity.format_time()));
        embed.thumbnail(spotify_activity.get_cover_url());
    }

    msg.create_interaction_response(&ctx.http, |resp| {
        resp.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|data| data.add_embed(embed))
    })
    .await?;

    Ok(())
}