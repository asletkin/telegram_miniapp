use std::env;
use reqwest::Client;
use serde_json::json;

pub async fn ask_openai(question: &str) -> Option<String> {
    let api_key = env::var("OPENAI_API_KEY").ok()?;
    let client = Client::new();

    let body = json!({
        "model": "gpt-4.1",
        "messages": [{
            "role": "user",
            "content": question
        }]
    });

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await;

    match response {
        Ok(res) => {
            let status = res.status();
            let text = res.text().await.unwrap_or_else(|_| "<empty>".to_string());
            println!("📥 Ответ OpenAI ({}): {}", status, text);

            if status.is_success() {
                let json: serde_json::Value = serde_json::from_str(&text).ok()?;
                Some(json["choices"][0]["message"]["content"].as_str()?.to_string())
            } else {
                None
            }
        }
        Err(err) => {
            println!("❌ Ошибка OpenAI запроса: {:?}", err);
            None
        }
    }
}


