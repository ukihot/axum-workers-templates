use crate::entities::user::User;

#[derive(Debug, Clone)]
pub struct Room {
    pub id: String,
    pub participants: Vec<User>,
}

impl Room {
    // 参加者の数を取得するメソッド
    pub fn participant_count(&self) -> usize {
        self.participants.len()
    }
}
