// fn main() {
//     println!("Hello, world!");
// }
#[allow(unused_imports)]
use log::{error, warn, info, debug, trace};
#[allow(unused_imports)]
use anyhow::{Result, Error, bail, anyhow};

// use std::fs;
use std::fs::File;
use std::io::BufReader;
// use std::io::prelude::*;

use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct Item {
    pub also_view: Option<Vec<String>>,
    pub asin: String,
    pub brand: Option<String>,
    pub category: Vec<String>,
    pub description: Option<Vec<String>>,
    pub title: Option<String>,
    pub main_cat: Option<String>,
    pub price: Option<String>,
    // pub rank: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Simple {
    pub also_view: Option<String>,
    pub asin: String,
    // pub itemid: Option<u64>,
    pub brand: Option<String>,
    pub category: String,
    pub description: Option<String>,
    pub title: Option<String>,
    pub main_cat: Option<String>,
    pub price: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Normalized {
    pub asin: String,
    // pub itemid: Option<u64>,
    pub brand: Option<String>,
    pub description: Option<String>,
    pub title: Option<String>,
    pub main_cat: Option<String>,
    pub price: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Category {
    pub asin: String,
    // pub itemid: Option<u64>,
    pub category: String,
}

#[derive(Serialize, Deserialize)]
pub struct AlsoView {
    pub asin: String,
    // pub itemid: Option<u64>,
    pub also_view: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    pub overall: String,
    pub verified: String,
    pub review_time: String,
    pub asin: String,
    pub reviewer_name: String,
    pub review_text: String,
    pub summary: String,
    pub unix_review_time: String,
    pub vote: Option<usize>,
    pub style: Option<String>,
    pub image: Option<String>,
    pub userid: u64,
    pub itemid: u64,
    pub rating: f32,
}

// use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init_timed();

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "warn");
    }

    // let filepath = "/data/train.csv";
    // let file = File::open(filepath)
    //     .expect("could not open file");
    // let buffered_reader = BufReader::new(file);
    // let mut rdr = csv::Reader::from_reader(buffered_reader);
    // let mut asin2itemid: HashMap<String, u64> = HashMap::new();
    // for result in rdr.deserialize() {
    //     // Notice that we need to provide a type hint for automatic
    //     // deserialization.
    //     let record: Record = result?;
    //     asin2itemid.insert(record.asin, record.itemid);
    // }
    // println!("asin2itemid: {}", asin2itemid.len());
    // return Ok(());
    // let deserializer = serde_json::Deserializer::from_reader(buffered_reader);

    let filepath = "/data/meta_Grocery_and_Gourmet_Food.json";
    let file = File::open(filepath)
        .expect("could not open file");
    let buffered_reader = BufReader::new(file);
    let deserializer = serde_json::Deserializer::from_reader(buffered_reader);
    
    let iterator = deserializer.into_iter::<Item>();
    let start = std::time::Instant::now();
    let mut vec_simple: Vec<Simple> = vec![];
    let mut vec_normalized: Vec<Normalized> = vec![];
    let mut vec_category: Vec<Category> = vec![];
    let mut vec_also_view: Vec<AlsoView> = vec![];

    use indicatif::{ProgressBar, ProgressStyle};
    let pbar = ProgressBar::new(300000);
    pbar.set_style(
        ProgressStyle::default_bar()
            .template("{eta_precise} [{bar:60.cyan/blue}] {wide_msg}")
            .progress_chars("=>-"),
    );
    pbar.enable_steady_tick(125);

    for (i, item) in iterator.enumerate() {
        pbar.set_message(&format!("{}", i));
        let item = item?;
        let description = match item.description {
            None => None,
            Some(vec_string) => Some(vec_string.join("\n\n")),
        };
        // let itemid = asin2itemid.get(&item.asin);
        let normalized = Normalized {
            asin: item.asin.clone(),
            // itemid: itemid.copied(),
            brand: item.brand.clone(),
            description: description.clone(),
            title: item.title.clone(),
            main_cat: item.main_cat.clone(),
            price: item.price.clone(),
        };
        for category in &item.category {
            vec_category.push(Category {
                asin: item.asin.clone(),
                // itemid: itemid.copied(),
                category: category.clone(),
            });
        }
        if let Some(vec_string) = &item.also_view {
            for s in vec_string {
                vec_also_view.push(AlsoView {
                    asin: item.asin.clone(),
                    // itemid: itemid.copied(),
                    also_view: s.clone(),
                });
            }
        }
        let simple = Simple {
            also_view: match item.also_view {
                None => None,
                Some(vec_string) => Some(vec_string.join("|")),
            },
            asin: item.asin.clone(),
            // itemid: itemid.copied(),
            brand: item.brand.clone(),
            category: item.category.join("|"),
            description: description.clone(),
            title: item.title.clone(),
            main_cat: item.main_cat.clone(),
            price: item.price.clone(),
        };
        vec_simple.push(simple);

        vec_normalized.push(normalized);
        pbar.set_position(i as u64);
    }
    pbar.finish();

    println!("{}, simple: {}, normalized: {}, category: {}, also_view: {}", arrange_millis::get(std::time::Instant::now().duration_since(start).as_millis()), vec_simple.len(), vec_normalized.len(), vec_category.len(), vec_also_view.len());


    let start = std::time::Instant::now();

    let mut wtr = csv::Writer::from_path("/data/json.csv")?;
    for record in vec_simple {
        wtr.serialize(record)?;
    }
    wtr.flush()?;

    let mut wtr = csv::Writer::from_path("/data/normalized.csv")?;
    for record in vec_normalized {
        wtr.serialize(record)?;
    }
    wtr.flush()?;

    let mut wtr = csv::Writer::from_path("/data/category.csv")?;
    for record in vec_category {
        wtr.serialize(record)?;
    }
    wtr.flush()?;

    let mut wtr = csv::Writer::from_path("/data/also_view.csv")?;
    for record in vec_also_view {
        wtr.serialize(record)?;
    }
    wtr.flush()?;

    println!("{}", arrange_millis::get(std::time::Instant::now().duration_since(start).as_millis()));

    Ok(())
}
