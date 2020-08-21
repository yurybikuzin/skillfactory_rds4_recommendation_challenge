// fn main() {
//     println!("Hello, world!");
// }
// #![recursion_limit="4096"]
#[allow(unused_imports)]
use anyhow::{anyhow, bail, Context, Error, Result};
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use std::path::PathBuf;
use structopt::StructOpt;
// use std::fs;
use json::Json;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
//
#[derive(Debug, StructOpt)]
struct Opt {
    /// Path to toml config file
    #[structopt(parse(from_os_str), default_value = "../../data")]
    data: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemIdAsin {
    pub itemid: usize,
    pub asin: String,
}

pub struct RecordItemIdAsin {
    pub itemid: u16,
    pub asin: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AlsoViewUsed {
    pub itemid: usize,
    pub also_view_itemid: usize,
    pub is_train: bool,
}

pub struct RecordAlsoView {
    pub itemid: u16,
    pub also_view_itemid: u16,
    pub is_train: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryUsed {
    pub itemid: usize,
    pub category: String,
    pub is_train: bool,
}

pub struct RecordCategory {
    pub itemid: u16,
    pub category_id: u16,
    pub is_train: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Train {
    pub overall: f64,
    pub verified: String,
    pub unix_review_time: u64,
    pub reviewer_name: Option<String>,
    pub summary: Option<String>,
    pub vote: Option<String>,
    pub style: Option<String>,
    pub image: Option<String>,
    pub userid: u64,
    pub itemid: u64,
    pub rating: f64,
}

pub struct RecordTrain {
    pub overall: u8,
    pub verified: bool,
    pub unix_review_time_truncated: u32,
    pub reviewer_name_id: Option<u32>,
    pub summary_id: Option<u32>,
    pub vote: Option<u16>,
    // pub style: Option<String>,
    // pub image: Option<String>,
    pub userid: u32,
    pub itemid: u16,
    pub rating: u8,
}

pub struct RecordImage {
    pub itemid: u16,
    pub image_id: u16,
}

use serde::{Deserialize, Serialize};

macro_rules! normalize {
    ($value:expr => $type:ty, $map:ident) => {{
        if let Some(id) = $map.get(&$value) {
            *id as $type
        } else {
            let id = $map.len() as $type;
            $map.insert($value.clone(), id);
            id
        }
    }};
}

macro_rules! normalize_opt {
    ($value:expr => $type:ty, $map:ident) => {
        $value.map(|s| {
            normalize!(s => $type, $map)
        })
    };
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NormalizedUsed {
    pub itemid: usize,
    pub brand: Option<String>,
    pub description: Option<String>,
    pub title: Option<String>,
    pub main_cat: Option<String>,
    pub price: Option<String>,
    pub is_train: bool,
}

pub struct RecordItem {
    pub itemid: u16,
    pub brand_id: Option<u16>,
    pub description_id: Option<u16>,
    pub title_id: Option<u16>,
    pub main_cat_id: Option<u8>,
    pub price_id: Option<u16>,
}

use std::collections::{HashMap, HashSet};

use regex::Regex;
#[tokio::main]
async fn main() -> Result<()> {
    let start_total = std::time::Instant::now();
    pretty_env_logger::init_timed();
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "warn");
    }
    {
        let opt = Opt::from_args();
        let data_dir = PathBuf::from(&opt.data);

        {
            let filepath = get_filepath(&data_dir, "normalized_used.csv.zip");
            let contents = unzip(&filepath).context(format!("{:?}", filepath))?;
            {
                let start = std::time::Instant::now();
                let mut brands = HashMap::<String, u16>::new();
                let mut descriptions = HashMap::<String, u16>::new();
                let mut prices = HashMap::<String, u16>::new();
                let mut titles = HashMap::<String, u16>::new();
                let mut main_cats = HashMap::<String, u8>::new();
                let mut vec_main = Vec::<RecordItem>::new();
                let buffered_reader = BufReader::new(contents.as_bytes());
                let mut rdr = csv::Reader::from_reader(buffered_reader);
                for result in rdr.deserialize() {
                    let record: NormalizedUsed = result?;
                    let brand_id = normalize_opt!(record.brand => u16, brands);
                    let description_id = normalize_opt!(record.description => u16, descriptions);
                    let price_id = normalize_opt!(record.price => u16, prices);
                    let title_id = normalize_opt!(record.title => u16, titles);
                    let main_cat_id = normalize_opt!(record.main_cat => u8, main_cats);
                    vec_main.push(RecordItem {
                        itemid: record.itemid as u16,
                        brand_id,
                        description_id,
                        price_id,
                        title_id,
                        main_cat_id,
                    });
                }
                println!(
                    "{}, vec_record: {}, brands: {}, descriptions: {}, prices: {}, titles: {}, main_cats: {}",

                    arrange_millis::get(std::time::Instant::now().duration_since(start).as_millis()),
                    vec_main.len(), brands.len(), descriptions.len(), prices.len(), titles.len(), main_cats.len()
                    );
            }
        }

        {
            let start = std::time::Instant::now();
            let filepath = get_filepath(&data_dir, "itemid_asin.csv");
            println!("Read {:?}", filepath);
            let file = std::fs::File::open(&filepath).context(format!("{:?}", filepath))?;
            let mut vec_main = Vec::<RecordItemIdAsin>::new();
            let buffered_reader = BufReader::new(file);
            let mut rdr = csv::Reader::from_reader(buffered_reader);
            for result in rdr.deserialize() {
                let record: ItemIdAsin = result.context("ItemIdAsin")?;
                let itemid = record.itemid as u16;
                let asin = record.asin;
                vec_main.push(RecordItemIdAsin { itemid, asin })
            }
            println!(
                "{}, vec_main: {}",
                arrange_millis::get(std::time::Instant::now().duration_since(start).as_millis(),),
                vec_main.len(),
            );
        }

        {
            let filepath = get_filepath(&data_dir, "also_view_used.csv.zip");
            let contents = unzip(&filepath).context(format!("{:?}", filepath))?;
            {
                let start = std::time::Instant::now();
                let mut vec_main = Vec::<RecordAlsoView>::new();
                let buffered_reader = BufReader::new(contents.as_bytes());
                let mut rdr = csv::Reader::from_reader(buffered_reader);
                for result in rdr.deserialize() {
                    let record: AlsoViewUsed = result.context("AlsoViewUsed")?;
                    let itemid = record.itemid as u16;
                    let also_view_itemid = record.also_view_itemid as u16;
                    let is_train = record.is_train;
                    vec_main.push(RecordAlsoView {
                        itemid,
                        also_view_itemid,
                        is_train,
                    })
                }
                println!(
                    "{}, vec_main: {}",
                    arrange_millis::get(
                        std::time::Instant::now().duration_since(start).as_millis(),
                    ),
                    vec_main.len(),
                );
            }
        }

        {
            let filepath = get_filepath(&data_dir, "category_used.csv.zip");
            let contents = unzip(&filepath).context(format!("{:?}", filepath))?;
            {
                let start = std::time::Instant::now();
                let mut categorys = HashMap::<String, u16>::new();
                let mut vec_main = Vec::<RecordCategory>::new();
                let buffered_reader = BufReader::new(contents.as_bytes());
                let mut rdr = csv::Reader::from_reader(buffered_reader);
                for result in rdr.deserialize() {
                    let record: CategoryUsed = result.context("")?;
                    let itemid = record.itemid as u16;
                    let is_train = record.is_train;
                    let category_id = normalize!(record.category => u16, categorys);
                    vec_main.push(RecordCategory {
                        itemid,
                        category_id,
                        is_train,
                    })
                }
                println!(
                    "{}, vec_main: {}, categorys: {}",
                    arrange_millis::get(
                        std::time::Instant::now().duration_since(start).as_millis(),
                    ),
                    vec_main.len(),
                    categorys.len(),
                );
            }
        }

        {
            let filepath = get_filepath(&data_dir, "train.csv.zip");
            let contents = unzip(&filepath).context(format!("{:?}", filepath))?;
            {
                let start = std::time::Instant::now();
                let buffered_reader = BufReader::new(contents.as_bytes());
                let mut reviewer_names = HashMap::<String, u32>::new();
                let mut summarys = HashMap::<String, u32>::new();
                let mut images = HashMap::<String, u16>::new();
                let mut vec_main = Vec::<RecordTrain>::new();
                let mut vec_image = Vec::<RecordImage>::new();
                let mut rdr = csv::Reader::from_reader(buffered_reader);
                let mut style_keys = HashSet::new();
                for (i, result) in rdr.deserialize().enumerate() {
                    let record: Train = result?;
                    let itemid = record.itemid as u16;
                    let userid = record.userid as u32;
                    let rating = record.rating as u8;
                    let overall = record.overall as u8;
                    let verified = record.verified == "True";
                    let unix_review_time_truncated = (record.unix_review_time / 100) as u32;
                    let reviewer_name_id =
                        normalize_opt!(record.reviewer_name => u32, reviewer_names);
                    let summary_id = normalize_opt!(record.summary => u32, summarys);
                    let vote = if let Some(vote) = record.vote {
                        lazy_static::lazy_static! {
                            static ref RE_COMMA: Regex = Regex::new(",").unwrap();
                        }
                        let vote = RE_COMMA.replace_all(&vote, "").to_string();
                        Some(vote.parse::<u16>().context(vote)?)
                    } else {
                        None
                    };
                    if let Some(style) = record.style {
                        parse_style(&style, &mut style_keys, i)?;
                    }
                    if let Some(image) = record.image {
                        lazy_static::lazy_static! {
                            static ref RE_QUOTE: Regex = Regex::new("'").unwrap();
                        }
                        let image = RE_QUOTE.replace_all(&image, "\"").to_string();

                        let json = Json::from_str(&image, format!("[{}].image", i))?;
                        for image in json.iter_vec().context("json.iter_vec()")? {
                            if let Ok(image) = image.as_string() {
                                let image_id = normalize!(image => u16, images);

                                vec_image.push(RecordImage { itemid, image_id });
                            }
                        }
                    }
                    vec_main.push(RecordTrain {
                        overall,
                        verified,
                        unix_review_time_truncated,
                        reviewer_name_id,
                        summary_id,
                        vote,
                        userid,
                        itemid,
                        rating,
                    });
                }
                println!(
                    "{}, vec_main: {}, images: {}, style_key: {:?}",
                    arrange_millis::get(
                        std::time::Instant::now().duration_since(start).as_millis()
                    ),
                    vec_main.len(),
                    images.len(),
                    style_keys,
                );
            }
        }
    }

