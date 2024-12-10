mod controllers;
mod dtos;
mod entities;
mod handlers;
mod input_ports;
mod interactors;
mod repositories;

use crate::handlers::handle_room_participation::handle_room;
use crate::repositories::room_repository::RoomRepositoryImpl;
use axum::{http::Response, routing::post, Router};
use handlers::handle_room_participation::handle_current_rooms;
use once_cell::sync::Lazy;
use reqwest::header::{HeaderName, HeaderValue};
use tower_http::cors::{Any, CorsLayer};
use tower_service::Service;
use worker::*;

static REPOSITORY: Lazy<RoomRepositoryImpl> = Lazy::new(RoomRepositoryImpl::new);

#[event(fetch)]
async fn fetch(req: HttpRequest, _env: Env, _ctx: Context) -> Result<Response<axum::body::Body>> {
    console_error_panic_hook::set_once();

    // CORS設定
    let origins = [
        "https://bug-free-palm-tree-7qjgqxgq6472www9-5173.app.github.dev"
            .parse::<HeaderValue>()
            .unwrap(),
    ];
    let cors_options = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(origins)
        .allow_headers([HeaderName::from_static("content-type")]);

    // アプリケーションのルーティング設定
    let mut app = Router::new()
        .route("/", post(handle_room))
        .route("/ll", post(handle_current_rooms))
        .layer(cors_options)
        .with_state(REPOSITORY.clone());

    Ok(app.call(req).await?)
}
