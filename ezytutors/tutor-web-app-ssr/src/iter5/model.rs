use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct TutorRegisterForm {
    pub userid: String,
    pub password: String,
    pub confirmation: String,
    pub name: String,
    pub imageurl: String,
    pub profile: String,
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
    pub userid: String,
    pub tutor_id: Option<i32>,
    pub user_password: String,
}