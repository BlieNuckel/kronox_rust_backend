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
    pub cachedAt: String,
    pub baseUrl: String,
    pub startsAt: String,
    pub schedule: Bson,
    pub generated_uuids: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct ScheduleID {
    pub scheduleId: String,
    pub startsAt: String,
}
