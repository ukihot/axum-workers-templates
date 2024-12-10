use crate::dtos::room_object::RoomObject;
use crate::dtos::user_object::UserObject;
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
        let user = UserObject { id: req.user_id };
        let room = RoomObject {
            code: req.room_code,
        };

        self.input_port.participate(room, user).await
    }

    pub async fn handle_current_status(&self) -> Json<JoinResponse> {
        self.input_port.get_all_room().await
    }
}
