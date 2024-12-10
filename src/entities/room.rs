use crate::entities::user::{User, UserRole};

#[derive(Debug, Clone)]
pub struct Room {
    pub room_name: String,
    pub participants: Vec<User>,
}

impl Room {
    // 参加者の数を取得
    pub fn participant_count(&self) -> usize {
        self.participants.len()
    }

    pub fn add_participant(&mut self, user: User) -> Result<(), String> {
        // 役割ごとの制限
        let max_count = match user.role {
            UserRole::Participant => 2,
            UserRole::Observer => 8,
        };

        // 役割ごとの参加者数を確認
        let count_by_role = self
            .participants
            .iter()
            .filter(|u| u.role == user.role)
            .count();
        if count_by_role >= max_count {
            return Err(format!(
                "Cannot add user with ID {}. The room already has the maximum number of {}.",
                user.id, user.role
            ));
        }

        // 既存のユーザーが部屋に参加しているか確認
        if self.participants.iter().any(|u| u.id == user.id) {
            return Err("User already exists in the room.".to_string());
        }

        // ユーザーを部屋に追加
        self.participants.push(user);
        Ok(())
    }

    pub fn list_participants(&self) -> String {
        if self.participants.is_empty() {
            return "No participants in this room.".to_string();
        }
        let participant_names: Vec<String> =
            self.participants.iter().map(|u| u.name.clone()).collect();
        format!(
            "{} has {} participant(s): {}",
            self.room_name,
            self.participant_count(),
            participant_names.join(", ")
        )
    }
}

impl PartialEq for Room {
    fn eq(&self, other: &Self) -> bool {
        self.room_name == other.room_name
    }
}

impl Eq for Room {}
