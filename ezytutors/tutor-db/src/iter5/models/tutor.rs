use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Tutor {
    pub tutor_id: i32, // 고유 id (자동생성)
    pub tutor_name: String,
    pub tutor_picture_url: String, // 강사 사진 URL.
    pub tutor_profile: String, // 강사 간단 소개.
}

#[derive(Deserialize, Debug, Clone)]
pub struct NewTutor {
    pub tutor_name: String,
    pub tutor_picture_url: String,
    pub tutor_profile: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateTutor {
    pub tutor_name: Option<String>,
    pub tutor_picture_url: Option<String>,
    pub tutor_profile: Option<String>,
}

impl From<web::Json<NewTutor>> for NewTutor {
    fn from(new_tutor: web::Json<NewTutor>) -> Self {
        NewTutor {
            tutor_name: new_tutor.tutor_name.clone(),
            tutor_picture_url: new_tutor.tutor_picture_url.clone(),
            tutor_profile: new_tutor.tutor_profile.clone(),
        }
    }
}

impl From<web::Json<UpdateTutor>> for UpdateTutor {
    fn from(new_tutor: web::Json<UpdateTutor>) -> Self {
        UpdateTutor {
            tutor_name: new_tutor.tutor_name.clone(),
            tutor_picture_url: new_tutor.tutor_picture_url.clone(),
            tutor_profile: new_tutor.tutor_profile.clone(),
        }
    }
}