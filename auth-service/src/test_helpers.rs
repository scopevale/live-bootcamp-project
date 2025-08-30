use uuid::Uuid;

pub fn get_random_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}
