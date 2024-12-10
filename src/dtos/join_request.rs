use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct JoinRequest {
    pub user_id: String,
    pub room_code: String,
}
