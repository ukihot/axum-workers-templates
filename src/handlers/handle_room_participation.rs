use crate::controllers::room_controller::RoomController;
use crate::dtos::{join_request::JoinRequest, join_response::JoinResponse};
use crate::interactors::room_interactor::RoomInteractor;
use crate::repositories::room_repository::RoomRepositoryImpl;
use axum::Json;

pub async fn handle_room(Json(req): Json<JoinRequest>) -> Json<JoinResponse> {
    let controller = create_room_controller();
    controller.handle_join_request(req).await
}

fn create_room_controller() -> RoomController<RoomInteractor<RoomRepositoryImpl>> {
    let repository = RoomRepositoryImpl::new();
    let interactor = RoomInteractor::new(repository);
    RoomController::new(interactor)
}
