use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TutorRegisterForm {
    pub user_id: String,
    pub password: String,
    pub confirmation: String,
    pub name: String,
    pub imageurl: String,
    pub profile: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TutorSigninForm {
    pub user_id: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TutorResponse {
    pub tutor_id: i32,
    pub tutor_name: String,
    pub tutor_picture_url: String,
    pub tutor_profile: String,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct User {
    pub user_id: String,
    pub tutor_id: Option<i32>,
    pub user_password: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct NewCourse {
    pub course_name: String, // 강의 이름.
    pub course_description: String, // 강의 설명.
    pub course_format: String, // 강의 형태(ex. 영상, e-book, 대면 교육 등).
    pub course_structure: Option<String>, // 첨부 설명 문서.
    pub course_duration: String, // 강의 길이.
    pub course_price: Option<i32>, // 강의 가격.
    pub course_language: Option<String>, // 강의 지원 언어.
    pub course_level: Option<String>, // 강의 수준.
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateCourse {
    pub course_name: Option<String>,
    pub course_description: Option<String>,
    pub course_format: Option<String>,
    pub course_structure: Option<String>,
    pub course_duration: Option<String>,
    pub course_price: Option<i32>,
    pub course_language: Option<String>,
    pub course_level: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewCourseResponse {
    pub tutor_id: i32,
    pub course_id: i32,
    pub course_name: String,
    pub course_description: String,
    pub course_format: String,
    pub course_structure: Option<String>,
    pub course_duration: String,
    pub course_price: Option<i32>,
    pub course_language: Option<String>,
    pub course_level: Option<String>,
    pub posted_time: String, // 강의 게시 일자(자동 생성)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UpdateCourseResponse {
    pub tutor_id: i32,
    pub course_id: i32,
    pub course_name: String,
    pub course_description: String,
    pub course_format: String,
    pub course_structure: String,
    pub course_duration: String,
    pub course_price: i32,
    pub course_language: String,
    pub course_level: String,
    pub posted_time: String,
}

impl From<web::Json<NewCourseResponse>> for NewCourseResponse {
    fn from(new_course: web::Json<NewCourseResponse>) -> Self {
        NewCourseResponse {
            tutor_id: new_course.tutor_id,
            course_id: new_course.course_id,
            course_name: new_course.course_name.clone(),
            course_description: new_course.course_description.clone(),
            course_format: new_course.course_format.clone(),
            course_structure: new_course.course_structure.clone(),
            course_duration: new_course.course_duration.clone(),
            course_price: new_course.course_price,
            course_language: new_course.course_language.clone(),
            course_level: new_course.course_level.clone(),
            posted_time: new_course.posted_time.clone(),
        }
    }
}

impl From<web::Json<UpdateCourseResponse>> for UpdateCourseResponse {
    fn from(new_course: web::Json<UpdateCourseResponse>) -> Self {
        UpdateCourseResponse {
            tutor_id: new_course.tutor_id,
            course_id: new_course.course_id,
            course_name: new_course.course_name.clone(),
            course_description: new_course.course_description.clone(),
            course_format: new_course.course_format.clone(),
            course_structure: new_course.course_structure.clone(),
            course_duration: new_course.course_duration.clone(),
            course_price: new_course.course_price,
            course_language: new_course.course_language.clone(),
            course_level: new_course.course_level.clone(),
            posted_time: new_course.posted_time.clone(),
        }
    }
}