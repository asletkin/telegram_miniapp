use actix_multipart::Multipart;
use actix_web::{post, web, HttpRequest, HttpResponse, Error};
use futures_util::stream::StreamExt;
use serde::Deserialize;
use std::fs::File;
use std::io::Write;
use uuid::Uuid;

use crate::openai; // Подключаем openai::ask_openai

#[derive(Deserialize)]
struct ChatInput {
    message: String,
}

#[post("/chat")]
pub async fn chat_handler(
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse, Error> {
    if let Some(content_type) = req.headers().get("Content-Type") {
        if let Ok(ct_str) = content_type.to_str() {
            if ct_str.starts_with("application/json") {
                return handle_text(req, payload).await;
            } else if ct_str.starts_with("multipart/form-data") {
                return handle_file(&req, payload).await;
            }
        }
    }

    Ok(HttpResponse::BadRequest().body("Неподдерживаемый тип контента"))
}

async fn handle_text(_req: HttpRequest, mut payload: web::Payload) -> Result<HttpResponse, Error> {
    use actix_web::web::BytesMut;
    let mut body_bytes = BytesMut::new();

    while let Some(chunk) = payload.next().await {
        let data = chunk?;
        body_bytes.extend_from_slice(&data);
    }

    let parsed: ChatInput = serde_json::from_slice(&body_bytes)?;
    println!("📨 Текст: {}", parsed.message);

    // Ответ через OpenAI
    let reply = match openai::ask_openai(&parsed.message).await {
        Some(resp) => resp,
        None => "⚠️ Не удалось получить ответ от OpenAI".to_string(),
    };

    Ok(HttpResponse::Ok().json(serde_json::json!({ "reply": reply })))
}

async fn handle_file(req: &HttpRequest, payload: web::Payload) -> Result<HttpResponse, Error> {
    let mut multipart = Multipart::new(req.headers(), payload);

    while let Some(field) = multipart.next().await {
        let mut field = field?;
        let filename = format!(
            "{}_{}",
            Uuid::new_v4(),
            field.content_disposition().get_filename().unwrap_or("file")
        );
        let filepath = format!("./static/uploads/{}", filename);
        let mut f = File::create(&filepath)?;

        while let Some(chunk) = field.next().await {
            let data = chunk?;
            f.write_all(&data)?;
        }

        println!("📎 Файл сохранён: {}", filepath);
        return Ok(HttpResponse::Ok().body(format!("Файл получен: {}", filename)));
    }

    Ok(HttpResponse::BadRequest().body("Файл не получен"))
}

