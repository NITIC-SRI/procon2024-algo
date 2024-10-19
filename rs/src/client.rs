use crate::utils::Data;

// -H "Procon-Token: token1"
pub async fn get(url: String, token: String) -> Data {
    let client = reqwest::Client::new();
    let res = client.get(&url).header("Procon-Token", token).send().await;

    let body = res.unwrap().text().await;
    match body {
        Ok(body) => {
            println!("Successfully got from {}", body);
            let data: Data = serde_json::from_str(&body).unwrap();
            data
        }
        Err(_) => panic!("Error"),
    }

pub async fn post(url: String, json_request: String, token: String) {
    let client = reqwest::Client::new();
    match client
        .post(&url)
        .header("Content-Type", "application/json")
        .header("Procon-Token", token)
        .body(json_request)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                println!("Successfully posted to {}", url);
            } else {
                eprintln!(
                    "Failed to post to {}: {}",
                    url,
                    response.text().await.unwrap()
                );
            }
        }
        Err(e) => eprintln!("Request error: {}", e),
    }
}