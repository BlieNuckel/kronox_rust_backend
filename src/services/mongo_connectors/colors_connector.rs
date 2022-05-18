use mongodb::{bson::Document, options::ClientOptions, Client, Collection};
use std::{env, error::Error};

use crate::models::CourseColor;

static DB_NAME: &str = "test_db";
static COLOR_COLL_NAME: &str = "course_colors";

pub struct ColorConnector {
    collection: Collection<CourseColor>,
}

impl ColorConnector {
    async fn build() -> Result<ColorConnector, Box<dyn Error>> {
        let client_options = ClientOptions::parse(env::var("mongoURI")?).await?;
        let client = Client::with_options(client_options)?;

        let db_handle = client.database(DB_NAME);

        let collection_handle = db_handle.collection::<CourseColor>(COLOR_COLL_NAME);

        Ok(ColorConnector {
            collection: collection_handle,
        })
    }
}
