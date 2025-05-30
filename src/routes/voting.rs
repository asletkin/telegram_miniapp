use actix_web::{get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use std::fs;
use crate::state::VOTES;

#[derive(Serialize)]
struct ImageWithVotes {
    filename: String,
    votes: u32,
}

#[get("/images")]
async fn list_images() -> HttpResponse {
    let mut data = vec![];
    let vote_map = VOTES.lock().unwrap();

    if let Ok(entries) = fs::read_dir("./static/uploads") {
        for entry in entries.flatten() {
            let file_name = entry.file_name().to_string_lossy().into_owned();
            let votes = vote_map.get(&file_name).cloned().unwrap_or(0);
            data.push(ImageWithVotes {
                filename: file_name,
                votes,
            });
        }
    }

    HttpResponse::Ok().json(data)
}

#[derive(Deserialize)]
struct VoteData {
    filename: String,
}

#[post("/vote")]
async fn vote_image(info: web::Json<VoteData>) -> HttpResponse {
    let mut vote_map = VOTES.lock().unwrap();
    *vote_map.entry(info.filename.clone()).or_insert(0) += 1;
    HttpResponse::Ok().body("Голос засчитан!")
}

