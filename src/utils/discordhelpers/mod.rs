pub mod embeds;
pub mod interactions;

use serenity::model::prelude::*;

use serenity::client::bridge::gateway::ShardManager;
use tokio::sync::MutexGuard;


pub async fn send_global_presence(shard_manager: &MutexGuard<'_, ShardManager>, sum: u64) {
    let server_count = {
        if sum < 10000 {
            sum.to_string()
        } else {
            format!("{:.1}k", sum / 1000)
        }
    };

    let presence_str = format!("in {} servers", server_count);

    let runners = shard_manager.runners.lock().await;
    for (_, v) in runners.iter() {
        v.runner_tx
            .set_presence(Some(Activity::playing(&presence_str)), OnlineStatus::Online);
    }
}
