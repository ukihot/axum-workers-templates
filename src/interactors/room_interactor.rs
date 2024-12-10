use crate::dtos::{join_response::JoinResponse, room_object::RoomObject, user_object::UserObject};
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
    async fn participate(&self, room: RoomObject, user: UserObject) -> Json<JoinResponse> {
        let room_code = &room.code;
        let user_id = &user.id;

        // ルームを取得または新規作成
        let room = self
            .repository
            .get_room(room_code)
            .unwrap_or_else(|| self.repository.create_room(room_code));

        // 部屋に何人いるか調査
        let participant_count = room.lock().unwrap().participant_count();

        // Userオブジェクトの生成
        let user = if participant_count == 0 || participant_count == 1 {
            User::new_participant(user_id.clone(), None)
        } else {
            User::new_observer(user_id.clone(), None)
        };

        // Userの保存
        if let Err(err) = self.repository.save(&room, &user) {
            return Json(JoinResponse {
                message: format!("Error: {}", err),
            });
        }
        let participants = room.lock().unwrap().list_participants();

        // レスポンスメッセージを生成
        let response_message = format!("Hello {}! {}", user.name, participants);

        Json(JoinResponse {
            message: response_message,
        })
    }

    async fn get_all_room(&self) -> Json<JoinResponse> {
        // リポジトリから全ての部屋情報を取得
        let all_rooms = self.repository.get_all_repository();

        // 部屋の数を取得
        let room_count = all_rooms.0["rooms"].as_array().unwrap_or(&vec![]).len();
        let ver = self.repository.get_version();

        // JSON形式の全部屋情報をレスポンスメッセージとして組み立てる
        let response_message = format!(
            "There are {} rooms in {}. Details: {}",
            room_count,
            ver,
            all_rooms.0 // JSONデータを文字列として埋め込む
        );

        Json(JoinResponse {
            message: response_message,
        })
    }
}
