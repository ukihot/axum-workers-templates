use crate::dtos::{join_response::JoinResponse, room_object::RoomObject, user_object::UserObject};
use axum::Json;

pub trait RoomInputPort {
    async fn participate(&self, room: RoomObject, user: UserObject) -> Json<JoinResponse>;

    async fn get_all_room(&self) -> Json<JoinResponse>;
}
