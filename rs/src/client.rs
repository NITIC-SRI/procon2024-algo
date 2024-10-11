use crate::utils::Data;

pub async fn get(url: String) -> Data {
    let res = reqwest::get(&url).await.unwrap();
    let body = res.text().await;
    match body {
        Ok(body) => {
            let data: Data = serde_json::from_str(&body).unwrap();
            data
        }
        Err(_) => panic!("Error"),
    }
}
