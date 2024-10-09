use std::env;

use actix_web::{error, web, App, Error, HttpResponse, HttpServer, Result};
use actix_web::web::Data;
use serde::{Deserialize, Serialize};
use tera::Tera;

// Tera 템플릿 엔진을 AppState에 저장.
// 강사 이름 입력 폼 표시하는 핸들러 함수.
async fn index(tmpl: web::Data<tera::Tera>)
-> Result<HttpResponse, Error> {
    let s = tmpl
        // 아무 템플릿 변수도 없어서 데이터 삽입은 없음.
        // 하지만 Tera는 항상 Context 객체를 받기 때문에.
        .render("form.html", &tera::Context::new())
        .map_err(|_| error::ErrorInternalServerError("템플릿 에러"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[derive(Serialize, Deserialize)]
pub struct Tutor {
    name: String,
}

// submit 버튼 누르면 호출되는 핸들러 함수.
async fn handle_post_tutor(
    tmpl: web::Data<tera::Tera>,
    params: web::Form<Tutor>,
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("name", &params.name);
    ctx.insert("text", "Welcome!");

    let s = tmpl
        .render("user.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("템플릿 에러"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

// 웹 애플리케이션 라우트 구성.
fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            // 기본적으로 폼 입력 페이지 표시하는 핸들러 함수 호출.
            .service(web::resource("/").route(web::get().to(index)))
            // 입력 제출 시, post 요청 처리 핸들러 함수 호출.
            .service(web::resource("/tutors").route(web::post().to(handle_post_tutor)))
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on: 127.0.0.1:8080");
    HttpServer::new(|| {
        let tera = Tera::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/static/iter2/**/*"
        ))
        .unwrap();

        App::new()
            .app_data(Data::new(tera))
            .configure(app_config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}