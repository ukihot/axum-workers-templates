use crate::controllers::room_controller::RoomController;
use crate::dtos::{join_request::JoinRequest, join_response::JoinResponse};
use crate::interactors::room_interactor::RoomInteractor;
use crate::repositories::room_repository::RoomRepository;
use crate::repositories::room_repository::RoomRepositoryImpl;
use axum::{extract::State, Json};

#[axum::debug_handler]
pub async fn handle_room(
    State(repository): State<RoomRepositoryImpl>,
    Json(req): Json<JoinRequest>,
) -> Json<JoinResponse> {
    let controller = create_controller(repository);
    controller.handle_join_request(req).await
}

#[axum::debug_handler]
pub async fn handle_current_rooms(
    State(repository): State<RoomRepositoryImpl>,
) -> Json<JoinResponse> {
    let controller = create_controller(repository);
    controller.handle_current_status().await
}

fn create_controller<R: RoomRepository>(repository: R) -> RoomController<RoomInteractor<R>> {
    let interactor = RoomInteractor::new(repository);
    RoomController::new(interactor)
}
