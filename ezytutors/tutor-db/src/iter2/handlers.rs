// 그냥 에러 확인용 스켈레톤 코드임. 별 동작 없음.
use super::models::Course;
use super::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

pub async fn get_courses_for_tutor(
    _app_state: web::Data<AppState>,
    _params: web::Path<(i32,)>,
) -> HttpResponse {
    HttpResponse::Ok().json("Success")
}

pub async fn get_course_details(
    _app_state: web::Data<AppState>,
    _params: web::Path<(i32, i32)>,
) -> HttpResponse {
    HttpResponse::Ok().json("Success")
}

pub async fn post_new_course(
    _new_course: web::Json<Course>,
    _app_state: web::Data<AppState>
) -> HttpResponse {
    HttpResponse::Ok().json("Success")
}


// 각 핸들러 함수 단위 테스트.
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
    use dotenv::dotenv;
    use sqlx::postgres::PgPool;
    use std::env;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn get_all_courses_success() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect(
            "DATABASE_URL is not set in .env file");

        let pool = PgPool::connect(&database_url).await.unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });

        let tutor_id = web::Path::from((1, ));
        let resp = get_courses_for_tutor(app_state, tutor_id).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_course_details_test() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect(
            "DATABASE_URL is not set in .env file");

        let pool = PgPool::connect(&database_url).await.unwrap();

        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });

        let params = web::Path::from((1, 2));
        let resp = get_course_details(app_state, params).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn post_course_success() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect(
            "DATABASE_URL is not set in .env file");

        let pool = PgPool::connect(&database_url).await.unwrap();

        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });

        let date = NaiveDate::from_ymd_opt(2024, 9, 3).unwrap();
        let time = NaiveTime::from_hms_opt(21, 31, 33).unwrap();
        let posted_time = NaiveDateTime::new(date, time);

        let new_course_msg = Course {
            course_id: 1,
            tutor_id: 1,
            course_name: "모든 개발자를 위한 HTTP 웹 기본 지식".into(),
            posted_time: Some(posted_time),
        };

        let course_param = web::Json(new_course_msg);
        let resp = post_new_course(course_param, app_state).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }
}