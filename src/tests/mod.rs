


#[cfg(test)]
mod test {
    use crate::apis::spotify::SClient;


    #[tokio::test]
    async fn it_works() {
        use crate::apis::spotify::SClient;

        let client = SClient::new().await;

        dbg!(client.track("4R0Tpwu6rAynrG37MNZ0GY").await.unwrap());
    }

    #[tokio::test]
    async fn it_works_2() {
        let client = SClient::new().await;
        let result = client.search("king gnu", SearchType::Artist).await.unwrap();

        match &result {
            rspotify::model::search::SearchResult::Artists(track) => {
                dbg!(track.items[0].clone());
                println!("===========");
                dbg!(track.clone());
            },
            err => println!("error: {:?}", err),
        }
    }
}
