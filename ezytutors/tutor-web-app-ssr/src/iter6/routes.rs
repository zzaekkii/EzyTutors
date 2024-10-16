use crate::handler::auth::{get_signup_form, get_signin_form, handle_signup, handle_signin};
use crate::handler::course::{delete_course, insert_course, update_course};
use actix_files as fs;
use actix_web::web;

pub fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(web::resource("/").route(web::get().to(get_signin_form)))
            .service(web::resource("/signin").route(web::post().to(handle_signin)))
            .service(web::resource("/signup")
                .route(web::get().to(get_signup_form))
                .route(web::post().to(handle_signup))),
    );
}

pub fn course_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/courses")
            .service(web::resource("{tutor_id}/course").route(web::post().to(insert_course)))
            .service(web::resource("{tutor_id}/{course_id}").route(web::post().to(update_course)))
            .service(web::resource("{tutor_id}/{course_id}/delete").route(web::post().to(delete_course)),
        ),
    );
}