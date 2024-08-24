use super::handlers::*;
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg
    // courses라는 새 리소스 스코프 생성.
    .service(web::scope("/courses")
    // 요청을 new_course 핸들러로 전달.
    .route("/", web::post().to(new_course)));
}