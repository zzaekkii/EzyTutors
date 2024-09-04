use super::models::Course;
use sqlx::postgres::PgPool;

pub async fn get_courses_for_tutor_db(pool: &PgPool, tutor_id: i32) -> Vec<Course> {
    // SQL 구문 생성.
    let course_rows = sqlx::query!(
        "SELECT tutor_id, course_id, course_name, posted_time FROM
        ezy_course_ch4 WHERE tutor_id = $1",
        tutor_id
    )
    .fetch_all(pool) // 쿼리 실행해서 커넥션 풀에 전달.
    .await
    .unwrap();

    // 결과 추출 후, Vector로 변환.
    course_rows
        .iter()
        .map(|course_row| Course {
            tutor_id: course_row.tutor_id,
            course_id: course_row.course_id,
            course_name: course_row.course_name.clone(),
            posted_time: Some(chrono::NaiveDateTime::from(course_row.posted_time.unwrap())),
        })
        .collect()
}

pub async fn get_course_for_details_db(pool: &PgPool, tutor_id: i32, course_id: i32) -> Course {
    // SQL 구문 생성.
    let course_row = sqlx::query!(
        "SELECT tutor_id, course_id, course_name, posted_time FROM
        ezy_course_ch4 WHERE tutor_id = $1 AND course_id = $2",
        tutor_id, course_id
    )
    .fetch_one(pool)
    .await
    .unwrap();

    // 쿼리 실행 후 결과를 Course 구조체로 반환.
    Course {
        tutor_id: course_row.tutor_id,
        course_id: course_row.course_id,
        course_name: course_row.course_name.clone(),
        posted_time: Some(chrono::NaiveDateTime::from(course_row.posted_time.unwrap())),
    }
}

pub async fn post_new_course_db(pool: &PgPool, new_course: Course) -> Course {
    // SQL 구문 생성.
    let course_row = sqlx::query!("INSERT INTO ezy_course_ch4 (
    tutor_id, course_id, course_name) VALUES ($1, $2, $3) RETURNING tutor_id, course_id, course_name, posted_time", new_course.tutor_id, new_course.course_id, new_course.course_name)
    .fetch_one(pool)
    .await
    .unwrap();

    Course {
        tutor_id: course_row.tutor_id,
        course_id: course_row.course_id,
        course_name: course_row.course_name.clone(),
        posted_time: Some(chrono::NaiveDateTime::from(course_row.posted_time.unwrap())),
    }
}