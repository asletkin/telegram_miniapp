use actix_web::{post, web, HttpResponse};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct GuessRequest {
    pub number: u8,
}

#[derive(Serialize)]
pub struct GuessResponse {
    pub correct: bool,
    pub target: u8,
    pub message: String,
}

#[post("/game/guess")]
pub async fn guess_number(info: web::Json<GuessRequest>) -> HttpResponse {
    let target = rand::thread_rng().gen_range(1..=10);

    let correct = info.number == target;
    let message = if correct {
        "🎉 Вы угадали!".to_string()
    } else {
        format!("❌ Неверно. Загадано число: {}", target)
    };

    HttpResponse::Ok().json(GuessResponse {
        correct,
        target,
        message,
    })
}

