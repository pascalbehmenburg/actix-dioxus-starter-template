pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub is_done: bool,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub struct CreateTodo {
    pub title: String,
    pub description: String,
}
