use crate::errors::EzyTutorError;
use crate::dbaccess::course::*;
use crate::models::course::{CreateCourse, UpdateCourse};
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
    new_course: web::Json<CreateCourse>,
) -> Result<HttpResponse, EzyTutorError> {
    post_new_course_db(&app_state.db, new_course.try_into()?)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn delete_course(
    app_state: web::Data<AppState>,
    path: web::Path<(i32, i32)>,
) -> Result<HttpResponse, EzyTutorError> {
    let (tutor_id, course_id) = path.into_inner();
    delete_course_db(&app_state.db, tutor_id, course_id)
        .await
        .map(|resp| HttpResponse::Ok().json(resp))
}

pub async fn update_course_details(
    app_state: web::Data<AppState>,
    update_course: web::Json<UpdateCourse>,
    path: web::Path<(i32, i32)>,
) -> Result<HttpResponse, EzyTutorError> {
    let (tutor_id, course_id) = path.into_inner();
    update_course_details_db(&app_state.db, tutor_id, course_id, update_course.try_into()?)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

// 각 핸들러 함수 단위 테스트.
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use actix_web::ResponseError;
    use dotenv::dotenv;
    use sqlx::postgres::PgPool;
    use std::env;
    use std::sync::Mutex;
    // use crate::models::course::CourseLevel;

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
    async fn get_course_detail_success_test() {
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

    #[actix_rt::test]
    async fn get_course_detail_failure_test() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect(
            "DATABASE_URL is not set in .env file");

        let pool = PgPool::connect(&database_url).await.unwrap();

        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });

        let params = web::Path::from((1, 999));
        let resp = get_course_for_details(app_state, params).await;
        match resp {
            Ok(_) => println!("이게 왜 돼"),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        }
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

        let new_course_msg = CreateCourse {
            tutor_id: 1,
            course_name: "모든 개발자를 위한 HTTP 웹 기본 지식".into(),
            course_description: Some("이거 김영한 강의 아님 아무튼 아님".into()),
            course_format: None,
            course_level: Some("Beginner".into()),
            // course_level: CourseLevel::Beginner,
            course_price: None,
            course_duration: None,
            course_language: Some("한국어".into()),
            course_structure: None,
        };

        let course_param = web::Json(new_course_msg);
        let resp = post_new_course(app_state, course_param).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn update_course_success() {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });

        let update_course_msg = UpdateCourse {
            course_name: Some("Rust 마스터 코스 v.최최최최최종".into()),
            course_description: Some("테스틉니다요".into()),
            course_format: None,
            course_level: Some("Expert".into()),
            // course_level: Some(CourseLevel::Expert),
            course_price: None,
            course_duration: None,
            course_language: Some("한국어".into()),
            course_structure: None,
        };

        let params: web::Path<(i32, i32)> = web::Path::from((1, 2));
        let update_param = web::Json(update_course_msg);
        let resp = update_course_details(app_state, update_param, params)
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[ignore]
    #[actix_rt::test]
    async fn delete_test_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });

        let params: web::Path<(i32, i32)> = web::Path::from((1, 2));
        let resp = delete_course(app_state, params).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn delete_test_failure() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });

        let params: web::Path<(i32, i32)> = web::Path::from((1, 999));
        let resp = delete_course(app_state, params).await;
        match resp {
            Ok(_) => println!("이게 왜 있어"),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        }
    }
}