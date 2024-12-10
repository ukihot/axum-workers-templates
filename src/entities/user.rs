use rand::Rng;
use std::fmt;

#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub role: UserRole,
}

#[derive(Debug, Clone, Default)]
pub enum UserRole {
    #[default]
    Participant,
    Observer,
}

impl User {
    // Participantとして生成
    pub fn new_participant(id: String, name: Option<String>) -> Self {
        Self {
            id,
            name: name.unwrap_or_else(User::generate_random_name),
            role: UserRole::Participant,
        }
    }

    // Observerとして生成
    pub fn new_observer(id: String, name: Option<String>) -> Self {
        Self {
            id,
            name: name.unwrap_or_else(User::generate_random_name),
            role: UserRole::Observer,
        }
    }

    // 名前をランダムに生成する関数
    fn generate_random_name() -> String {
        let mut rng = rand::thread_rng();
        (0..4)
            .map(|_| {
                // 'A'から'Z'までのランダムな文字を生成
                (rng.gen_range(b'A'..=b'Z')) as char
            })
            .collect()
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for User {}

impl fmt::Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let role_str = match self {
            UserRole::Participant => "Participant",
            UserRole::Observer => "Observer",
        };
        write!(f, "{}", role_str)
    }
}

impl PartialEq for UserRole {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (UserRole::Participant, UserRole::Participant)
                | (UserRole::Observer, UserRole::Observer)
        )
    }
}

impl Eq for UserRole {}
