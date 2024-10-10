use actix_files as fs;
use actix_web::{error, web, App, Error, HttpResponse, HttpServer, Result};
use actix_web::web::Data;
use serde::{Deserialize, Serialize};
use tera::Tera;
use awc::Client;

#[derive(Serialize, Deserialize, Debug)]
pub struct Tutor {
    pub tutor_id: i32,
    pub tutor_name: String,
    pub tutor_picture_url: String,
    pub tutor_profile: String,
}

async fn handle_get_tutors(tmpl: web::Data<tera::Tera>)
-> Result<HttpResponse, Error> {
    // 웹서비스 통신용 actix-web HTTP 클라이언트.
    let client = Client::default();

    // request 빌더 생성 & 전송.
    let response = client
        .get("http://localhost:3000/tutors/")
        .send() // 서버로 HTTP request 전송.
        .await
        .unwrap()
        .body() // 강사 리스트.
        .await
        .unwrap();

    // response body -> str 슬리이스로 변환.
    let str_list = std::str::from_utf8(&response.as_ref()).unwrap();
    // str 슬라이스를 Tutor 객체 벡터로 역직렬화.
    let tutor_list: Vec<Tutor> = serde_json::from_str(str_list).unwrap();
    let mut ctx = tera::Context::new();
    ctx.insert("tutors", &tutor_list);
    
    let rendered_html = tmpl
        .render("list.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("템플릿 에러"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(rendered_html))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on: 127.0.0.1:8080");
    HttpServer::new(|| {
        let tera = Tera::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/static/iter4/**/*"
        ))
        .unwrap();

        App::new()
            .app_data(Data::new(tera))
            .service(fs::Files::new("/static", "./static")
                .show_files_listing())
            .service(web::resource("/tutors").route(web::get().to(handle_get_tutors)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}