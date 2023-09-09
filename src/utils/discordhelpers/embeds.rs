use std::str;

use serenity::http::Http;
use serenity::{
    builder::{CreateEmbed, CreateMessage},
    model::prelude::*,
};

use crate::utils::constants::*;

#[derive(Default)]
pub struct EmbedOptions {
    pub is_assembly: bool,
}


pub trait ToEmbed {
    fn to_embed(self, author: &User, options: &EmbedOptions) -> CreateEmbed;
}


pub fn embed_message(emb: CreateEmbed) -> CreateMessage<'static> {
    let mut msg = CreateMessage::default();
    msg.embed(|e| {
        e.0 = emb.0;
        e
    });
    msg
}

pub async fn dispatch_embed(
    http: impl AsRef<Http>,
    channel: ChannelId,
    emb: CreateEmbed,
) -> serenity::Result<Message> {
    let emb_msg = embed_message(emb);
    channel
        .send_message(http, |e| {
            *e = emb_msg;
            e
        })
        .await
}



pub fn spotify_embed(user: &User) -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.color(SPOTIFY_GREEN)
        .author(|a|
            a.name(format!("{} is listening",
                           &user.name))
            .icon_url(&user.avatar_url().unwrap().as_str()));

    embed
}


pub fn build_invite_embed(invite_link: &str) -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.title("Invite Link");
    embed.color(COLOR_OKAY);
    embed.thumbnail(ICON_INVITE);
    let description = format!(
        "Click the link below to invite me to your server!\n\n[Invite me!]({})",
        invite_link
    );
    embed.description(description);
    embed
}

// pub fn build_join_embed(guild: &Guild) -> CreateEmbed {
//     let mut embed = CreateEmbed::default();
//     embed.title("Guild joined");
//     embed.color(COLOR_OKAY);
//     embed.field("Name", guild.name.clone(), true);
//     embed.field("Members", guild.member_count, true);
//     embed.field("Channels", guild.channels.len(), true);
//     if let Some(icon) = guild.icon_url() {
//         embed.thumbnail(icon);
//     }
//     embed.field("Guild ID", guild.id, true);
//     embed
// }

// pub fn build_leave_embed(guild: &GuildId) -> CreateEmbed {
//     let mut embed = CreateEmbed::default();
//     embed.title("Guild left");
//     embed.color(COLOR_FAIL);
//     embed.field("ID", format!("{}", guild.0), true);
//
//     embed
// }


pub fn build_fail_embed(author: &User, err: &str) -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.color(COLOR_FAIL);
    embed.title("Critical error");
    embed.description(err);
    let text = if author.discriminator == 0 { author.name.clone().to_string() } else { author.tag() };
    embed.footer(|f| f.text(format!("Requested by: {}", text)));

    embed
}
