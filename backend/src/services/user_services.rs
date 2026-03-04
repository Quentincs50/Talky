use sqlx::PgPool;
use uuid::Uuid;
use bcrypt::{hash, DEFAULT_COST};
use crate::dto::user::CreateUser;
use crate::models::user::User;

pub async fn create_user(pool: &PgPool, req: CreateUser) -> Result<User, sqlx::Error> {
    let password_hash = hash(&req.password, DEFAULT_COST).unwrap();

    sqlx::query_as::<_, User>(
        r#"
            INSERT INTO users (id_user, username, email, password_hash)
            VALUES ($1, $2, $3, $4)
            RETURNING id_user, username, email, password_hash, status, created_at
            "#
        ).bind(Uuid::new_v4())
         .bind(&req.username)
         .bind(&req.email)
         .bind(&password_hash)
         .fetch_one(pool)
         .await
}

pub async fn authenticate_user(pool: &PgPool, email: &str, password: &str,) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = $1"
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;

    Ok(user.filter(|u| bcrypt::verify(password, &u.password_hash).unwrap_or(false)))
}

pub async fn get_user(pool: &PgPool, id: Uuid) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id_user = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}


pub async fn update_user(
    pool: &PgPool, id: Uuid, username: &str, email: &str, status: &str,
) -> Result<User, sqlx::Error> {

    sqlx::query_as::<_, User>(
        r#"
        UPDATE users
        SET username = $1,
            email = $2,
            status = $3
        WHERE id_user = $4
        RETURNING id_user, username, email, password_hash, status, created_at
        "#
    )
    .bind(username)
    .bind(email)
    .bind(status)
    .bind(id)
    .fetch_one(pool)
    .await
}
