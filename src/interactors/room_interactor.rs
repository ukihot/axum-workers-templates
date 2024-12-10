use crate::dtos::{join_request::JoinRequest, join_response::JoinResponse};
use crate::entities::user::User;
use crate::input_ports::room_input_ports::RoomInputPort;
use crate::repositories::room_repository::RoomRepository;
use axum::Json;
pub struct RoomInteractor<R: RoomRepository> {
    repository: R,
}

impl<R: RoomRepository> RoomInteractor<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R: RoomRepository> RoomInputPort for RoomInteractor<R> {
    async fn participate(&self, req: JoinRequest) -> Json<JoinResponse> {
        let room = req.room.as_ref().expect("Room ID must be provided");
        let user_id = &req.user_id;

        // 部屋の参加者数を取得
        let participant_count = self.repository.get_participant_count(room);

        if participant_count >= 2 {
            // Observer
            let user = User::new_observer(req.user_id.clone(), None);
            self.repository.save(room, &user);
            Json(JoinResponse {
                message: format!("Hello {}! You are an observer.", user.name),
            })
        } else {
            // Participant
            let user = User::new_participant(req.user_id.clone(), None);
            self.repository.save(room, &user);
            Json(JoinResponse {
                message: format!("Hello {}!", user.name),
            })
        }
    }
}
