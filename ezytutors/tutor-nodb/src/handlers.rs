use super::state::AppState;
use actix_web::{web, HttpResponse};
use super::models::Course;
use chrono::Utc; // 등록 시간.

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

pub async fn new_course(
    // HTTP 요청의 데이터 페이로드 + 애플리케이션 상태를 받음.
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    println!("Received new course");
    let course_count_for_user = app_state
        .courses
        .lock() // 데이터 접근 시, 잠가주기.
        .unwrap()
        .clone()
        .into_iter() // course 컬렉션을 iterator로 변환.
        .filter(|course| course.tutor_id == new_course.tutor_id) // 요청으로 받은 tutor_id와 일치하는 것만.
        .count(); // 강의 수를 세고, 다음 강의 id 생성에 사용.
    
    // Safely convert usize to i32
    let new_course_id: i32 = (course_count_for_user + 1).try_into().unwrap_or_else(|_| {
        // Handle conversion failure, perhaps by returning an error response
        panic!("Conversion from usize to i32 failed. Maybe overflow i32..")
    });

    // 새 강의 인스턴스 생성.
    let new_course = Course {
        tutor_id: new_course.tutor_id,
        course_id: Some(new_course_id),
        course_name: new_course.course_name.clone(),
        posted_time: Some(Utc::now().naive_utc()),
    };

    // 새 강의 인스턴스를 강의 컬렉션(AppState에 포함)에 추가.
    app_state.courses.lock().unwrap().push(new_course);
    HttpResponse::Ok().json("Added course")
}

// 강사 id로 검색 기능.
pub async fn get_courses_for_tutor(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> HttpResponse {
    let tutor_id: i32 = params.into_inner();

    let filtered_courses = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|course| course.tutor_id == tutor_id)
        .collect::<Vec<Course>>();

    if filtered_courses.len() > 0 { // 강의가 있다면,
        HttpResponse::Ok().json(filtered_courses)
    } else { // 강의가 없다면,
        HttpResponse::Ok().json("No courses found for tutor".to_string())
    }
}

// 강사 id + 강의 id 검색 기능.
pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> HttpResponse {
    let (tutor_id, course_id) = params.into_inner();
    let selected_course = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .find(|x| x.tutor_id == tutor_id && x.course_id == Some(course_id))
        .ok_or("Course not found"); // Option<T>를 Result<T, E>로 변환.

    if let Ok(course) = selected_course {
        HttpResponse::Ok().json(course)
    } else {
        HttpResponse::Ok().json("Course not found".to_string())
    }
}

// 테스트 코드.
#[cfg(test)] // cargo test 실행 시에만 실행됨.
mod tests {
    use super::*; // 부모 모듈로부터 모든 핸들러 선언 import.
    use actix_web::http::StatusCode;
    use std::sync::Mutex;

    // 비동기 test를 위해 actix web의 비동기 런타임이 이 함수를 실행하도록 지정.
    #[actix_rt::test]
    async fn post_course_test() {
        let course = web::Json(Course {
            tutor_id: 1,
            course_name: "스프링 MVC 2편 - 백엔드 웹 개발 활용 기술".into(),
            course_id: None,
            posted_time: None,
        });

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });

        // 핸들러 호출.
        let resp = new_course(course, app_state).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_all_courses_success() {
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });

        // 요청 매개변수 시뮬레이션.
        let tutor_id: web::Path<i32> = web::Path::from(1);
        // 핸들러 호출.
        let resp = get_courses_for_tutor(app_state, tutor_id).await;

        // 응답 확인.
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_one_course_success() {
        let app_state: web:: Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });

        // 2개 매개변수를 가진 요청 시뮬레이션을 위한 Path 타입 객체.
        let params: web::Path<(i32, i32)> = web::Path::from((1, 1));
        // 핸들러 호출.
        let resp = get_course_detail(app_state, params).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }
}