use crate::routes::errors::ApiError;
use crate::AppState;
use axum::{
    body::Bytes,
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use axum_extra::extract::cookie::{Cookie, PrivateCookieJar};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct OauthCode {
    code: String,
}

#[derive(Deserialize)]
pub struct OauthResponse {
    access_token: String,
    expires_in: i64,
    token_type: String,
    scope: String,
    refresh_token: String,
}

pub async fn google_callback(
    State(mut state): State<AppState>,
    jar: PrivateCookieJar,
    Json(json): Json<OauthCode>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let mut hmap: HashMap<&str, String> = HashMap::with_capacity(4);

    hmap.insert("code", json.code);
    hmap.insert("client_id", state.oauth_id.to_owned());
    hmap.insert("client_secret", state.oauth_secret.to_owned());
    hmap.insert("grant_type", "authorization_code".to_string());

    let res = match state
        .ctx
        .post("https://oauth2.googleapis.com/token")
        .json(&hmap)
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => return Err(ApiError::ReqwestError.to_error(e.to_string())),
    };

    let res = match res.json::<OauthResponse>().await {
        Ok(res) => res,
        Err(e) => return Err(ApiError::ReqwestError.to_error(e.to_string())),
    };

    let expires_at = Utc::now() + Duration::seconds(res.expires_in);

    let profile_url = format!(
        "https://www.googleapis.com/auth/userinfo.profile?access_token={}",
        res.access_token
    );

    let res = match state.ctx.get(profile_url).send().await {
        Ok(res) => res,
        Err(e) => return Err(ApiError::ReqwestError.to_error(e.to_string())),
    };

    println!("{}", res.text().await.unwrap());

    // state.sessions.insert("email", UserSession{access_token: res.access_token, expires_at});

    Ok(StatusCode::OK)
}
