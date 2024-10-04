use crate::errors::EzyTutorError;
use crate::dbaccess::course::*;
use crate::models::course::Course;
use crate::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn get_courses_for_tutor(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, EzyTutorError> {
    let tutor_id = path.into_inner();
    get_courses_for_tutor_db(&app_state.db, tutor_id)
        .await
        .map(|courses| HttpResponse::Ok().json(courses))
}

pub async fn get_course_for_details(
    app_state: web::Data<AppState>,
    path: web::Path<(i32, i32)>,
) -> Result<HttpResponse, EzyTutorError> {
    let (tutor_id, course_id) = path.into_inner();
    get_course_for_details_db(&app_state.db, tutor_id, course_id)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn post_new_course(
    app_state: web::Data<AppState>,
    new_course: web::Json<Course>,
) -> Result<HttpResponse, EzyTutorError> {
    
    post_new_course_db(&app_state.db, new_course.into())
        .await
        .map(|course| HttpResponse::Ok().json(course))
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

        let tutor_id = web::Path::from(1);
        let resp = get_courses_for_tutor(app_state, tutor_id).await.unwrap();
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
        let resp = get_course_for_details(app_state, params).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[ignore]
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
            course_id: 9,
            tutor_id: 1,
            course_name: "모든 개발자를 위한 HTTP 웹 기본 지식".into(),
            posted_time: Some(posted_time),
        };

        let course_param = web::Json(new_course_msg);
        let resp = post_new_course(app_state, course_param).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
    }
}