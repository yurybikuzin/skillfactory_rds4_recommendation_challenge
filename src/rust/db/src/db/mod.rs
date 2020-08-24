use diesel::prelude::*;
use dotenv::dotenv;
// use std::env;

mod models;
mod schema;

pub use models::{
    NewAlsoView, NewCategory, NewDicBrand, NewDicCategory, NewDicDescription, NewDicImage,
    NewDicMainCat, NewDicReviewerName, NewDicSummary, NewDicTitle, NewImage, NewItem,
    NewItemidAsin, NewTrain,
};
pub use schema::{
    also_view, category, dic_brand, dic_category, dic_description, dic_image, dic_main_cat,
    dic_reviewer_name, dic_summary, dic_title, image, item, itemid_asin, train,
};
// use schema::train::dsl::*;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_url = "../flask/data/flask.db";
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
