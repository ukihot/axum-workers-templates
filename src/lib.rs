use axum::{http::Response, routing::post, Json, Router};
use once_cell::sync::Lazy;
use reqwest::header::{HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tower_http::cors::{Any, CorsLayer};
use tower_service::Service;
use worker::*;

type RoomStateType = Arc<Mutex<HashMap<String, Vec<String>>>>;

#[derive(Debug, Deserialize)]
pub struct JoinRequest {
    pub user_id: String,
    pub room: String,
}

#[derive(Debug, Serialize)]
pub struct JoinResponse {
    pub message: String,
}

static ROOM_STATE: Lazy<RoomStateType> = Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

fn router() -> Router {
    let origins = [
        "https://bug-free-palm-tree-7qjgqxgq6472www9-5173.app.github.dev"
            .parse::<HeaderValue>()
            .unwrap(),
    ];
    Router::new().route("/", post(join_game)).layer(
        CorsLayer::new()
            .allow_methods(Any)
            .allow_origin(origins)
            .allow_headers([HeaderName::from_static("content-type")]),
    )
}

#[event(fetch)]
async fn fetch(req: HttpRequest, _env: Env, _ctx: Context) -> Result<Response<axum::body::Body>> {
    console_error_panic_hook::set_once();
    Ok(router().call(req).await?)
}

pub async fn join_game(Json(req): Json<JoinRequest>) -> Json<JoinResponse> {
    let room = &req.room;
    let user_id = &req.user_id;

    let mut room_state = ROOM_STATE.lock().unwrap();

    if !room_state.contains_key(room) {
        room_state.insert(room.to_string(), Vec::new());
    }

    let participants = room_state.get_mut(room).unwrap();

    if participants.len() < 2 {
        participants.push(user_id.clone());
        Json(JoinResponse {
            message: format!("Hello {}!", user_id),
        })
    } else {
        Json(JoinResponse {
            message: format!("Hello {}! You are an observer.", user_id),
        })
    }
}
