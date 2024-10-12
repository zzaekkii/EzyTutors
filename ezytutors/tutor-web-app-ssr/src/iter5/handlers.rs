// use crate::dbaccess::{get_user_record, post_new_user};
use crate::errors::EzyTutorError;
use crate::iter5::state::AppState;
use crate::model::{TutorRegisterForm, TutorResponse, User};
use actix_web::{web, Error, HttpResponse, Result};
use argon2::{self, Config};
use serde_json::json;

use super::dbaccess::{get_user_record, post_new_user};

pub async fn show_register_form(tmpl: web::Data<tera::Tera>)
-> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("error", ""); // 서버에서 오류 종류 표시해주려고..
    ctx.insert("current_userid", "");
    ctx.insert("current_password", "");
    ctx.insert("current_confirmation", "");
    ctx.insert("current_name", "");
    ctx.insert("current_imageurl", "");
    ctx.insert("current_profile", "");
    
    let s = tmpl
        .render("register.html", &ctx)
        .map_err(|_| EzyTutorError::TeraError("템플릿 에러".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}

pub async fn handle_register(
    tmpl: web::Data<tera::Tera>,
    app_state: web::Data<AppState>,
    params: web::Form<TutorRegisterForm>,
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    let userid = params.userid.clone();
    let user = get_user_record(&app_state.db, userid.to_string()).await;

    // HTTP response body에 넣을 값.
    let s;

    // 이전에 만들어진 id가 아니라면 (id 중복 검사),
    if user.is_err() {
        // 비밀번호 매치 검사 (서버측)
        if params.password != params.confirmation {
            ctx.insert("error", "비밀번호가 일치하지 않습니다.");
            ctx.insert("current_userid", &params.userid);
            ctx.insert("current_password", "");
            ctx.insert("current_confirmation", "");
            ctx.insert("current_name", &params.name);
            ctx.insert("current_imageurl", &params.imageurl);
            ctx.insert("current_profile", &params.profile);

            s = tmpl
                .render("register.html", &ctx)
                .map_err(|_| EzyTutorError::TeraError("템플릿 에러".to_string()))?;
        } else { // 최종 통과.
            let new_tutor = json!({
                "tutor_name": &params.name,
                "tutor_picture_url": &params.imageurl,
                "tutor_profile": &params.profile
            });
            let awc_client = awc::Client::default();
            let res = awc_client
                .post("http://localhost:3000/tutors/")
                .send_json(&new_tutor)
                .await
                .unwrap()
                .body()
                .await?;

            let tutor_response: TutorResponse = serde_json::from_str(&std::str::from_utf8(&res)?)?;
            s = format!("강사 등록 완료.\n당신의 강사 id는 {}입니다.\n
            이지튜터를 시작하려면 로그인 하세요.\n", tutor_response.tutor_id);

            // 비밀번호 해싱.
            let salt = b"chunillyeom";
            let config = Config::default();
            let hash = argon2::hash_encoded(params.password.clone().as_bytes(), salt, &config).unwrap();
            let user = User {
                userid,
                tutor_id: Some(tutor_response.tutor_id),
                user_password: hash,
            };

            let _tutor_created = post_new_user(&app_state.db, user).await?;
        }
    } else {
        ctx.insert("error", "해당 id가 이미 존재합니다.");
        ctx.insert("current_userid", &params.userid);
        ctx.insert("current_password", "");
        ctx.insert("current_confirmation", "");
        ctx.insert("current_name", &params.name);
        ctx.insert("current_imageurl", &params.imageurl);
        ctx.insert("current_profile", &params.profile);
        
        s = tmpl
            .render("register.html", &ctx)
            .map_err(|_| EzyTutorError::TeraError("템플릿 에러".to_string()))?;
    };

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}