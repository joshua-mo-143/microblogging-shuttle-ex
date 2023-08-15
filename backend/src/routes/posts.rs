use crate::routes::errors::ApiError;
use crate::storage::bucket::upload_image;
use crate::AppState;
use axum::{
    body::Bytes,
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(sqlx::FromRow, Serialize)]
pub struct Post {
    id: i32,
    title: String,
    body: String,
    image_url: String,
    author: String,
}

pub async fn get_all_posts(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let query = match sqlx::query_as::<_, Post>(
        "SELECT
        posts.id, posts.title, posts.body, posts.image_url, users.displayname
        FROM POSTS
        LEFT JOIN USERS
        ON posts.author_id = users.id
        ORDER BY posts.created_at",
    )
    .fetch_all(&state.db)
    .await
    {
        Ok(res) => res,
        Err(e) => return Err(ApiError::SqlxError.to_error(e.to_string())),
    };

    Ok((StatusCode::OK, Json(query)))
}

pub async fn create_post(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let mut title = None;
    let mut body = None;
    let mut file = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let field_name = field.name().unwrap().to_string();
        match field_name.as_str() {
            "title" => title = Some(field.text().await.unwrap().to_string()),
            "body" => body = Some(field.text().await.unwrap().to_string()),
            "file" => file = Some(field.bytes().await.unwrap()),
            &_ => {}
        }
    }

    let title = title.unwrap();
    let body = body.unwrap();
    let file = file.unwrap();
    let title_as_uri = title.replace(" ", "-");
    let filename = format!("/username/01-01-01-{title_as_uri}.webp");

    if let Err(e) = upload_image(file, filename, state.bucket.clone()).await {
        return Err(ApiError::AWSError.to_error(e.to_string()));
    }

    if let Err(e) = sqlx::query(
        "INSERT INTO POSTS
        ",
    )
    .execute(&state.db)
    .await
    {
        return Err(ApiError::SqlxError.to_error(e.to_string()));
    }

    Ok(StatusCode::CREATED)
}
