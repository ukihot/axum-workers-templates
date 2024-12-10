mod controllers;
mod dtos;
mod entities;
mod handlers;
mod input_ports;
mod interactors;
mod repositories;

use crate::handlers::handle_room_participation::handle_room;
use axum::{http::Response, routing::post, Router};
use reqwest::header::{HeaderName, HeaderValue};
use tower_http::cors::{Any, CorsLayer};
use tower_service::Service;
use worker::*;

fn router() -> Router {
    let origins = [
        "https://bug-free-palm-tree-7qjgqxgq6472www9-5173.app.github.dev"
            .parse::<HeaderValue>()
            .unwrap(),
    ];
    let cors_options = CorsLayer::new()
            .allow_methods(Any)
            .allow_origin(origins)
            .allow_headers([HeaderName::from_static("content-type")]);

    Router::new().route("/", post(handle_room)).layer(cors_options)
}

#[event(fetch)]
async fn fetch(req: HttpRequest, _env: Env, _ctx: Context) -> Result<Response<axum::body::Body>> {
    console_error_panic_hook::set_once();
    Ok(router().call(req).await?)
}
