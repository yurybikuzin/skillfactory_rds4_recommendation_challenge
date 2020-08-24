use diesel::prelude::*;
use dotenv::dotenv;
// use std::env;

mod models;
mod schema;

pub use models::{NewDicImage, NewDicReviewerName, NewDicSummary, NewImage, NewTrain};
pub use schema::{dic_image, dic_reviewer_name, dic_summary, image, train};
// use schema::train::dsl::*;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_url = "../flask/data/flask.db";
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
