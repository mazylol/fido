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

pub async fn random_by_breed(breed: String) -> Random {
    reqwest::Client::new()
        .get(format!("https://dog.ceo/api/breed/{breed}/images/random"))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

pub async fn random_by_sub_breed(breed: String, sub_breed: String) -> Random {
    reqwest::Client::new()
        .get(format!(
            "https://dog.ceo/api/breed/{breed}/{sub_breed}/images/random"
        ))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}
