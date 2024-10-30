// handlers.rs
use axum::{Json, Extension, extract::State, http::StatusCode};
use serde::Deserialize;
use diesel::prelude::*;
use crate::{models::{User, NewUser}, schema::users::dsl::*};
use crate::db::DbPool;
use crate::auth::{hash_password, verify_password, create_jwt};

#[derive(Deserialize)]
pub struct UserInput {
    pub username: String,
    pub password: String,
}

pub async fn register_user(
    State(pool): State<DbPool>,
    Json(user_input): Json<UserInput>,
) -> Result<StatusCode, (StatusCode, String)> {
    let conn = pool.get().expect("Failed to get DB connection");
    
    let password_hash = hash_password(&user_input.password).map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to hash password".to_string()))?;
    let new_user = NewUser {
        username: user_input.username.clone(),
        password_hash,
    };

    diesel::insert_into(users)
        .values(&new_user)
        .execute(&conn)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to register user".to_string()))?;

    Ok(StatusCode::CREATED)
}

pub async fn login_user(
    State(pool): State<DbPool>,
    Json(user_input): Json<UserInput>,
) -> Result<Json<String>, (StatusCode, String)> {
    let conn = pool.get().expect("Failed to get DB connection");

    let user = users
        .filter(username.eq(&user_input.username))
        .first::<User>(&conn)
        .optional()
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string()))?;

    if let Some(user) = user {
        if verify_password(&user.password_hash, &user_input.password).unwrap() {
            let token = create_jwt(&user.username);
            Ok(Json(token))
        } else {
            Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()))
        }
    } else {
        Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()))
    }
}
