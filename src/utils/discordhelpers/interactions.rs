use serenity::{
    builder::CreateEmbed,
    client::Context,
    model::application::interaction::application_command::ApplicationCommandInteraction,
    model::application::interaction::InteractionResponseType,
};

pub async fn send_error_msg(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
    edit: bool,
    fail_embed: CreateEmbed,
) -> serenity::Result<()> {
    if edit {
        command
            .edit_original_interaction_response(&ctx.http, |rsp| {
                rsp.content("")
                    .set_embeds(Vec::new())
                    .components(|cmps| cmps.set_action_rows(Vec::new()))
                    .set_embed(fail_embed)
            })
            .await?;

        Ok(())

    } else {
        command
            .create_interaction_response(&ctx.http, |resp| {
                resp.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|d| {
                        d.content("")
                            .set_embeds(Vec::new())
                            .components(|cmps| cmps.set_action_rows(Vec::new()))
                            .set_embed(fail_embed)
                    })
            })
            .await
    }
}
