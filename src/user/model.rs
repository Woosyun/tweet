use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(rename="_id")]
    pub id: String,
    pub user_id: String,
    pub password: String,
    pub user_name: String,
    pub posts: Vec<String>,
}

impl User {
    pub fn new(user_id: String, password: String, user_name: String) -> Self {
        Self {
            id: String::new(),
            user_id,
            password,
            user_name,
            posts: vec![],
        }
    }
    pub fn set_id(&mut self, new_id: String) {
        self.id = new_id;
    }
}
