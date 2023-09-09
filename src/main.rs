mod apis;
mod cache;
mod events;
mod managers;
mod slashcmds;
mod tests;
mod utils;

use serenity::framework::StandardFramework;

use serenity::http::Http;
use serenity::prelude::GatewayIntents;
use std::collections::HashSet;
use std::{env, error::Error};


use logger::{info, warn, error};
extern crate pretty_env_logger;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if let Err(e) = dotenv::dotenv() {
        error!("Unable to find .env configuration file: {}", e);
    }

    pretty_env_logger::init();

    let token = env::var("BOT_TOKEN").expect("Expected bot token in .env file");

    let http = Http::new(&token);
    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();

            owners.insert(info.owner.id);

            if let Some(team) = info.team {
                for member in &team.members {
                    owners.insert(member.user.id);
                }
            }

            (owners, info.id)
        }
        Err(why) => {
            warn!("Could not access application info: {:?}", why);
            warn!("Trying environment variable for bot id...");
            let id = env::var("BOT_ID").expect("Unable to find BOT_ID environment variable");
            let bot_id = id.parse::<u64>().expect("Invalid bot id");
            (HashSet::new(), serenity::model::id::ApplicationId(bot_id))
        }
    };

    info!(
        "Registering owner(s): {}",
        owners
            .iter()
            .map(|o| format!("{}", o.0))
            .collect::<Vec<String>>()
            .join(", ")
    );

    if cfg!(debug_assertions) {
        warn!("Running bot in DEBUG mode...");
    }

    let app_id = env::var("APPLICATION_ID").expect("Expected application id in .env file");
    let framework = StandardFramework::new()
        .before(events::before)
        .after(events::after)
        .configure(|c| c.owners(owners))
        .bucket("no_spam", |b| b.delay(3).time_span(10).limit(3))
        .await
        .on_dispatch_error(events::dispatch_error);

    let intents = GatewayIntents::all();
    let mut client = serenity::Client::builder(token, intents)
        .framework(framework)
        .event_handler(events::Handler)
        .application_id(app_id.parse::<u64>().unwrap())
        .await?;

    cache::fill(
        client.data.clone(),
        bot_id.0,
        client.shard_manager.clone(),
    )
    .await?;

    if let Err(why) = client.start_autosharded().await {
        error!("Client error: {:?}", why);
    }

    Ok(())
}
