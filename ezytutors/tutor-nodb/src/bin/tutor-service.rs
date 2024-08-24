use actix_web::{web, App, HttpServer};
use std::io;
use std::sync::Mutex;

#[path = "../handlers.rs"]
mod handlers;
#[path ="../models.rs"]
mod models;
#[path = "../routes.rs"]
mod routes;
#[path = "../state.rs"]
mod state;

use routes::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // 애플리케이션 상태 초기화.
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm good. You've already asked me ".to_string(),
        visit_count: Mutex::new(0),
        courses: Mutex::new(vec![]),
    });

    // 웹 애플리케이션 정의.
    let app = move || {
        App::new()
            .app_data(shared_data.clone()) // 웹 애플리케이션 상태 등록.
            .configure(general_routes) // 웹 애플리케이션 라우트 구성.
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}