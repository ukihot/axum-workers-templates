use crate::entities::room::Room;
use crate::entities::user::User;
use dashmap::DashMap;
use std::sync::Arc;

pub trait RoomRepository {
    fn save(&self, room_id: &str, user: &User);
    fn get_participant_count(&self, room_id: &str) -> usize;
    fn get_room(&self, room_id: &str) -> Option<Arc<Room>>;
}

pub struct RoomRepositoryImpl {
    state: Arc<DashMap<String, Arc<Room>>>,
}

impl RoomRepositoryImpl {
    pub fn new() -> Self {
        Self {
            state: Arc::new(DashMap::new()),
        }
    }
}

impl RoomRepository for RoomRepositoryImpl {
    fn save(&self, room_id: &str, user: &User) {
        let mut room = self.state.entry(room_id.to_string()).or_insert_with(|| {
            Arc::new(Room {
                id: room_id.to_string(),
                participants: Vec::new(),
            })
        });

        Arc::make_mut(&mut room).participants.push(user.clone());
    }

    fn get_participant_count(&self, room_id: &str) -> usize {
        self.state
            .get(room_id)
            .map_or(0, |room| room.participant_count())
    }

    fn get_room(&self, room_id: &str) -> Option<Arc<Room>> {
        self.state.get(room_id).map(|room| Arc::clone(&room))
    }
}
