use super::schema::{dic_image, dic_reviewer_name, dic_summary, image, train};
// use serde::Serialize;

// #[derive(Serialize, Queryable)]
// struct Train {
//     pub id: i32,
//     pub userid: i32,
//     pub itemid: i32,
//     pub rating: i32,
//     pub overall: i32,
//     pub verified: i32,
//     pub unix_review_time: i32,
//     pub reviewer_name_id: Option<i32>,
//     pub summary_id: Option<i32>,
//     pub vote: Option<i32>,
//     // pub style: Option<String>,
//     // pub image: Option<String>,
// }

#[derive(Insertable)]
#[table_name = "train"]
pub struct NewTrain<'a> {
    pub id: &'a i32,
    pub userid: &'a i32,
    pub itemid: &'a i32,
    pub rating: &'a i32,
    pub overall: &'a i32,
    pub verified: &'a i32,
    pub unix_review_time: &'a i32,
    pub reviewer_name_id: Option<&'a i32>,
    pub summary_id: Option<&'a i32>,
    pub vote: Option<&'a i32>,
    // pub style: Option<String>,
    // pub image: Option<String>,
}

#[derive(Insertable)]
#[table_name = "image"]
pub struct NewImage<'a> {
    pub id: &'a i32,
    pub train_id: &'a i32,
    pub image_id: &'a i32,
}

#[derive(Insertable)]
#[table_name = "dic_reviewer_name"]
pub struct NewDicReviewerName<'a> {
    pub id: &'a i32,
    pub value: &'a str,
}

#[derive(Insertable)]
#[table_name = "dic_summary"]
pub struct NewDicSummary<'a> {
    pub id: &'a i32,
    pub value: &'a str,
}

#[derive(Insertable)]
#[table_name = "dic_image"]
pub struct NewDicImage<'a> {
    pub id: &'a i32,
    pub value: &'a str,
}
