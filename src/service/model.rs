use chrono::{DateTime, Utc};
use tokio_postgres::types::Timestamp;

#[derive(Default, Debug)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub current_position: String,
    pub age: i8,
    pub experience_years: i8,
    pub biography: String,
    pub icon: Vec<u8>,
    pub data: UserInfoData,
    pub first_name: String,
    pub last_name: String,
    pub created_date: DateTime<Utc>
}

pub struct UserInfoData {
    pub skills: Vec<String>,
}