use actix_web::{get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
//use std::collections::HashMap;

#[derive(Serialize)]
pub struct QuizQuestion {
    pub id: u32,
    pub question: String,
    pub options: Vec<String>,
    pub time_limit_sec: u32,
}

#[derive(Deserialize)]
pub struct QuizAnswer {
    pub id: u32,
    pub answer: usize,
}

#[derive(Serialize)]
pub struct QuizResult {
    pub correct: bool,
    pub correct_answer: usize,
}

static QUESTIONS: &[(&str, &[&str], usize)] = &[
    ("Столица Франции?", &["Париж", "Лондон", "Рим", "Берлин"], 0),
    ("2 + 2 * 2 = ?", &["6", "8", "4", "2"], 0),
    ("Язык, на котором мы пишем сейчас?", &["Rust", "Python", "Go", "C++"], 0),
];

#[get("/quiz")]
pub async fn get_quiz() -> HttpResponse {
    let questions: Vec<QuizQuestion> = QUESTIONS.iter().enumerate().map(|(i, (q, opts, _))| {
        QuizQuestion {
            id: i as u32,
            question: q.to_string(),
            options: opts.iter().map(|s| s.to_string()).collect(),
            time_limit_sec: 15,
        }
    }).collect();

    HttpResponse::Ok().json(questions)
}

#[post("/quiz/submit")]
pub async fn submit_quiz(data: web::Json<QuizAnswer>) -> HttpResponse {
    if let Some((_, _, correct_idx)) = QUESTIONS.get(data.id as usize) {
        let is_correct = data.answer == *correct_idx;
        HttpResponse::Ok().json(QuizResult {
            correct: is_correct,
            correct_answer: *correct_idx,
        })
    } else {
        HttpResponse::BadRequest().body("Неверный ID вопроса")
    }
}

