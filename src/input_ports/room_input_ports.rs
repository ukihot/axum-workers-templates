use crate::dtos::{join_request::JoinRequest, join_response::JoinResponse};
use axum::Json;

pub trait RoomInputPort {
    async fn participate(&self, req: JoinRequest) -> Json<JoinResponse>;
}
