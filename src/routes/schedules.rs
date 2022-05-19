use crate::services::mongo_connectors::ScheduleConnector;
use rocket::{response::content, Route};
use serde_json;

#[allow(non_snake_case)]
#[get("/schedules/<id>?<school>&<startTag>")]
async fn get_schedule(id: String, school: String, startTag: String) -> content::Json<String> {
    let schedule_db_result = ScheduleConnector::build().await;
    let schedule_db = match schedule_db_result {
        Ok(schedule_db) => schedule_db,
        Err(error) => panic!("Couldn't get db {error}"),
    };

    let schedule_result = schedule_db.get_schedule(id, startTag).await;
    let schedule = match schedule_result {
        Ok(schedule) => schedule,
        Err(error) => panic!("Couldn't find schedule {error}"),
    };

    return match schedule {
        Some(schedule) => content::Json(serde_json::to_string(&schedule).unwrap()),
        None => panic!("Couldn't find schedule"),
    };
}

#[post("/schedules/search?<school>")]
fn search_schedules(school: String) -> String {
    return format!("school: {school}");
}

pub fn schedule_routes() -> Vec<Route> {
    return rocket::routes![get_schedule, search_schedules];
}
