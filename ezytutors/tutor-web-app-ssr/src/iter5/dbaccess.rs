use crate::errors::EzyTutorError;
use crate::model::*;
use sqlx::postgres::PgPool;

pub async fn get_user_record(pool: &PgPool, user_id: String)
-> Result<User, EzyTutorError> {
    let user_row = sqlx::query_as!(
        User,
        r#"SELECT * FROM ezyweb_user
        WHERE user_id = $1"#,
        user_id
    )
    .fetch_optional(pool)
    .await?;

    if let Some(user) = user_row {
        Ok(user)
    } else {
        Err(EzyTutorError::NotFound("User id not found".into()))
    }
}

pub async fn post_new_user(pool: &PgPool, new_user: User)
-> Result<User, EzyTutorError> {
    let user_row = sqlx::query_as!(
        User,
        r#"INSERT INTO ezyweb_user (user_id, tutor_id, user_password)
        VALUES ($1, $2, $3)
        RETURNING user_id, tutor_id, user_password"#,
        new_user.user_id,
        new_user.tutor_id,
        new_user.user_password
    )
    .fetch_one(pool)
    .await?;

    Ok(user_row)
}