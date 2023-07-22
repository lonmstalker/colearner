use bytes::Bytes;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio_postgres::types::Timestamp;
use crate::service::enums::{PositionType, ProjectStatus, ProjectType};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub current_position: String,
    pub age: Option<i8>,
    pub experience_years: i8,
    pub biography: Option<String>,
    pub icon: Option<Bytes>,
    pub skills: Option<Vec<String>>,
    pub first_name: String,
    pub last_name: Option<String>,
    pub created_date: DateTime<Utc>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct UserProject {
    pub id: i32,
    pub telegram_id: String,
    pub name: String,
    pub description: String,
    pub achievements: Option<Vec<String>>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ProposedProject {
    pub id: i32,
    pub author_id: i32,
    pub title: String,
    pub description: String,
    pub status: ProjectStatus,
    pub p_type: ProjectType,
    pub created_date: DateTime<Utc>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ProposedProjectMember {
    pub id: i32,
    pub user_id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub work_week_hours: Option<u8>,
    pub position_type: PositionType,
    pub skills: Option<Vec<String>>,
    pub joined_date: DateTime<Utc>,
}
