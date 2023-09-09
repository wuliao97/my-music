use rspotify::{Credentials, ClientCredsSpotify, ClientResult};
use rspotify::clients::BaseClient;
use rspotify::model::{idtypes, FullTrack, SearchResult, SearchType};

#[derive(Debug)]
pub struct SClient {
    client: ClientCredsSpotify,
}

#[allow(dead_code)]
impl SClient {
    #[must_use]
    pub async fn new() -> Self {
        dotenv::dotenv().unwrap();
        let creds = Credentials::from_env().unwrap();
        let client = ClientCredsSpotify::new(creds);
        client.request_token().await.unwrap();

        Self {
            client
        }
    }


    pub async fn search(&self, q: &str, search_type: SearchType) -> ClientResult<SearchResult> {
        let result = self.client.search(
            q,
            search_type,
            None,
            None,
            Some(50),
            None
        )
        .await.unwrap();

        Ok(result)
    }

    pub async fn track(&self, track_id: &str) -> ClientResult<FullTrack> {
        let id = idtypes::TrackId::from_id(track_id).unwrap();
        let track = self.client.track(id).await.unwrap();
        Ok(track)
    }
}

