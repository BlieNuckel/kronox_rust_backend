use crate::models::Schedule;
use futures::TryStreamExt;
use mongodb::{bson::doc, options::ClientOptions, Client, Collection, Cursor};
use std::{env, error::Error};

static DB_NAME: &str = "test_db";
static SCHEDULE_COLL_NAME: &str = "schedules";
static mut SCHEDULE_CONNECTOR: Option<ScheduleConnector> = None;

pub struct ScheduleConnector {
    collection: Collection<Schedule>,
}

impl ScheduleConnector {
    pub async fn build() -> Result<&'static ScheduleConnector, Box<dyn Error + Send + Sync>> {
        if unsafe { SCHEDULE_CONNECTOR.is_some() } {
            return Ok(unsafe { &SCHEDULE_CONNECTOR.as_ref().unwrap() });
        }

        let client_options = ClientOptions::parse(env::var("mongoURI")?).await?;
        let client = Client::with_options(client_options)?;

        let db_handle = client.database(DB_NAME);

        let collection_handle = db_handle.collection::<Schedule>(SCHEDULE_COLL_NAME);
        unsafe {
            SCHEDULE_CONNECTOR = Some(ScheduleConnector {
                collection: collection_handle,
            });
        }

        return Ok(unsafe { &SCHEDULE_CONNECTOR.as_ref().unwrap() });
    }

    pub async fn get_schedules(&self) -> Result<Vec<Schedule>, Box<dyn Error + Send + Sync>> {
        let mut schedule_cursor: Cursor<Schedule> = self.collection.find(doc! {}, None).await?;
        let mut schedule_arr: Vec<Schedule> = Vec::new();

        while let Some(schedule) = schedule_cursor.try_next().await? {
            schedule_arr.push(schedule);
        }

        return Ok(schedule_arr);
    }

    pub async fn get_schedule(
        &self,
        id: String,
        start_tag: String,
    ) -> Result<Option<Schedule>, Box<dyn Error>> {
        return Ok(self
            .collection
            .find_one(
                doc! { "_id": {"scheduleId": id, "startsAt": start_tag} },
                None,
            )
            .await?);
    }

    // pub async fn set_schedule(&self, schedule: Schedule) -> () {
    //     self.collection.find_one_and
    // }
}
