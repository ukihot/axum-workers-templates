use crate::entities::room::Room;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct JoinRequest {
    pub user_id: String,
    pub room: Option<String>,
}

impl From<JoinRequest> for Room {
    fn from(req: JoinRequest) -> Self {
        Room {
            id: req.room.unwrap(),
            participants: Vec::new(),
        }
    }
}
