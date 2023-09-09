use serenity::{
    client::Context,
    model::{
        gateway::Activity,
        id::UserId,
        prelude::application_command::ApplicationCommandInteraction,
    },
};
use chrono::*;

use crate::url;
use crate::apis::spotify::SClient;
use crate::utils::constants::{SPOTIFY_TRACK_URL, SPOTIFY_GREEN};


#[derive(Debug)]
pub struct SpotifyActivity {
    pub activity: Option<Activity>,
    client: SClient,
    urls: Vec<String>,
}

#[allow(dead_code)]
impl SpotifyActivity {
    #[must_use]
    pub async fn new(ctx: &Context, msg: &ApplicationCommandInteraction, user_id: &UserId) -> Self {
        let cache = &ctx.cache;
        let guild_id = &msg.guild_id.unwrap();
        let cached_guild = cache.guild(guild_id).unwrap();

        let presence = cached_guild.presences.get(&user_id).unwrap();
        let activity = presence.activities.iter().find(|activity| activity.name.eq("Spotify")).cloned();

        let client = SClient::new().await;
        let urls = Vec::new();

        Self {
            activity,
            client,
            urls,
        }
    }


    pub async fn listening(&mut self) -> bool {
        if self.activity.is_some() {
            self.extract_url().await;

            return true;
        }
        false
    }


    pub async fn extract_url(&mut self) {
        let track = self.client.track(self.get_track_id().as_str()).await.unwrap();
        self.urls.push(track.external_urls.get("spotify").unwrap().clone());
        self.urls.push(track.album.external_urls.get("spotify").unwrap().clone());

        let artist_urls = track.artists
            .iter()
            .map(|artist| artist.external_urls.get("spotify").unwrap().clone())
            .collect::<Vec<String>>();

        self.urls.extend(artist_urls);
    }


    pub fn get_act(&self) -> Activity {
        self.activity.clone().unwrap()
    }


    pub fn get_title(&self) -> String {
        let material = self.get_act();
        let title = &material.details.unwrap();
        let url = self.get_track_url();
        url!(title, url)
    }


    pub fn get_artists(&self) -> String {
        let material = self.get_act();
        let artists = material.state.unwrap().split(";").map(|a| a.to_string()).collect::<Vec<String>>();
        let artist_url = &self.urls[2..];

        if artists.len() != artist_url.len() {
            let artist = artists.get(0).unwrap();
            let url = self.urls.get(0).unwrap();
            return url!(artist, url);
        }

        let mut urls = Vec::new();
        for (artist, url) in artists.iter().zip(artist_url) {
            urls.push(url!(artist, url));
        }

        urls.join(", ")
    }


    pub fn get_album(&self) -> String {
        let material = self.get_act();
        let album = material.assets.unwrap().large_text.unwrap();
        let url = self.urls.get(1).unwrap();
        url!(album, url)
    }


    pub fn get_cover_url(&self) -> String {
        let material = self.get_act();
        let cover_literal = material.assets.unwrap().large_image.unwrap();
        let i = cover_literal.char_indices().nth(8).unwrap().0;
        let cover = &cover_literal[i..];

        format!("https://i.scdn.co/image/{}", cover.to_string())
    }


    pub fn get_track_id(&self) -> String {
        let material = self.get_act();

        String::from(material.sync_id.clone().unwrap())
    }


    pub fn get_track_url(&self) -> String {
        let track_id = self.get_track_id();
        format!("{}{}", SPOTIFY_TRACK_URL, track_id)
    }

    pub fn start(&self) -> u64 {
        self.get_act().timestamps.unwrap().start.unwrap() / 1000
    }

    pub fn end(&self) -> u64 {
        self.get_act().timestamps.unwrap().end.unwrap() / 1000
    }

    pub fn duration(&self) -> u64 {
         self.end() - self.start()
    }

    pub fn format_time(&self) -> String {
        let duration = self.duration() as i64;
        let time = NaiveDateTime::from_timestamp_opt(duration, 0).unwrap();
        let length = if duration < 60 {
             time.format("%S")
        } else if duration > 3600 {
            time.format("%H:%M:%S")
        } else {
            time.format("%M:%S")
        };
        format!("Tile: {}", length).to_string()
    }

    pub fn party_id(&self) -> String {
        self.get_act().party.unwrap().id.unwrap()
    }

    pub async fn get_colour() -> u32 {
        SPOTIFY_GREEN
    }
}