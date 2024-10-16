use crate::iter6::state::AppState;
use crate::dbaccess::{get_user_record, post_new_user};
use crate::errors::EzyTutorError;
use crate::model::{TutorRegisterForm, TutorSigninForm, TutorResponse, User};
use actix_web::{web, Error, HttpResponse, Result};
use argon2::{self, Config};
use serde_json::json;


pub async fn get_signup_form(tmpl: web::Data<tera::Tera>)
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
        .render("signup.html", &ctx)
        .map_err(|_| EzyTutorError::TeraError("템플릿 에러".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}

pub async fn handle_signup(
    tmpl: web::Data<tera::Tera>,
    app_state: web::Data<AppState>,
    params: web::Form<TutorRegisterForm>,
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    let user_id = params.user_id.clone();
    let user = get_user_record(&app_state.db, user_id.to_string()).await;

    // HTTP response body에 넣을 값.
    let s;

    // 이전에 만들어진 id가 아니라면 (id 중복 검사),
    if user.is_err() {
        // 비밀번호 매치 검사 (서버측)
        if params.password != params.confirmation {
            ctx.insert("error", "비밀번호가 일치하지 않습니다.");
            ctx.insert("current_userid", &params.user_id);
            ctx.insert("current_password", "");
            ctx.insert("current_confirmation", "");
            ctx.insert("current_name", &params.name);
            ctx.insert("current_imageurl", &params.imageurl);
            ctx.insert("current_profile", &params.profile);

            s = tmpl
                .render("signup.html", &ctx)
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
            s = format!("강사 등록 완료. 당신의 강사 id는 {}입니다. 
            이지튜터를 시작하려면 로그인 하세요.\n", tutor_response.tutor_id);

            // 비밀번호 해싱.
            let salt = b"chunillyeom";
            let config = Config::default();
            let hash = argon2::hash_encoded(params.password.clone().as_bytes(), salt, &config).unwrap();
            let user = User {
                user_id,
                tutor_id: Some(tutor_response.tutor_id),
                user_password: hash,
            };

            let _tutor_created = post_new_user(&app_state.db, user).await?;
        }
    } else {
        ctx.insert("error", "해당 id가 이미 존재합니다.");
        ctx.insert("current_userid", &params.user_id);
        ctx.insert("current_password", "");
        ctx.insert("current_confirmation", "");
        ctx.insert("current_name", &params.name);
        ctx.insert("current_imageurl", &params.imageurl);
        ctx.insert("current_profile", &params.profile);
        
        s = tmpl
            .render("signup.html", &ctx)
            .map_err(|_| EzyTutorError::TeraError("템플릿 에러".to_string()))?;
    };

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(s))
}

pub async fn get_signin_form(tmpl: web::Data<tera::Tera>)
-> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("error", "");
    ctx.insert("current_userid", "");
    ctx.insert("current_password", "");

    let s = tmpl
        .render("signin.html", &ctx)
        .map_err(|_| EzyTutorError::TeraError("템플릿 에러".to_string()))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn handle_signin(
    tmpl: web::Data<tera::Tera>,
    app_state: web::Data<AppState>,
    params: web::Form<TutorSigninForm>,
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    let s;
    let user_id = params.user_id.clone();
    let user = get_user_record(&app_state.db, user_id.to_string()).await;

    /*
     * 비밀번호 해싱&저장을 클라이언트 서버에서 하는 미친 구조를
     * 책에서 만들어놔서 이 부분은 챕터 끝나고 싹 바꿀 예정.
     * 
     * DB 설계 바꾸고, 렌더링 서버엔 DB 삭제.
     * 해싱도 백엔드에서 처리하고 넘겨주는 걸로 교체.
     * 렌더링 서버를 애초에 왜 따로 둔거지 이 책은?
     * 그냥 끝내고 새로 게시판/채팅 프로젝트를 만드는 게 낫겠다.
    */

    // 유저 아이디가 존재한다면,
    if let Ok(user) = user {
        let does_password_match = argon2::verify_encoded(
            &user.user_password.trim(),
            params.password.clone().as_bytes(),
        )
        .unwrap();
        
        // 비밀번호 일치하지 않으면,
        if !does_password_match {
            ctx.insert("error", "아이디 또는 비밀번호가 잘못되었습니다.");
            ctx.insert("current_userid", &params.user_id);
            ctx.insert("current_password", &params.password);

            s = tmpl
                .render("signin.html", &ctx)
                .map_err(|_| EzyTutorError::TeraError("템플릿 에러 여기야?".to_string()))?;
        } else { // 비밀번호가 일치하면,
            ctx.insert("title", &"로그인 성공!".to_owned());
            ctx.insert("name", &params.user_id);
            ctx.insert(
                "message",
                &"이지튜터에 성공적으로 로그인하셨습니다.".to_owned(),
            );

            s = tmpl
                .render("user.html", &ctx)
                .map_err(|_| EzyTutorError::TeraError("템플릿 에러".to_string()))?;
        }
    } else {
        ctx.insert("error", "아이디 또는 비밀번호가 잘못되었습니다.");
        ctx.insert("current_userid", &params.user_id);
        ctx.insert("current_password", &params.password);

        s = tmpl
            .render("signin.html", &ctx)
            .map_err(|_| EzyTutorError::TeraError("템플릿 에러".to_string()))?;
    }

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}