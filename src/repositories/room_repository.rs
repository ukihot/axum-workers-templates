use crate::entities::room::Room;
use crate::entities::user::User;
use axum::Json;
use dashmap::DashMap;
use rand::{distributions::Alphanumeric, Rng};
use serde_json::json;
use std::sync::{Arc, Mutex};

pub trait RoomRepository {
    fn save(&self, room: &Arc<Mutex<Room>>, user: &User) -> Result<(), String>;
    fn get_room(&self, room_code: &str) -> Option<Arc<Mutex<Room>>>;
    fn create_room(&self, room_code: &str) -> Arc<Mutex<Room>>;
    fn get_version(&self) -> String;
    fn get_all_repository(&self) -> Json<serde_json::Value>;
}

#[derive(Clone)]
pub struct RoomRepositoryImpl {
    state: Arc<DashMap<usize, Arc<Mutex<Room>>>>,
    ver: String,
}

impl RoomRepositoryImpl {
    pub fn new() -> Self {
        let random_version: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect();

        Self {
            state: Arc::new(DashMap::<usize, Arc<Mutex<Room>>>::new()),
            ver: random_version,
        }
    }

    fn generate_unique_id(&self) -> usize {
        loop {
            let id = rand::thread_rng().gen::<usize>();
            if !self.state.contains_key(&id) {
                return id;
            }
        }
    }
}

impl RoomRepository for RoomRepositoryImpl {
    fn save(&self, room: &Arc<Mutex<Room>>, user: &User) -> Result<(), String> {
        // `room`のロックを取得
        let room_name = room
            .lock()
            .map_err(|e| format!("Failed to lock room: {:?}", e))?
            .room_name
            .clone();

        // `state`内に既存の部屋を検索し、存在しない場合は早期リターン
        let entry = self
            .state
            .iter()
            .find(|entry| matches!(entry.value().lock(), Ok(guard) if guard.room_name == room_name))
            .ok_or_else(|| format!("Room with name '{}' does not exist.", room_name))?;

        // 既存の部屋をロックして参加者を追加
        let mut existing_room = entry
            .value()
            .lock()
            .map_err(|e| format!("Failed to lock room: {:?}", e))?;
        existing_room.add_participant(user.clone())?;

        Ok(())
    }

    fn get_room(&self, room_code: &str) -> Option<Arc<Mutex<Room>>> {
        self.state.iter().find_map(|entry| {
            if entry.value().lock().unwrap().room_name == room_code {
                Some(Arc::clone(entry.value()))
            } else {
                None
            }
        })
    }

    fn create_room(&self, room_code: &str) -> Arc<Mutex<Room>> {
        let room_id = self.generate_unique_id();
        let new_room = Arc::new(Mutex::new(Room {
            room_name: room_code.to_owned(),
            participants: Vec::new(),
        }));

        self.state.insert(room_id, Arc::clone(&new_room));
        new_room
    }

    fn get_version(&self) -> String {
        self.ver.clone()
    }

    fn get_all_repository(&self) -> Json<serde_json::Value> {
        let rooms: Vec<_> = self
            .state
            .iter()
            .map(|entry| {
                let room = entry.value().lock().unwrap();
                let participants: Vec<_> = room
                    .participants
                    .iter()
                    .map(|user| {
                        json!({
                            "id": user.id,
                            "name": user.name,
                            "role": user.role.to_string()
                        })
                    })
                    .collect();

                json!({
                    "room_id": room.room_name,
                    "participants": participants,
                })
            })
            .collect();

        Json(json!({
            "rooms": rooms
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::user::User;
    use std::sync::Arc;

    #[test]
    fn test_add_user_to_room_success() {
        let repo = RoomRepositoryImpl::new();
        let room_code = "TestRoom";
        let room = repo.create_room(room_code);
        let user = User::new_participant("user1".to_string(), None);

        // 部屋にユーザーを追加
        assert!(repo.save(&room, &user).is_ok());

        // ユーザーが部屋に追加されていることを確認
        let room = repo.get_room(room_code).unwrap();
        let locked_room = room.lock().unwrap();
        assert_eq!(locked_room.participant_count(), 1);
    }

    #[test]
    fn test_add_existing_user_to_room() {
        let repo = RoomRepositoryImpl::new();
        let room_code = "TestRoom";
        let room = repo.create_room(room_code);
        let user = User::new_participant("user1".to_string(), None);

        // 初回の追加は成功するはず
        assert!(repo.save(&room, &user).is_ok());

        // 同じユーザーを再度追加しようとするとエラーになるはず
        assert!(repo.save(&room, &user).is_err());
    }

    #[test]
    fn test_add_user_exceeding_role_limit() {
        let repo = RoomRepositoryImpl::new();
        let room_code = "TestRoom";
        let room = repo.create_room(room_code);

        // 役割ごとの最大人数制限を満たすために2人のユーザーを追加
        let user1 = User::new_participant("user1".to_string(), None);
        let user2 = User::new_participant("user2".to_string(), None);
        assert!(repo.save(&room, &user1).is_ok());
        assert!(repo.save(&room, &user2).is_ok());

        // 3人目のユーザー追加を試みるとエラーが発生することを確認
        let user3 = User::new_participant("user3".to_string(), None);
        assert!(repo.save(&room, &user3).is_err());
    }

    #[test]
    fn test_save_room_not_found() {
        let repo = RoomRepositoryImpl::new();
        let room_code = "NonExistentRoom";
        let room = Arc::new(Mutex::new(Room {
            room_name: room_code.to_string(),
            participants: Vec::new(),
        }));

        let user = User::new_participant("user1".to_string(), None);

        // 部屋が存在しない場合のエラーを確認
        assert!(repo.save(&room, &user).is_err());
    }

    #[test]
    fn test_get_all_repository() {
        let repo = RoomRepositoryImpl::new();
        let room_code = "TestRoom1";
        let room1 = repo.create_room(room_code);
        let user1 = User::new_participant("user1".to_string(), None);
        repo.save(&room1, &user1).unwrap();

        let room_code2 = "TestRoom2";
        let room2 = repo.create_room(room_code2);
        let user2 = User::new_participant("user2".to_string(), None);
        repo.save(&room2, &user2).unwrap();

        // get_all_repository の結果を確認
        let json = repo.get_all_repository();
        let rooms = json.0["rooms"].as_array().unwrap();
        assert_eq!(rooms.len(), 2);
    }

    #[test]
    fn test_add_observer_to_room() {
        let repo = RoomRepositoryImpl::new();
        let room_code = "TestRoom";
        let room = repo.create_room(room_code);

        // 役割ごとの最大人数制限を満たすために2人のユーザーを追加
        let user1 = User::new_participant("user1".to_string(), None);
        let user2 = User::new_participant("user2".to_string(), None);
        assert!(repo.save(&room, &user1).is_ok());
        assert!(repo.save(&room, &user2).is_ok());

        // `Observer`を追加
        let observer = User::new_observer("observer1".to_string(), None);
        assert!(repo.save(&room, &observer).is_ok());

        // 追加された`Observer`が部屋に存在することを確認
        let room = repo.get_room(room_code).unwrap();
        let locked_room = room.lock().unwrap();
        assert_eq!(locked_room.participant_count(), 3);
    }
}
