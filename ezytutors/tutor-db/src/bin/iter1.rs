use dotenv::dotenv;
use std::{env, io};
use tsd::io;
use sqlx::postgres::PgPool;
use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct Course {
    pub course_id: i32,
    pub tutor_id: i32,
    pub course_name: String,
    pub posted_time: Option<NaiveDateTime>,
}

// 비동기 Actic 웹 서버 실행 후, sqlx로 db연결.
#[actix_rt::main]
async fn main() -> io::Result<()> {
    // 메모리에 환경 변수 로드.
    dotenv.ok();
    let database_url = env::var("DATABASE_URL").expect(
        "DATABASE_URL is not set in .env file");

        // sqlx로 db 커넥션 풀 생성.
    let db_pool = PgPool::connect(&database_url).await().unwrap();

    let course_rows = sqlx::query!(
        r#"select course_id, tutor_id, course_name, posted_time from ezy_course_ch4 where course_id = $1"#, 1
    )
    .fetch_all(&db_pool)
    .await
    .unwrap();

    let mut courses_list = vec![];
    for course_row in course_rows {
        courses_list.push(Course {
            course_id: course_row.course_id,
            tutor_id: course_row.tutor_id,
            course_name: course_row.course_name,
            posted_time: Some(chrono::NaiveDateTime::from(course_row.posted_time.unwrap())),
        })
    }
    println!("Courses = {:?}", courses_list);
    Ok(())

}