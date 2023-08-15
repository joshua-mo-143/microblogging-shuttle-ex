use crate::routes::errors::ApiError;
use crate::routes::posts::Post;
use crate::storage::bucket::upload_image;
use crate::AppState;
use axum::{
    body::Bytes,
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize, sqlx::FromRow)]
pub struct User {
    displayname: String,
    bio: String,
}

pub async fn get_user_info(
    State(state): State<AppState>,
    Path(displayname): Path<String>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let query = match sqlx::query_as::<_, User>(
        r#"SELECT 
    displayname, bio
    FROM USERS
    WHERE displayname = $1
    ORDER BY CREATED_AT"#,
    )
    .bind(displayname)
    .fetch_all(&state.db)
    .await
    {
        Ok(res) => res,
        Err(e) => return Err(ApiError::SqlxError.to_error(e.to_string())),
    };

    Ok((StatusCode::OK, Json(query)))
}

pub async fn get_user_posts(
    State(state): State<AppState>,
    Path(displayname): Path<String>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let query = match sqlx::query_as::<_, Post>(
        r#"SELECT
        *
        FROM posts
        LEFT JOIN USERS
        on posts.author_id == user.id"#,
    )
    .fetch_all(&state.db)
    .await
    {
        Ok(res) => res,
        Err(e) => return Err(ApiError::SqlxError.to_error(e.to_string())),
    };

    Ok((StatusCode::OK, Json(query)))
}
