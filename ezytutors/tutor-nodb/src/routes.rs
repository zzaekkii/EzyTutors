use super::handlers::*;
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        // courses라는 새 리소스 스코프 생성.
        web::scope("/courses")
            // 요청을 new_course 핸들러로 전달.
            .route("/", web::post().to(new_course))
            // 강사(tutor_id)의 강의들을 받아오는 라우트.
            .route("/{tutor_id}", web::get().to(get_courses_for_tutor))
            // 강사 세부 정보를 얻는 라우트.
            .route("/{tutor_id}/{course_id}", web::get().to(get_course_detail)),
    );
}