use std::sync::Arc;

#[allow(dead_code)]
pub struct StatsManager {
    client: Arc<reqwest::Client>,
    servers: u64,
    shards: u64,
    boot_count: Vec<u64>,
    leave_queue: u64,
    join_queue: u64,
}

impl StatsManager {
    pub fn new() -> StatsManager {
        StatsManager {
            client: Arc::new(reqwest::Client::new()),
            servers: 0,
            leave_queue: 0,
            join_queue: 0,
            shards: 0,
            boot_count: Vec::new(),
        }
    }

    pub fn shard_count(&self) -> u64 {
        self.shards
    }

    pub fn add_shard(&mut self, server_count: u64) {
        self.shards += 1;
        self.boot_count.push(server_count);
    }
}
