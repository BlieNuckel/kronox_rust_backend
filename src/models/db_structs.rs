use mongodb::bson::Bson;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CourseColor {
    pub _id: String,
    pub color: String,
}

#[derive(Deserialize, Serialize)]
pub struct Schedule {
    pub _id: ScheduleID,
    pub cached_at: String,
    pub base_url: String,
    pub starts_at: String,
    pub schedule: Bson,
    pub generated_uuids: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct ScheduleID {
    pub schedule_id: String,
    pub starts_at: String,
}
