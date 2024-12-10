use crate::dtos::{join_request::JoinRequest, join_response::JoinResponse};
use crate::input_ports::room_input_ports::RoomInputPort;
use axum::Json;

pub struct RoomController<P: RoomInputPort> {
    input_port: P,
}

impl<P: RoomInputPort> RoomController<P> {
    pub fn new(input_port: P) -> Self {
        Self { input_port }
    }

    pub async fn handle_join_request(&self, req: JoinRequest) -> Json<JoinResponse> {
        self.input_port.participate(req).await
    }
}