    println!(
        "timing total: {}",
        arrange_millis::get(
            std::time::Instant::now()
                .duration_since(start_total)
                .as_millis(),
        ),
    );

    Ok(())
}

fn get_filepath(data_dir: &PathBuf, filename: &str) -> PathBuf {
    let mut filepath = data_dir.clone();
    filepath.push(filename);
    filepath
}

fn parse_style(style: &str, style_keys: &mut HashSet<String>, i: usize) -> Result<()> {
    lazy_static::lazy_static! {
        static ref RE_INCH: Regex = Regex::new(r#"(?P<prefix>[ /])(?P<inches>(:?(:?\d\.)?\d+)|Jar)""#).unwrap();
        static ref RE_INCH_REVERT: Regex = Regex::new(r#"<INCH>"#).unwrap();
        static ref RE_AND: Regex = Regex::new(r#" '[Nn]'? "#).unwrap();
        static ref RE_THEM: Regex = Regex::new(r#" 'Em "#).unwrap();
        static ref RE_THEM_REVERT: Regex = Regex::new(r#"<THEM>"#).unwrap();
        static ref RE_AND_REVERT: Regex = Regex::new(r#"<AND>"#).unwrap();
        static ref RE_QUOTE_TO_SKIP: Regex = Regex::new(r#"(?P<before>[^\s{])'(?P<after>[^},:])"#).unwrap();
        static ref RE_QUOTE_TO_SKIP_REVERT: Regex = Regex::new(r#"<QUOTE>"#).unwrap();
        //
        static ref RE_DOUBLEQUOTE_TO_SKIP: Regex = Regex::new(r#"(?P<before>[ ])"(?P<after>['])"#).unwrap();
        static ref RE_DOUBLEQUOTE_TO_SKIP_REVERT: Regex = Regex::new(r#"<DOUBLEQUOTE>"#).unwrap();
        static ref RE_DOUBLEQUOTED_TO_SKIP: Regex = Regex::new(r#""(?P<content>Bacon|Original)""#).unwrap();
        static ref RE_DOUBLEQUOTED_TO_SKIP_REVERT: Regex = Regex::new(r#"<DOUBLEQUOTED (?P<content>[^>]+)>"#).unwrap();
        static ref RE_QUOTE: Regex = Regex::new("'").unwrap();
    }
    let style_orig = style.to_owned();
    let style = RE_INCH.replace_all(&style, "$prefix$inches<INCHES>");
    let style = RE_AND.replace_all(&style, "<AND>").to_string();
    let style = RE_THEM.replace_all(&style, "<THEM>").to_string();
    let style = RE_QUOTE_TO_SKIP
        .replace_all(&style, "$before<QUOTE>$after")
        .to_string();
    let style = RE_DOUBLEQUOTE_TO_SKIP
        .replace_all(&style, "$before<DOUBLEQUOTE>$after")
        .to_string();
    let style = RE_DOUBLEQUOTED_TO_SKIP
        .replace_all(&style, "<DOUBLEQUOTED $content>")
        .to_string();
    let style = RE_QUOTE.replace_all(&style, "\"").to_string();
    let style = RE_AND_REVERT.replace_all(&style, " 'n' ").to_string();
    let style = RE_THEM_REVERT.replace_all(&style, " 'Em ").to_string();
    let style = RE_QUOTE_TO_SKIP_REVERT.replace_all(&style, "'");
    let style = RE_DOUBLEQUOTE_TO_SKIP_REVERT.replace_all(&style, "\\\"");
    let style = RE_DOUBLEQUOTED_TO_SKIP_REVERT.replace_all(&style, "\\\"$content\\\"");
    let style = RE_INCH_REVERT.replace_all(&style, "\"");
    let json = Json::from_str(&style, format!("[{}].style", i))
        .context(format!("orig: {}, from: {}", style_orig, style))?;

    for (key, _) in json.iter_map().context("json.iter_map()")? {
        style_keys.insert(key.to_owned());
    }
    // if style_keys.len() > 0 {
    //     bail!(
    //         "orig: {}, json: {}",
    //         style_orig,
    //         serde_json::to_string_pretty(&json.value).unwrap()
    //     );
    // }
    Ok(())
}

// use rusqlite::{Connection, NO_PARAMS};
use std::path::Path;

fn unzip(filepath: &Path) -> Result<String> {
    let start = std::time::Instant::now();

    println!("unzip {:?} . . .", filepath);
    let file = File::open(&filepath).context("File::open(&filepath)")?;
    let buffered_reader = BufReader::new(file);
    let mut zip =
        zip::ZipArchive::new(buffered_reader).context("zip::ZipArchive::new(buffered_reader)")?;
    let mut file = zip.by_index(0).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let len = contents.len();
    println!(
        "{}, {}, unzip {:?}",
        arrange_millis::get(std::time::Instant::now().duration_since(start).as_millis()),
        if len < 1024 {
            format!("{}b", len)
        } else if len < 1024 * 1024 {
            format!("{}Kb", len / 1024)
        } else if len < 1024 * 1024 * 1024 {
            format!("{}Mb", len / (1024 * 1024))
        } else {
            format!("{}Gb", len / (1024 * 1024 * 1024))
        },
        filepath,
    );
    Ok(contents)
}
