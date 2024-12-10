use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct JoinResponse {
    pub message: String,
}
