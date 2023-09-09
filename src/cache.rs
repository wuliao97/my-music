use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::sync::Arc;

use tokio::sync::Mutex;
use tokio::sync::RwLock;

use serenity::client::bridge::gateway::ShardManager;
use serenity::prelude::{TypeMap, TypeMapKey};

use crate::managers::stats::StatsManager;
use crate::utils::blocklist::Blocklist;

use crate::managers::command::CommandManager;
use lru_cache::LruCache;
use serenity::model::channel::Message;
use crate::apis::spotify::SClient;

/** Caching **/

/// Contains bot configuration information provided mostly from environment variables
pub struct ConfigCache;
impl TypeMapKey for ConfigCache {
    type Value = Arc<RwLock<HashMap<&'static str, String>>>;
}

/// Our endpoints for the in-house statistics tracing - see apis/dbl.rs
pub struct StatsManagerCache;
impl TypeMapKey for StatsManagerCache {
    type Value = Arc<Mutex<StatsManager>>;
}

/// Internal blocklist for abusive users or guilds
pub struct BlocklistCache;
impl TypeMapKey for BlocklistCache {
    type Value = Arc<RwLock<Blocklist>>;
}

/// Contains the shard manager - used to send global presence updates
pub struct ShardManagerCache;
impl TypeMapKey for ShardManagerCache {
    type Value = Arc<Mutex<ShardManager>>;
}

#[derive(Clone)]
pub struct MessageCacheEntry {
    pub our_msg: Message,
    pub original_msg: Message,
}

impl MessageCacheEntry {
    pub fn new(our_msg: Message, original_msg: Message) -> Self {
        MessageCacheEntry {
            our_msg,
            original_msg,
        }
    }
}

/// Message  cache to interact with our own messages after they are dispatched
pub struct MessageCache;
impl TypeMapKey for MessageCache {
    type Value = Arc<Mutex<LruCache<u64, MessageCacheEntry>>>;
}

/// Holds the Command Manager which handles command registration logic
pub struct CommandCache;
impl TypeMapKey for CommandCache {
    type Value = Arc<RwLock<CommandManager>>;
}

pub struct SpotifyContainer;
impl TypeMapKey for SpotifyContainer {
    type Value = Arc<RwLock<SClient>>;
}


pub async fn fill(
    data: Arc<RwLock<TypeMap>>,
    id: u64,
    shard_manager: Arc<tokio::sync::Mutex<ShardManager>>,
) -> Result<(), Box<dyn Error>> {
    let mut data = data.write().await;

    // Lets map some common things in BotInfo
    let mut map = HashMap::<&str, String>::new();

    // optional additions
    let emoji_identifiers = [
        "SUCCESS_EMOJI_ID",
        "SUCCESS_EMOJI_NAME",
        "FAIL_EMOJI_NAME",
        "FAIL_EMOJI_ID",
        "LOADING_EMOJI_ID",
        "LOADING_EMOJI_NAME",
        "LOGO_EMOJI_NAME",
        "LOGO_EMOJI_ID",
    ];
    for id in &emoji_identifiers {
        if let Ok(envvar) = env::var(id) {
            if !envvar.is_empty() {
                map.insert(id, envvar);
            }
        }
    }

    map.insert("GIT_HASH_LONG", String::from(env!("GIT_HASH_LONG")));
    map.insert("GIT_HASH_SHORT", String::from(env!("GIT_HASH_SHORT")));
    map.insert("INVITE_LINK", env::var("INVITE_LINK")?);
    map.insert("DISCORDBOTS_LINK", env::var("DISCORDBOTS_LINK")?);
    map.insert("GITHUB_LINK", env::var("GITHUB_LINK")?);
    map.insert("STATS_LINK", env::var("STATS_LINK")?);
    map.insert("BOT_ID", id.to_string());
    data.insert::<ConfigCache>(Arc::new(RwLock::new(map)));

    // Shard manager for universal presence
    data.insert::<ShardManagerCache>(shard_manager);

    // Message delete cache
    data.insert::<MessageCache>(Arc::new(Mutex::new(LruCache::new(25))));

    // Stats tracking
    let stats = StatsManager::new();
    data.insert::<StatsManagerCache>(Arc::new(Mutex::new(stats)));

    // Blocklist
    let blocklist = Blocklist::new();
    data.insert::<BlocklistCache>(Arc::new(RwLock::new(blocklist)));

    // Commands
    let commands = CommandManager::new();
    data.insert::<CommandCache>(Arc::new(RwLock::new(commands)));

    // Spotify
    let spotify = SClient::new().await;
    data.insert::<SpotifyContainer>(Arc::new(RwLock::new(spotify)));

    Ok(())
}
