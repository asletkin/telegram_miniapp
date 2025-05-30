use actix_multipart::Multipart;
use actix_web::{post, HttpResponse};
use futures_util::stream::StreamExt as _;
use std::fs::File;
use std::io::Write;
use uuid::Uuid;

use crate::telegram::api::send_media_to_channel;

#[post("/upload")]
pub async fn upload(mut payload: Multipart) -> HttpResponse {
    let mut saved_file_path = None;

    while let Some(item) = payload.next().await {
        if let Ok(mut field) = item {
            let filename = format!("{}_{}", Uuid::new_v4(), field.content_disposition().get_filename().unwrap_or("file"));
            let filepath = format!("./static/uploads/{}", filename);
            let mut f = File::create(&filepath).unwrap();

            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                f.write_all(&data).unwrap();
            }

            saved_file_path = Some(filepath.clone());

            // Отправим в Telegram
            if let Err(e) = send_media_to_channel(&filepath).await {
                println!("Ошибка при отправке в Telegram: {:?}", e);
            }
        }
    }

    if let Some(path) = saved_file_path {
        HttpResponse::Ok().body(format!("Файл загружен: {}", path))
    } else {
        HttpResponse::BadRequest().body("Не удалось загрузить файл")
    }
}

