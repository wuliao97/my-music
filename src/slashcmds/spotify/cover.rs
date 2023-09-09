use serenity::{
    client::Context,
    framework::standard::CommandResult,
    model::application::interaction::application_command::ApplicationCommandInteraction,
};

use crate::{
    utils::{
        constants::SPTFY_ERR_MSG_CASE_1,
        spotify::SpotifyActivity,
        discordhelpers::embeds::{
            spotify_embed,
            build_fail_embed,
        },
}};
use crate::slashcmds::parse;


pub async fn cover(ctx: &Context, msg: &ApplicationCommandInteraction) -> CommandResult {
    let user = parse(msg.clone());
    let mut activity = SpotifyActivity::new(&ctx, &msg, &user.id).await;
    let mut emb = spotify_embed(&user);

    if !activity.listening().await {
        emb = build_fail_embed(&user, SPTFY_ERR_MSG_CASE_1);
    } else {
        let title = activity.get_act().details.unwrap();
        let track_url = activity.get_track_url();
        let cover_url = activity.get_cover_url();

        emb.title(title)
            .url(track_url)
            .image(cover_url);
    };

    msg.create_interaction_response(&ctx.http, |resp| {
        resp.interaction_response_data(|data| {
            data.add_embed(emb)
        })
    }).await?;

    Ok(())
}