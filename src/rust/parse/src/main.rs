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

use serde::{Deserialize, Serialize};
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

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Simple {
//     pub also_view: Option<String>,
//     pub asin: String,
//     pub itemid: Option<usize>,
//     pub brand: Option<String>,
//     pub category: String,
//     pub description: Option<String>,
//     pub title: Option<String>,
//     pub main_cat: Option<String>,
//     pub price: Option<String>,
// }
//
// #[derive(Serialize, Deserialize, Debug)]
// pub struct Normalized {
//     pub asin: String,
//     pub itemid: Option<usize>,
//     pub brand: Option<String>,
//     pub description: Option<String>,
//     pub title: Option<String>,
//     pub main_cat: Option<String>,
//     pub price: Option<String>,
// }

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

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Category {
//     pub asin: String,
//     pub itemid: Option<usize>,
//     pub category: String,
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryUsed {
    pub itemid: usize,
    pub category: String,
    pub is_train: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AlsoViewUsed {
    pub itemid: usize,
    pub also_view_itemid: usize,
    pub is_train: bool,
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct AlsoView {
//     pub asin: String,
//     pub itemid: Option<usize>,
//     pub also_view: String,
//     pub also_view_itemid: Option<usize>,
// }

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    pub itemid: usize,
    pub asin: String,
}

use std::collections::{HashMap, HashSet};

use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct VecMut {
    vec: Arc<RwLock<Vec<u8>>>,
}

impl VecMut {
    pub fn new() -> Self {
        Self {
            vec: Arc::new(RwLock::new(Vec::new())),
        }
    }
    pub fn len(&self) -> usize {
        self.vec.read().unwrap().len()
    }
}

impl std::io::Write for VecMut {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut vec = self.vec.write().expect("RwLock::write()");
        for i in buf {
            (*vec).push(*i)
        }
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
const COUNT_SIMPLE: usize = 287208;
const COUNT_CATEGORY: usize = 1120436;
const COUNT_ALSO_VIEW: usize = 1899302;
const COUNT_NORMALIZED: usize = COUNT_SIMPLE;

macro_rules! save_csv {
    ($data_dir: expr, $var: ident) => {
        let filename = &stringify!($var)[4..];
        save_csv!($data_dir, $var, filename);
    };
    ($data_dir: expr, $var: ident, $filename: expr) => {
        let buf = VecMut::new();
        let mut wtr = csv::Writer::from_writer(buf.clone());
        for record in $var {
            wtr.serialize(record)?;
        }
        wtr.flush()?;

        let filename = format!("{}.csv.zip", $filename);
        let filepath = get_filepath(&$data_dir, &filename);
        let file = std::fs::File::create(&filepath).unwrap();
        let mut zip = zip::ZipWriter::new(file);
        let options = zip::write::FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .unix_permissions(0o755);
        zip.start_file(filename, options)?;
        zip.write_all(&buf.vec.read().unwrap())?;
        zip.finish()?;
        println!("written: {:?}", filepath);
    };
}

use std::path::Path;
fn read_asin2itemid_from(filepath: &Path) -> Result<HashMap<String, usize>> {
    let file = File::open(&filepath).expect("could not open file");
    let buffered_reader = BufReader::new(file);
    let mut rdr = csv::Reader::from_reader(buffered_reader);
    let mut asin2itemid: HashMap<String, usize> = HashMap::new();
    let mut set_itemid: HashSet<usize> = HashSet::new();
    for result in rdr.deserialize() {
        let record: Record = result?;
        asin2itemid.insert(record.asin, record.itemid);
        set_itemid.insert(record.itemid);
    }
    println!(
        "filepath: {:?}, asin2itemid: {}, set_itemid: {}",
        filepath,
        asin2itemid.len(),
        set_itemid.len()
    );
    Ok(asin2itemid)
}

fn get_filepath(data_dir: &PathBuf, filename: &str) -> PathBuf {
    let mut filepath = data_dir.clone();
    filepath.push(filename);
    filepath
}

// use rusqlite::{Connection, NO_PARAMS};

use itertools::Itertools;
#[tokio::main]
async fn main() -> Result<()> {
    let start_total = std::time::Instant::now();
    pretty_env_logger::init_timed();

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "warn");
    }

    let opt = Opt::from_args();
    let data_dir = PathBuf::from(&opt.data);

    let filepath = get_filepath(&data_dir, "itemid_asin.csv");
    let asin2itemid = read_asin2itemid_from(&filepath)?;

    let filepath = get_filepath(&data_dir, "itemid_asin_train.csv");
    let asin2itemid_train = read_asin2itemid_from(&filepath)?;

    let start = std::time::Instant::now();

    let filename = "meta_Grocery_and_Gourmet_Food.json.zip";
    println!("Reading '{}' . . .", filename);
    let filepath = get_filepath(&data_dir, filename);
    let file = File::open(&filepath).context(format!("{:?}", filepath))?;
    let buffered_reader = BufReader::new(file);
    let mut zip = zip::ZipArchive::new(buffered_reader)?;
    let mut file = zip.by_index(0).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!(
        "unzip: {}",
        arrange_millis::get(std::time::Instant::now().duration_since(start).as_millis())
    );
    let buffered_reader = BufReader::new(contents.as_bytes());

    let deserializer = serde_json::Deserializer::from_reader(buffered_reader);

    let iterator = deserializer.into_iter::<Item>();
    let start = std::time::Instant::now();
    // let mut vec_simple: Vec<Simple> = Vec::with_capacity(COUNT_SIMPLE);
    // let mut vec_normalized: Vec<Normalized> = Vec::with_capacity(COUNT_NORMALIZED);
    // let mut vec_category: Vec<Category> = Vec::with_capacity(COUNT_CATEGORY);
    // let mut vec_also_view: Vec<AlsoView> = Vec::with_capacity(COUNT_ALSO_VIEW);

    let mut vec_normalized_used: Vec<NormalizedUsed> = Vec::with_capacity(COUNT_NORMALIZED);
    let mut vec_category_used: Vec<CategoryUsed> = Vec::with_capacity(COUNT_CATEGORY);
    let mut vec_also_view_used: Vec<AlsoViewUsed> = Vec::with_capacity(COUNT_ALSO_VIEW);

    use indicatif::{ProgressBar, ProgressStyle};
    let pbar = ProgressBar::new(COUNT_SIMPLE as u64);
    pbar.set_style(
        ProgressStyle::default_bar()
            .template("{eta_precise} [{bar:60.cyan/blue}] {wide_msg}")
            .progress_chars("=>-"),
    );
    pbar.enable_steady_tick(125);

    let mut normalized_is_train_count = 0;
    let mut category_is_train_count = 0;
    let mut also_view_is_train_count = 0;
    for (i, item) in iterator.enumerate() {
        pbar.set_message(&format!("{}", i));
        let item = item?;
        let description = match item.description {
            None => None,
            Some(vec_string) => Some(vec_string.join("\n\n")),
        };
        let itemid = asin2itemid.get(&item.asin);
        let is_train = asin2itemid_train.contains_key(&item.asin);
        // let normalized = Normalized {
        //     asin: item.asin.clone(),
        //     itemid: itemid.copied(),
        //     brand: item.brand.clone(),
        //     description: description.clone(),
        //     title: item.title.clone(),
        //     main_cat: item.main_cat.clone(),
        //     price: item.price.clone(),
        // };
        // for category in &item.category {
        //     vec_category.push(Category {
        //         asin: item.asin.clone(),
        //         itemid: itemid.copied(),
        //         category: category.clone(),
        //     });
        // }
        // if let Some(vec_string) = &item.also_view {
        //     for s in vec_string {
        //         let also_view_itemid = asin2itemid.get(s.as_str());
        //         vec_also_view.push(AlsoView {
        //             asin: item.asin.clone(),
        //             itemid: itemid.copied(),
        //             also_view: s.clone(),
        //             also_view_itemid: also_view_itemid.copied(),
        //         });
        //     }
        // }
        if let Some(itemid) = itemid {
            vec_normalized_used.push(NormalizedUsed {
                itemid: *itemid,
                is_train,
                brand: item.brand.clone(),
                description: description.clone(),
                title: item.title.clone(),
                main_cat: item.main_cat.clone(),
                price: item.price.clone(),
            });
            if is_train {
                normalized_is_train_count += 1;
            }
            for category in &item.category {
                vec_category_used.push(CategoryUsed {
                    itemid: *itemid,
                    is_train,
                    category: category.clone(),
                });
                if is_train {
                    category_is_train_count += 1;
                }
            }
            if let Some(vec_string) = &item.also_view {
                for s in vec_string {
                    if let Some(also_view_itemid) = asin2itemid.get(s.as_str()) {
                        vec_also_view_used.push(AlsoViewUsed {
                            itemid: *itemid,
                            is_train,
                            also_view_itemid: *also_view_itemid,
                        });
                        if is_train {
                            also_view_is_train_count += 1;
                        }
                    }
                }
            }
        }
        // let simple = Simple {
        //     also_view: match item.also_view {
        //         None => None,
        //         Some(vec_string) => Some(vec_string.join("|")),
        //     },
        //     asin: item.asin.clone(),
        //     itemid: itemid.copied(),
        //     brand: item.brand.clone(),
        //     category: item.category.join("|"),
        //     description: description.clone(),
        //     title: item.title.clone(),
        //     main_cat: item.main_cat.clone(),
        //     price: item.price.clone(),
        // };
        // vec_simple.push(simple);
        // vec_normalized.push(normalized);
        pbar.set_position(i as u64);
    }
    pbar.finish();

    // println!(
    //     "{}, simple: {}, normalized/used: {}/{}, category/used: {}/{}, also_view/used: {}/{}",
    //     arrange_millis::get(std::time::Instant::now().duration_since(start).as_millis()),
    //     vec_simple.len(),
    //     vec_normalized.len(),
    //     vec_normalized_used.len(),
    //     vec_category.len(),
    //     vec_category_used.len(),
    //     vec_also_view.len(),
    //     vec_also_view_used.len(),
    // );
    println!(
        "{}, normalized_used/is_train: {}/{}, category_used/is_train: {}/{}, also_view_used/is_train: {}/{}",
        arrange_millis::get(std::time::Instant::now().duration_since(start).as_millis()),
        vec_normalized_used.len(),
        normalized_is_train_count,
        vec_category_used.len(),
        category_is_train_count,
        vec_also_view_used.len(),
        also_view_is_train_count,
        );

    let start = std::time::Instant::now();

    // save_csv!(data_dir, vec_simple, "json");
    // save_csv!(data_dir, vec_normalized);
    // save_csv!(data_dir, vec_category);
    // save_csv!(data_dir, vec_also_view);
    save_csv!(data_dir, vec_normalized_used);
    save_csv!(data_dir, vec_category_used);
    save_csv!(data_dir, vec_also_view_used);

    let filepath = get_filepath(&data_dir, "flask.db");
    let conn = rusqlite::Connection::open(&filepath).context(format!("{:?}", filepath))?;
    let sql = "
        create table data {
            itemid,
        }
    ";
    conn.execute(sql, NO_PARAMS).context(sql)?;

    println!(
        "write *.csv.zip took {}",
        arrange_millis::get(std::time::Instant::now().duration_since(start).as_millis())
    );
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
