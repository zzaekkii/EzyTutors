// 강의를 위한 데이터 모델.
use actix_web::web;
use chrono::NaiveDateTime;
// Rust 데이터 구조 - HTTP msg 전송용 포맷 역직렬화, 직렬화.
use serde::{Deserialize, Serialize}; 

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Course {
    pub tutor_id: i32,
    pub course_id: Option<i32>,
    pub course_name: String,
    pub posted_time: Option<NaiveDateTime>,
}

// HTTP request 데이터를 Rust 구조체로 변환.
impl From<web::Json<Course>> for Course {
    fn from(course: web::Json<Course>) -> Self {
        Course {
            tutor_id: course.tutor_id,
            course_id: course.course_id,
            course_name: course.course_name.clone(),
            posted_time: course.posted_time,
        }
    }
}