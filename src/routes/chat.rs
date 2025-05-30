use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::openai::ask_openai;

#[derive(Deserialize)]
pub struct ChatInput {
    message: String,
}

#[derive(Serialize)]
pub struct ChatOutput {
    reply: String,
}

#[post("/chat")]
pub async fn chat_handler(body: web::Json<ChatInput>) -> HttpResponse {
    if let Some(answer) = ask_openai(&body.message).await {
        HttpResponse::Ok().json(ChatOutput { reply: answer })
    } else {
        HttpResponse::InternalServerError().json(ChatOutput {
            reply: "Ошибка при обращении к OpenAI".into(),
        })
    }
}

