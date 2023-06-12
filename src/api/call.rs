use crate::api::types::Random;

pub async fn random() -> Random {
    reqwest::Client::new()
        .get(" https://dog.ceo/api/breeds/image/random")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}
