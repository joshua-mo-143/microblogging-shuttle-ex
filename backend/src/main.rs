use axum::{
    extract::FromRef,
    routing::{get, patch, post},
    Router,
};
use axum_extra::extract::cookie::Key;
use reqwest::Client;
use routes::{oauth, posts};
use s3::{creds::Credentials, Bucket, Region};
use shuttle_secrets::SecretStore;
use sqlx::PgPool;
use std::collections::HashMap;

pub mod routes;
pub mod storage;

#[derive(Clone)]
pub struct AppState {
    bucket: Bucket,
    db: PgPool,
    ctx: Client,
    oauth_id: String,
    oauth_secret: String,
    sessions: HashMap<String, UserSession>,
    key: Key,
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_shared_db::Postgres] db: PgPool,
    #[shuttle_secrets::Secrets] secrets: SecretStore,
) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!().run(&db).await.unwrap();

    let access_key = secrets.get("R2_ACCESS_KEY_ID").unwrap();
    let secret_key = secrets.get("R2_SECRET_ACCESS_KEY").unwrap();
    let account_id = secrets.get("R2_ACCOUNT_ID").unwrap();
    let oauth_id = secrets.get("GOOGLE_OAUTH_CLIENT_ID").unwrap();
    let oauth_secret = secrets.get("GOOGLE_OAUTH_CLIENT_SECRET").unwrap();

    let usersessions: HashMap<String, UserSession> = HashMap::new();

    let creds = Credentials::new(Some(&access_key), Some(&secret_key), None, None, None).unwrap();

    let bucket = Bucket::new(
        "shuttle-test",
        Region::Custom {
            region: "weur".to_string(),
            endpoint: format!("https://{account_id}.r2.cloudflarestorage.com"),
        },
        creds,
    )
    .unwrap();

    let ctx = Client::new();

    let sessions: HashMap<String, UserSession> = HashMap::new();

    let state = AppState {
        bucket,
        db,
        ctx,
        oauth_id,
        oauth_secret,
        sessions,
        key: Key::generate(),
    };

    let posts_router = Router::new().route("/", get(posts::get_all_posts).post(posts::create_post));

    let auth_router = Router::new().route("/google_callback", post(oauth::google_callback));

    let api_router = Router::new()
        .nest("/posts", posts_router)
        .nest("/auth", auth_router);

    let router = Router::new()
        .nest("/api", api_router)
        .with_state(state.into());

    Ok(router.into())
}

#[derive(Clone)]
pub struct UserSession {
    token_id: String,
    expires_at: String,
}
