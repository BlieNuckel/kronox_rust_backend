use crate::services::mongo_connectors::ScheduleConnector;
use rocket::Route;

#[get("/schedules/<id>?<school>&<start_tag>")]
async fn get_schedule(id: String, school: String, start_tag: String) -> String {
    let schedule_db_result = ScheduleConnector::build().await;
    let schedule_db = match schedule_db_result {
        Ok(schedule_db) => schedule_db,
        Err(error) => panic!("Couldn't get db {error}"),
    };

    let schedule_result = schedule_db.get_schedule(id).await;
    let schedule = match schedule_result {
        Ok(schedule) => schedule,
        Err(error) => panic!("Couldn't find schedule {error}"),
    };

    return match schedule {
        Some(schedule) => schedule._id.schedule_id,
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
