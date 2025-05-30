use actix_files::{Files, NamedFile};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
//use std::path::PathBuf;

mod openai;
mod telegram {
    pub mod api;
}

mod routes {
    pub mod chat;
    pub mod media;
    pub mod voting;
}

mod state;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Сервер работает! 🚀")
}

#[post("/webhook")]
async fn telegram_webhook(body: web::Json<telegram::api::TelegramMessage>) -> HttpResponse {
    println!("➡️ Получен webhook от Telegram");

    if let Some(msg) = &body.message {
        println!("📨 Сообщение от чата {}: {:?}", msg.chat.id, msg.text);

        if let Some(text) = &msg.text {
            if let Some(reply) = openai::ask_openai(text).await {
                println!("🤖 Ответ OpenAI: {}", reply);
                telegram::api::send_message(msg.chat.id, &reply).await;
            } else {
                println!("⚠️ Не удалось получить ответ от OpenAI");
            }
        }
    } else {
        println!("⚠️ Пустое сообщение в webhook");
    }

    HttpResponse::Ok().finish()
}

#[get("/chat")]
async fn chat_page() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("./static/app/chat.html")?)
}

#[get("/upload")]
async fn upload_page() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("./static/app/upload.html")?)
}

#[get("/voting")]
async fn voting_page() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("./static/app/voting.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    println!("Сервер запущен на http://0.0.0.0:3000");

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(telegram_webhook)
            .service(chat_page)
            .service(upload_page)
            .service(voting_page)
            .service(Files::new("/app", "./static/app").index_file("index.html"))
            .service(routes::chat::chat_handler)
            .service(routes::media::upload)
            .service(routes::voting::list_images)
            .service(routes::voting::vote_image)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}

