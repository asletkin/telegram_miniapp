use serde::{Deserialize};
use std::env;
use reqwest::Client;
//use reqwest::multipart::{Form, Part};
//use tokio::fs::File;
use tokio::io::AsyncReadExt;

/// Структуры для обработки входящих сообщений от Telegram Webhook

#[derive(Deserialize)]
pub struct TelegramMessage {
    pub message: Option<Message>,
}

#[derive(Deserialize)]
pub struct Message {
    pub chat: Chat,
    pub text: Option<String>,
}

#[derive(Deserialize)]
pub struct Chat {
    pub id: i64,
}

/// Отправка текстового сообщения
pub async fn send_message(chat_id: i64, text: &str) {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let url = format!("https://api.telegram.org/bot{}/sendMessage", token);

    let body = serde_json::json!({
        "chat_id": chat_id,
        "text": text
    });

    let client = Client::new();
    let res = client.post(&url).json(&body).send().await;

    match res {
        Ok(r) => {
            if !r.status().is_success() {
                eprintln!("⚠️ Ошибка при отправке сообщения: {:?}", r.text().await);
            }
        }
        Err(e) => {
            eprintln!("❌ Ошибка HTTP при отправке сообщения: {:?}", e);
        }
    }
}

/// Отправка изображения или видео в канал
pub async fn send_media_to_channel(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("TELEGRAM_BOT_TOKEN")?;
    let chat_id = env::var("TELEGRAM_CHANNEL_ID")?;

    let client = Client::new();

    let mime_type = mime_guess::from_path(path)
        .first_or_octet_stream()
        .essence_str()
        .to_string();

    let mut file = tokio::fs::File::open(path).await?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).await?;

    let part = reqwest::multipart::Part::bytes(buf)
        .file_name(path.to_string())
        .mime_str(&mime_type)?;

    let form = if mime_type.starts_with("video") {
        reqwest::multipart::Form::new().part("video", part)
    } else {
        reqwest::multipart::Form::new().part("photo", part)
    };

    let url = if mime_type.starts_with("video") {
        format!("https://api.telegram.org/bot{}/sendVideo", token)
    } else {
        format!("https://api.telegram.org/bot{}/sendPhoto", token)
    };

    let res = client
        .post(&url)
        .multipart(form)
        .query(&[("chat_id", &chat_id)])
        .send()
        .await?;

    if !res.status().is_success() {
        eprintln!(
            "⚠️ Telegram API вернул ошибку при отправке медиа: {}",
            res.text().await.unwrap_or_default()
        );
    }

    Ok(())
}


