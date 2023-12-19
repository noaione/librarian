use axum::{
    extract::{Query, State},
    http::HeaderMap,
    response::IntoResponse,
    Json, Router,
};
use redis::{aio::Connection, AsyncCommands, Commands};
use serde_json::Value;

use crate::{komga::KomgaUserCreateOptionSharedLibraries, AppState};

use super::AuthToken;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct InviteOption {
    #[serde(rename = "labelsAllow")]
    pub labels_allow: Option<Vec<String>>,
    #[serde(rename = "labelsExclude")]
    pub labels_exclude: Option<Vec<String>>,
    #[serde(rename = "sharedLibraries")]
    pub shared_libraries: Option<KomgaUserCreateOptionSharedLibraries>,
    #[serde(rename = "expiresAt")]
    pub expire_at: Option<u64>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct InviteToken {
    token: String,
    option: InviteOption,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct InviteQuery {
    token: String,
}

pub async fn create_invite_token(
    State(state): State<AppState>,
    _: AuthToken,
    Json(option): Json<InviteOption>,
) -> impl IntoResponse {
    let token = uuid::Uuid::new_v4().to_string();

    let invite_token = InviteToken {
        token: token.clone(),
        option,
    };

    let mut redis_conn = state.redis.get_connection().unwrap();
    redis_conn
        .set::<String, String, String>(
            format!("librarian:invite:{}", token),
            serde_json::to_string(&invite_token).unwrap(),
        )
        .unwrap();

    let invite_token_json: Value =
        serde_json::from_str(&serde_json::to_string(&invite_token).unwrap()).unwrap();

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    // wrap the json in a {"ok": true, "data": {}} object
    let wrapped_json: Value = serde_json::json!({
        "ok": true,
        "data": invite_token_json
    });

    (headers, serde_json::to_string(&wrapped_json).unwrap())
}

pub async fn get_invite_config(State(state): State<AppState>, _: AuthToken) -> impl IntoResponse {
    // Get all the options available in Komga

    let labels = match state.komga.get_sharing_labels().await {
        Ok(labels) => labels,
        Err(_) => {
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", "application/json".parse().unwrap());

            // wrap the json in a {"ok": true, "data": {}} object
            let wrapped_json: Value = serde_json::json!({
                "ok": false,
                "error": "Failed to get labels from Komga"
            });

            return (headers, serde_json::to_string(&wrapped_json).unwrap());
        }
    };
    let libraries = match state.komga.get_libraries().await {
        Ok(libraries) => libraries,
        Err(_) => {
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", "application/json".parse().unwrap());

            // wrap the json in a {"ok": true, "data": {}} object
            let wrapped_json: Value = serde_json::json!({
                "ok": false,
                "error": "Failed to get libraries from Komga"
            });

            return (headers, serde_json::to_string(&wrapped_json).unwrap());
        }
    };

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    // wrap the json in a {"ok": true, "data": {}} object
    let wrapped_json: Value = serde_json::json!({
        "ok": true,
        "data": {
            "labels": labels,
            "libraries": libraries
        }
    });

    (headers, serde_json::to_string(&wrapped_json).unwrap())
}

async fn remove_token_or(redis_conn: &mut Connection, token: &InviteToken) -> Result<(), ()> {
    let current_unix: u64 = chrono::Utc::now().timestamp() as u64;

    match token.option.expire_at {
        Some(expire_at) => {
            if current_unix > expire_at {
                redis_conn
                    .del::<String, String>(format!("librarian:invite:{}", token.token))
                    .await
                    .unwrap_or("".to_string());
                Err(())
            } else {
                Ok(())
            }
        }
        None => Ok(()),
    }
}

pub async fn get_invite_token(
    State(state): State<AppState>,
    query: Query<InviteQuery>,
) -> impl IntoResponse {
    let mut redis_conn = state.redis.get_async_connection().await.unwrap();

    let data = redis_conn
        .get::<String, String>(format!("librarian:invite:{}", query.token))
        .await;

    match data {
        Ok(data) => {
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", "application/json".parse().unwrap());

            let raw_val: InviteToken = serde_json::from_str(&data).unwrap();

            match remove_token_or(&mut redis_conn, &raw_val).await {
                Ok(_) => {
                    // wrap the json in a {"ok": true, "data": {}} object
                    let wrapped_json: Value = serde_json::json!({
                        "ok": true,
                        "data": raw_val,
                    });

                    (headers, serde_json::to_string(&wrapped_json).unwrap())
                }
                Err(_) => {
                    // wrap the json in a {"ok": true, "data": {}} object
                    let wrapped_json: Value = serde_json::json!({
                        "ok": false,
                        "error": "Invite token expired"
                    });

                    return (headers, serde_json::to_string(&wrapped_json).unwrap());
                }
            }
        }
        Err(_) => {
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", "application/json".parse().unwrap());

            // wrap the json in a {"ok": true, "data": {}} object
            let wrapped_json: Value = serde_json::json!({
                "ok": false,
                "error": "Invite token not found"
            });

            (headers, serde_json::to_string(&wrapped_json).unwrap())
        }
    }
}

pub async fn get_all_invite_token(
    _: AuthToken,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let mut redis_conn = state.redis.get_async_connection().await.unwrap();

    let all_keys = redis_conn
        .keys::<&str, Vec<String>>("librarian:invite:*")
        .await
        .unwrap_or(vec![]);

    let mut merged_token = vec![];
    for key in all_keys {
        let data: String = redis_conn.get(key).await.unwrap();

        let raw_val: InviteToken = serde_json::from_str(&data).unwrap();

        match remove_token_or(&mut redis_conn, &raw_val).await {
            Ok(_) => {
                merged_token.push(raw_val);
            }
            Err(_) => {}
        }
    }

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    // wrap the json in a {"ok": true, "data": {}} object
    let wrapped_json: Value = serde_json::json!({
        "ok": true,
        "data": merged_token,
    });

    (headers, serde_json::to_string(&wrapped_json).unwrap())
}

pub fn invite_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", axum::routing::get(get_invite_token))
        .route("/all", axum::routing::get(get_all_invite_token))
        .route("/create", axum::routing::post(create_invite_token))
        .route("/config", axum::routing::get(get_invite_config))
        .with_state(state)
}
