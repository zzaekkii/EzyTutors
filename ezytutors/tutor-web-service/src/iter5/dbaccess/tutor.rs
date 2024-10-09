use crate::errors::EzyTutorError;
use crate::models::tutor::{Tutor, NewTutor, UpdateTutor};
use sqlx::postgres::PgPool;

pub async fn get_all_tutors_db(pool: &PgPool)
-> Result<Vec<Tutor>, EzyTutorError> {
    let tutor_rows = sqlx::query!(
        "SELECT tutor_id, tutor_name, tutor_picture_url, tutor_profile
        FROM ezy_tutor_ch6")
            .fetch_all(pool)
            .await?;

    let tutors: Vec<Tutor> = tutor_rows
        .iter()
        .map(|tutor| Tutor {
            tutor_id: tutor.tutor_id,
            tutor_name: tutor.tutor_name.clone(),
            tutor_picture_url: tutor.tutor_picture_url.clone(),
            tutor_profile: tutor.tutor_profile.clone(),
        })
        .collect();

    match tutors.len() {
        0 => Err(EzyTutorError::NotFound("No tutors found".into())),
        _ => Ok(tutors),
    }
}

pub async fn get_tutor_details_db(
    pool: &PgPool,
    tutor_id: i32
) -> Result<Tutor, EzyTutorError> {
    let tutor_row = sqlx::query!(
        "SELECT tutor_id, tutor_name, tutor_picture_url, tutor_profile
        FROM ezy_tutor_ch6
        WHERE tutor_id = $1",
        tutor_id
    )
    .fetch_one(pool)
    .await
    .map(|tutor|
        Tutor {
            tutor_id: tutor.tutor_id,
            tutor_name: tutor.tutor_name.clone(),
            tutor_picture_url: tutor.tutor_picture_url.clone(),
            tutor_profile: tutor.tutor_profile.clone(),
        }    
    )
    .map_err(|_err| EzyTutorError::NotFound("Tutor id not found".into()))?;

    Ok(tutor_row)
}

pub async fn post_new_tutor_db(pool: &PgPool, new_tutor: NewTutor)
-> Result<Tutor, EzyTutorError> {
    let tutor_row = sqlx::query!(
        r#"INSERT INTO ezy_tutor_ch6
        (tutor_name, tutor_picture_url, tutor_profile)
        VALUES ($1, $2, $3)
        RETURNING tutor_id, tutor_name, tutor_picture_url, tutor_profile"#,
        new_tutor.tutor_name,
        new_tutor.tutor_picture_url,
        new_tutor.tutor_profile)
            .fetch_one(pool)
            .await?;
    
    Ok(Tutor {
        tutor_id: tutor_row.tutor_id,
        tutor_name: tutor_row.tutor_name.clone(),
        tutor_picture_url: tutor_row.tutor_picture_url.clone(),
        tutor_profile: tutor_row.tutor_profile.clone(),
    })
}

pub async fn update_tutor_details_db(
    pool: &PgPool,
    tutor_id: i32,
    update_tutor: UpdateTutor
) -> Result<Tutor, EzyTutorError> {
    let current_tutor_row = sqlx::query!(
        "SELECT tutor_id, tutor_name, tutor_picture_url, tutor_profile
        FROM ezy_tutor_ch6
        WHERE tutor_id = $1",
        tutor_id
    )
    .fetch_one(pool)
    .await
    .map_err(|_err| EzyTutorError::NotFound("Tutor id not found".into()))?;

    let name = update_tutor.tutor_name
        .unwrap_or(current_tutor_row.tutor_name);
    let picture_url = update_tutor.tutor_picture_url
        .unwrap_or(current_tutor_row.tutor_picture_url);
    let profile = update_tutor.tutor_profile
        .unwrap_or(current_tutor_row.tutor_profile);

    let updated_tutor = sqlx::query_as!(
        Tutor,
        r#"UPDATE ezy_tutor_ch6
        SET tutor_name = $1, tutor_picture_url = $2, tutor_profile = $3
        WHERE tutor_id = $4
        RETURNING *"#,
        name,
        picture_url,
        profile,
        tutor_id
    )
    .fetch_one(pool)
    .await?;

    Ok(updated_tutor)
}

pub async fn delete_tutor_db(pool: &PgPool, tutor_id: i32)
-> Result<String, EzyTutorError> {
    let tutor_row = sqlx::query!(
        "DELETE FROM ezy_tutor_ch6
        WHERE tutor_id = $1", // 매개변수를 사용하는 게 SQL 인젝션 대비에 더 유리.
        tutor_id
    )
    .execute(pool)
    .await
    .map_err(|_err| EzyTutorError::DBError("Unable to delete tutor".into()))?;

    Ok(format!("Deleted {:?} record", tutor_row.rows_affected()))
}