use std::env;
use serenity::{
    client::Context,
    framework::standard::CommandResult,
    model::application::interaction::application_command::ApplicationCommandInteraction,
};
use serenity::model::application::command::Command;

use crate::{quote, url};
use crate::utils::constants::COLOR_OKAY;


pub async fn about(ctx: &Context, msg: &ApplicationCommandInteraction) -> CommandResult {
    let command_pieces = {
        let commands = Command::get_global_application_commands(&ctx.http).await.unwrap().len();
        format!("`{}`", commands)
    };

    let cache = &ctx.cache;

    let users = {
        let s_count = &cache.guild_count();
        let u_count = &cache.user_count();
        format!("`{}` Servers\n`{}` Users", s_count, u_count)
    };
    let support = {
        let version = env::var("CARGO_PKG_VERSION").unwrap();
        format!("Project version: `{}`", version)
    };
    let source = {
        let gh_url = env::var("CARGO_PKG_REPOSITORY").unwrap();
        let readme_path = env::var("CARGO_PKG_README").unwrap();
        let readme_url = format!("{}/master/{}", gh_url, readme_path);
        quote!(format!("{}, {}", url!("Github", gh_url), url!("Readme", readme_url)))
    };

    let bot = cache.current_user();
    let avatar_url = bot.avatar_url().unwrap();
    let description = env::var("CARGO_PKG_DESCRIPTION").unwrap();

    let fields = vec![
        ("Command", quote!(command_pieces), true),
        ("User", quote!(users), true),
        ("Platform", quote!(support), true),
        ("Source", source, true),
    ];

    msg.create_interaction_response(&ctx.http, |resp| {
        resp.interaction_response_data(|data| {
            data.embed(|e| {
                e.title("Thanks for using me!")
                    .color(COLOR_OKAY)
                    .thumbnail(avatar_url)
                    .description(description)
                    .fields(fields)
            })
        })
    }).await?;

    Ok(())
}