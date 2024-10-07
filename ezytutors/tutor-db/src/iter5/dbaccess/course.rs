use crate::errors::EzyTutorError;
use crate::models::course::*;
use sqlx::postgres::PgPool;

pub async fn get_courses_for_tutor_db(pool: &PgPool, tutor_id: i32)
-> Result<Vec<Course>, EzyTutorError> {
    // tutor_id 유효한지 검사.
    let tutor_exists = sqlx::query!(
        "SELECT * FROM ezy_course_ch6
        WHERE tutor_id = $1",
        tutor_id
    )
    .fetch_optional(pool)
    .await?;

    if tutor_exists.is_none() {
        return Err(EzyTutorError::NotFound("Tutor id is not found".into()))
    }

    let course_rows: Vec<Course> = sqlx::query_as!(
        Course,
        "SELECT * FROM ezy_course_ch6
        WHERE tutor_id = $1 ORDER BY course_id DESC",
        tutor_id
    )
    .fetch_all(pool) // 쿼리 실행해서 커넥션 풀에 전달.
    .await?;

    Ok(course_rows)
}

pub async fn get_course_for_details_db(pool: &PgPool, tutor_id: i32, course_id: i32)
-> Result<Course, EzyTutorError> {
    // tutor_id 유효한지 검사.
    let tutor_exists = sqlx::query!(
        "SELECT * FROM ezy_course_ch6
        WHERE tutor_id = $1",
        tutor_id
    )
    .fetch_optional(pool)
    .await?;

    if tutor_exists.is_none() {
        return Err(EzyTutorError::NotFound("Tutor id is not found".into()))
    }
    
    let course_row = sqlx::query_as!(
        Course,
        "SELECT * FROM ezy_course_ch6
        WHERE tutor_id = $1 AND course_id = $2",
        tutor_id, course_id
    )
    .fetch_optional(pool)
    .await?;

    if let Some(course) = course_row {
        Ok(course)
    } else {
        Err(EzyTutorError::NotFound("Course id not found".into()))
    }
}

pub async fn post_new_course_db(pool: &PgPool, new_course: CreateCourse)
-> Result<Course, EzyTutorError> {
    // SQL 구문 생성.
    let course_row = sqlx::query_as!(
        Course,
        r#"INSERT INTO ezy_course_ch6 (tutor_id, course_name, course_description, course_format, course_structure, course_duration, course_price, course_language, course_level)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING *"#,
        new_course.tutor_id,
        new_course.course_name,
        new_course.course_description,
        new_course.course_format,
        new_course.course_structure,
        new_course.course_duration,
        new_course.course_price,
        new_course.course_language,
        new_course.course_level
    )
    .fetch_one(pool)
    .await?;

    Ok(course_row)
}

pub async fn delete_course_db(
    pool: &PgPool,
    tutor_id: i32,
    course_id: i32,
) -> Result<String, EzyTutorError> {
    let course_row = sqlx::query!(
        "DELETE FROM ezy_course_ch6
        WHERE tutor_id = $1 AND course_id = $2",
        tutor_id, course_id,
    )
    .execute(pool)
    .await?;

    Ok(format!("Deleted {:#?} record", course_row))
}

pub async fn update_course_details_db(
    pool: &PgPool,
    tutor_id: i32,
    course_id: i32,
    update_course: UpdateCourse,
) -> Result<Course, EzyTutorError> {
    let current_course_row = sqlx::query_as!(
        Course,
        "SELECT * FROM ezy_course_ch6
        WHERE tutor_id = $1 AND course_id = $2",
        tutor_id, course_id
    )
    .fetch_one(pool)
    .await
    .map_err(|_err| EzyTutorError::NotFound(
        "Course id not found".into()))?;
    
    let name = update_course.course_name
        .unwrap_or(current_course_row.course_name);
    let description = update_course.course_description
        .unwrap_or(current_course_row.course_description.unwrap_or_default());
    let format = update_course.course_format
        .unwrap_or(current_course_row.course_format.unwrap_or_default());
    let structure = update_course.course_structure
        .unwrap_or(current_course_row.course_structure.unwrap_or_default());
    let duration = update_course.course_duration
        .unwrap_or(current_course_row.course_duration.unwrap_or_default());
    let level = update_course.course_level
        .unwrap_or(current_course_row.course_level.unwrap_or_default());
    let language = update_course.course_language
        .unwrap_or(current_course_row.course_language.unwrap_or_default());
    let price = update_course.course_price
        .unwrap_or(current_course_row.course_price.unwrap_or_default());

    let course_row = sqlx::query_as!(
        Course,
        r#"UPDATE ezy_course_ch6
        set course_name = $1, course_description = $2, course_format = $3, course_structure = $4,
        course_duration = $5, course_price = $6, course_language = $7, course_level = $8 
        WHERE tutor_id = $9 AND course_id = $10
        RETURNING *"#,
        name,
        description,
        format,
        structure,
        duration,
        price,
        language,
        level,
        tutor_id,
        course_id
    )
    .fetch_one(pool)
    .await;

    if let Ok(course) = course_row {
        Ok(course)
    } else {
        Err(EzyTutorError::NotFound("Course id not found".into()))
    }
}