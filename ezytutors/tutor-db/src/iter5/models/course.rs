use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

use crate::errors::EzyTutorError;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Course {
    pub tutor_id: i32,
    pub course_id: i32, // (자동 생성)
    pub course_name: String, // 강의 이름.
    pub course_description: Option<String>, // 강의 설명.
    pub course_format: Option<String>, // 강의 형태(ex. 영상, e-book, 대면 교육 등).
    pub course_structure: Option<String>, // 첨부 설명 문서.
    pub course_duration: Option<String>, // 강의 길이.
    pub course_price: Option<i32>, // 강의 가격.
    pub course_language: Option<String>, // 강의 지원 언어.
    pub course_level: Option<String>, // 강의 수준.
    pub posted_time: Option<NaiveDateTime>, // 강의 게시 일자(자동 생성)
}

// 클라이언트 입력 전용 구조체 - 자동 생성 필드 제외.
#[derive(Deserialize, Debug, Clone)]
pub struct CreateCourse {
    pub tutor_id: i32,
    pub course_name: String, // 강의 이름.
    pub course_description: Option<String>, // 강의 설명.
    pub course_format: Option<String>, // 강의 형태(ex. 영상, e-book, 대면 교육 등).
    pub course_structure: Option<String>, // 첨부 설명 문서.
    pub course_duration: Option<String>, // 강의 길이.
    pub course_price: Option<i32>, // 강의 가격.
    pub course_language: Option<String>, // 강의 지원 언어.
    pub course_level: Option<String>, // 강의 수준.
}

// 강의 업데이트 전용 구조체 - tutor_id 고정.
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

impl TryFrom<web::Json<CreateCourse>> for CreateCourse {
    type Error = EzyTutorError;

    fn try_from(new_course: web::Json<CreateCourse>) -> Result<Self, Self::Error> {
        Ok(CreateCourse {
            tutor_id: new_course.tutor_id,
            course_name: new_course.course_name.clone(),
            course_description: new_course.course_description.clone(),
            course_format: new_course.course_format.clone(),
            course_structure: new_course.course_structure.clone(),
            course_duration: new_course.course_duration.clone(),
            course_price: new_course.course_price,
            course_language: new_course.course_language.clone(),
            course_level: new_course.course_level.clone(),
        })
    }
}

impl TryFrom<web::Json<UpdateCourse>> for UpdateCourse {
    type Error = EzyTutorError;

    fn try_from(new_course: web::Json<UpdateCourse>) -> Result<Self, Self::Error> {
        Ok(UpdateCourse {
            course_name: new_course.course_name.clone(),
            course_description: new_course.course_description.clone(),
            course_format: new_course.course_format.clone(),
            course_structure: new_course.course_structure.clone(),
            course_duration: new_course.course_duration.clone(),
            course_price: new_course.course_price,
            course_language: new_course.course_language.clone(),
            course_level: new_course.course_level.clone(),
        })
    }
}