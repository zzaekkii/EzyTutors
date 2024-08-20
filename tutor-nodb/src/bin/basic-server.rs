// 모듈 import
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;

// 라우트 구성.
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

// 핸들러 구성.
pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("Hello, EzyTutors is stayin' alive~");
}

// HTTP 서버를 인스턴스화하고 실행.
#[actix_rt::main]
async fn main() -> io::Result<()> {
    // app 만들고 라우트 구성.
    let app = move || App::new().configure(general_routes);

    // HTTP 서버 시작.
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}