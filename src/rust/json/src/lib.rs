#[allow(unused_imports)]
use log::{error, warn, info, debug, trace};
#[allow(unused_imports)]
use anyhow::{anyhow, bail, Result, Error, Context};

use std::fmt;
use serde_json::{Value, Map};
use tokio::fs::File;
use tokio::prelude::*;
use std::convert::TryFrom;

// ============================================================================
// ============================================================================

#[derive(Debug, Clone)]
pub enum By {
    Index(usize),
    Key(String),
}

impl By {
    pub fn key<S: AsRef<str>>(key: S) -> Self {
        Self::Key(key.as_ref().to_owned())
    }
    pub fn index(index: usize) -> Self {
        Self::Index(index)
    }
}

impl fmt::Display for By {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            By::Key(key) => {
                write!(f, ".{:?}", key)?;
            },
            By::Index(index) => {
                write!(f, "[{}]", index)?;
            },
        }
        Ok(())
    }
}

// ============================================================================

#[derive(Debug, Clone)]
pub enum JsonSource {
    FilePath(std::path::PathBuf),
    Name(String),
}

impl fmt::Display for JsonSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonSource::FilePath(file_path) => {
                write!(f, "{:?}", file_path)?;
            },
            JsonSource::Name(name) => {
                write!(f, "{:?}", name)?;
            },
        }
        Ok(())
    }
}

// ============================================================================

#[derive(Debug, Clone)]
pub struct JsonPath {
    pub src: JsonSource,
    pub items: Vec<By>,
}

impl JsonPath {
    pub fn new<>(src: JsonSource) -> Self {
        Self {
            src,
            items: Vec::new(),
        }
    }
    pub fn add(&self, path_item: By) -> Self {
        let mut items = self.items.clone();
        items.push(path_item);
        let src = self.src.to_owned();
        Self {src, items}
    }
}

impl fmt::Display for JsonPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.src)?;
        for item in self.items.iter() {
            write!(f, "{}", item)?;
        }
        Ok(())
    }
}

// ============================================================================

#[derive(Clone, Debug)]
pub struct Json {
    pub value: Value,
    pub path: JsonPath,
}


macro_rules! as_ {
    ($type: ty) => {
        paste::item!{
            pub fn [< as_ $type >] (&self) -> Result<$type> {
                match &self.value {
                    Value::Number(n) => {
                        if n.[< is_ $type >] () {
                            let n = n.[< as_ $type >]().unwrap();
                            Ok(n)
                        } else {
                            bail!("{} expected to be a {}, but: {}", self.path, stringify!($type), n);
                        }
                    },
                    _ => {
                        bail!("{} expected to be a {}, but: {}", self.path, stringify!($type), serde_json::to_string_pretty(&self.value)?);
                    },
                }
            }
        }
    };
    ($from: ty => $type: ty) => {
        paste::item!{
            pub fn [< as_ $type >] (&self) -> Result<$type> {
                let n = self.[< as_ $from >]()?;
                let n = $type::try_from(n)
                    .map_err(|err| Error::new(err))
                    .context(format!("{} expected to be a {}, but: {}", self.path, stringify!($type), n))?;
                Ok(n)
            }
        }
    };
}

macro_rules! parse_as_ {
    ($type: ty) => {
        paste::item!{
            pub fn [< parse_as_ $type >] (&self) -> Result<$type> {
                match &self.value {
                    Value::Number(n) => {
                        if n.[< is_ $type >] () {
                            let n = n.[< as_ $type >]().unwrap();
                            Ok(n)
                        } else {
                            bail!("{} expected to be a {}, but: {}", self.path, stringify!($type), n);
                        }
                    },
                    Value::String(s) => {
                        s.parse::<$type>().context(format!("{} expected to be a {} or String parseable to {}, but: {}", self.path, stringify!($type), stringify!($type), serde_json::to_string_pretty(&self.value)?))
                    },
                    _ => {
                        bail!("{} expected to be a {} or String parseable to {}, but: {}", self.path, stringify!($type), stringify!($type), serde_json::to_string_pretty(&self.value)?);
                    },
                }
            }
        }
    };
    ($from: ty => $type: ty) => {
        paste::item!{
            pub fn [< parse_as_ $type >] (&self) -> Result<$type> {
                match &self.value {
                    Value::Number(n) => {
                        if n.[< is_ $from >] () {
                            let n = n.[< as_ $from >]().unwrap();
                            let n = $type::try_from(n)
                                .map_err(|err| Error::new(err))
                                .context(format!("{} expected to be a {}, but: {}", self.path, stringify!($type), n))?;
                            Ok(n)
                        } else {
                            bail!("{} expected to be a {}, but: {}", self.path, stringify!($type), n);
                        }
                    },
                    Value::String(s) => {
                        s.parse::<$type>().context(format!("{} expected to be a {} or String parseable to {}, but: {}", self.path, stringify!($type), stringify!($type), serde_json::to_string_pretty(&self.value)?))
                    },
                    _ => {
                        bail!("{} expected to be a {} or String parseable to {}, but: {}", self.path, stringify!($type), stringify!($type), serde_json::to_string_pretty(&self.value)?);
                    },
                }
            }
        }
    };
}

macro_rules! parse_as__after {
    ($type: ty) => {
        paste::item!{
            pub fn [< parse_as_ $type _after> ]<F: Fn(&str) -> std::borrow::Cow<str>> (&self, closure: F) -> Result<$type> {
                match &self.value {
                    Value::Number(n) => {
                        if n.[< is_ $type >] () {
                            let n = n.[< as_ $type >]().unwrap();
                            Ok(n)
                        } else {
                            bail!("{} expected to be a {}, but: {}", self.path, stringify!($type), n);
                        }
                    },
                    Value::String(s) => {
                        closure(s).parse::<$type>().context(format!("{} expected to be a {} or String parseable to {}, but: {}", self.path, stringify!($type), stringify!($type), serde_json::to_string_pretty(&self.value)?))
                    },
                    _ => {
                        bail!("{} expected to be a {} or String parseable to {}, but: {}", self.path, stringify!($type), stringify!($type), serde_json::to_string_pretty(&self.value)?);
                    },
                }
            }
        }
    };
    // ($from: ty => $type: ty, $closure: tt) => {
    //     paste::item!{
    //         pub fn [< parse_as_ $type >] (&self) -> Result<$type> {
    //             match &self.value {
    //                 Value::Number(n) => {
    //                     if n.[< is_ $from >] () {
    //                         let n = n.[< as_ $from >]().unwrap();
    //                         let n = $type::try_from(n)
    //                             .map_err(|err| Error::new(err))
    //                             .context(format!("{} expected to be a {}, but: {}", self.path, stringify!($type), n))?;
    //                         Ok(n)
    //                     } else {
    //                         bail!("{} expected to be a {}, but: {}", self.path, stringify!($type), n);
    //                     }
    //                 },
    //                 Value::String(s) => {
    //                     $closure(s).parse::<$type>().context(format!("{} expected to be a {} or String parseable to {}, but: {}", self.path, stringify!($type), stringify!($type), serde_json::to_string_pretty(&self.value)?))
    //                 },
    //                 _ => {
    //                     bail!("{} expected to be a {} or String parseable to {}, but: {}", self.path, stringify!($type), stringify!($type), serde_json::to_string_pretty(&self.value)?);
    //                 },
    //             }
    //         }
    //     }
    // };
}

impl<'a> Json {
    pub fn new(value: Value, src: JsonSource) -> Self {
        Self {
            value,
            path: JsonPath::new(src),
        }
    }
    pub async fn from_file<P: AsRef<std::path::Path>>(file_path: P) -> Result<Self> {
        let file_path = file_path.as_ref();
        let mut file = File::open(file_path).await
            .map_err(|err| Error::new(err))
            .context(format!(r#"Json::from_file({:?}): File::open"#, file_path))?
        ;
        let mut contents = vec![];
        file.read_to_end(&mut contents).await
            .map_err(|err| Error::new(err))
            .context(format!(r#"Json::from_file({:?}): file.read_to_end"#, file_path))?
        ;
        let s = std::str::from_utf8(&contents)?;

        let value: Value = serde_json::from_str(&s)
            .map_err(|err| Error::new(err))
            .context(format!(r#"Json::from_file({:?}): serde_json::from_str {:?}"#, file_path, s))?
        ;
        Ok(Self {
            value,
            path: JsonPath::new(JsonSource::FilePath(file_path.to_owned())),
        })
    }
    pub fn from_str<S: AsRef<str>, S2: AsRef<str>>(s: S, name: S2) -> Result<Self> {
        let value: Value = serde_json::from_str(s.as_ref())?;
        Ok(Self {
            value,
            path: JsonPath::new(JsonSource::Name(name.as_ref().to_owned())),
        })
    }
    pub fn get<'b, P: AsRef<[By]>>(&'b self, path_items: P) -> Result<Self> {
        let mut ret = self.clone();
        for path_item in path_items.as_ref().iter() {
            ret = ret.get_by_path_item(&path_item)?;
        }
        Ok(ret)
    }
    fn get_by_path_item<'b>(&'b self, path_item: &By) -> Result<Self> {
        let get_ret = 
            match path_item {
                By::Key(key) => {
                    self.value.get(key)
                },
                By::Index(index) => {
                    self.value.get(index)
                },
            }
        ;
        let value: serde_json::Value = match get_ret {
            Some(value) => value.clone(),
            None => {
                bail!("{}: not found {} at {}", self.path, path_item, serde_json::to_string_pretty(&self.value)?);
            },
        };
        let path = self.path.add(path_item.to_owned());
        Ok(Self { value, path })
    }
    pub fn as_str<'b>(&'b self) -> Result<&'b str> {
        match &self.value {
            Value::String(s) => Ok(s),
            _ => {
                bail!("{} expected to be a String, but: {}", self.path, serde_json::to_string_pretty(&self.value)?);
            },
        }
    }
    pub fn as_string(&self) -> Result<String> {
        Ok(self.as_str()?.to_owned())
    }
    pub fn as_vec(&self) -> Result<&Vec<Value>> {
        match &self.value {
            Value::Array(vec) => Ok(vec),
            _ => {
                bail!("{} expected to be an Array, but: {}", self.path, serde_json::to_string_pretty(&self.value)?);
            },
        }
    }
    pub fn as_map(&self) -> Result<&Map<String, Value>> {
        match &self.value {
            Value::Object(map) => Ok(map),
            _ => {
                bail!("{} expected to be an Object, but: {}", self.path, serde_json::to_string_pretty(&self.value)?);
            },
        }
    }
    pub fn as_null(&self) -> Result<()> {
        match &self.value {
            Value::Null => Ok(()),
            _ => {
                bail!("{} expected to be a Null, but: {}", self.path, serde_json::to_string_pretty(&self.value)?);
            },
        }
    }
    pub fn as_bool(&self) -> Result<bool> {
        match &self.value {
            Value::Bool(b) => Ok(*b),
            _ => {
                bail!("{} expected to be a Bool, but: {}", self.path, serde_json::to_string_pretty(&self.value)?);
            },
        }
    }
    pub fn iter_map<'b>(&'b self) -> Result<MapIterator<'b>> {
        let map = self.as_map()?;
        let iter = MapIterator {
            iter: map.iter(),
            path: &self.path,
        };
        Ok(iter)
    }
    pub fn iter_vec<'b>(&'b self) -> Result<VecIterator<'b>> {
        let vec = self.as_vec()?;
        let iter = VecIterator {
            iter: vec.iter(),
            path: &self.path,
            index: 0,
        };
        Ok(iter)
    }

    as_!(u64);
    as_!(u64 => usize);
    as_!(u64 => u32);
    as_!(u64 => u16);
    as_!(u64 => u8);

    as_!(i64);
    as_!(i64 => isize);
    as_!(i64 => i32);
    as_!(i64 => i16);
    as_!(i64 => i8);

    as_!(f64);

    parse_as_!(u64);
    parse_as_!(u64 => usize);
    parse_as_!(u64 => u32);
    parse_as_!(u64 => u16);
    parse_as_!(u64 => u8);

    parse_as_!(i64);
    parse_as_!(i64 => isize);
    parse_as_!(i64 => i32);
    parse_as_!(i64 => i16);
    parse_as_!(i64 => i8);

    parse_as_!(f64);

    parse_as__after!(u64);
}

impl fmt::Display for Json {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: ", self.path)?;
        write!(f, "{}", serde_json::to_string_pretty(&self.value).unwrap())?;
        Ok(())
    }
}

// ============================================================================

// #[derive(Debug)]
pub struct MapIterator<'a> {
    path: &'a JsonPath,
    iter: serde_json::map::Iter<'a>,
}

impl<'a> Iterator for MapIterator<'a> {
    type Item = (&'a str, Json);
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.iter.next();
        match item {
            None => None,
            Some((key, value)) => {
                let key = key.as_ref();
                let value = Json {
                    value: value.clone(),
                    path: self.path.add(By::key(key)),
                };
                Some((key, value))
            }
        }
    }
}

// ============================================================================

#[derive(Debug)]
pub struct VecIterator<'a> {
    path: &'a JsonPath,
    iter: std::slice::Iter<'a, Value>,
    index: usize,
}

impl<'a> Iterator for VecIterator<'a> {
    type Item = Json;
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.iter.next();
        match item {
            None => None,
            Some(value) => {
                let value = Json {
                    value: value.clone(),
                    path: self.path.add(By::index(self.index)),
                };
                self.index += 1;
                Some(value)
            }
        }
    }
}

// ============================================================================
// ============================================================================
// ============================================================================

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use log::{error, warn, info, debug, trace};
    use super::*;

    use pretty_assertions::{assert_eq};

    #[tokio::test]
    async fn test_json_0() -> Result<()> {
        test_helper::init();

        let file_path = std::path::Path::new("test_data/sample.json");
        let json: Json = Json::from_file(&file_path).await?;

        let key = "defaultModificationId";
        let by: Vec<By> = vec![By::key(key)];
        let value = json.get(by)?;
        let tst = value.as_u64()?;
        assert_eq!(tst, 363896u64);

        let tst = value.as_usize()?;
        assert_eq!(tst, 363896usize);

        let tst = value.as_u32()?;
        assert_eq!(tst, 363896u32);

        let err = value.as_u16();
        let tst = err.unwrap_err().downcast::<String>().unwrap();
        let eta = format!("{:?}.{:?} expected to be a u16, but: {}", file_path, key, 363896);
        assert_eq!(tst, eta);

        let err = value.as_u8();
        let tst = err.unwrap_err().downcast::<String>().unwrap();
        let eta = format!("{:?}.{:?} expected to be a u8, but: {}", file_path, key, 363896);
        assert_eq!(tst, eta);

        let err = value.as_str();
        let tst = err.unwrap_err().downcast::<String>().unwrap();
        let eta = format!("{:?}.{:?} expected to be a String, but: {}", file_path, key, serde_json::to_string_pretty(&value.value)?);
        assert_eq!(tst, eta);
        Ok(())
    }

    #[tokio::test]
    async fn test_json_1() -> Result<()> {
        test_helper::init();

        let file_path = std::path::Path::new("test_data/sample.json");
        let json: Json = Json::from_file(&file_path).await?;

        let value = json.get([By::key("defaultModificationId")])?;
        let tst = format!("{}", value);
        let eta = r#""test_data/sample.json"."defaultModificationId": 363896"#;
        assert_eq!(tst, eta);
        
        Ok(())
    }
        
    #[tokio::test]
    async fn test_json_2() -> Result<()> {
        test_helper::init();

        let file_path = std::path::Path::new("test_data/sample.json");
        let json: Json = Json::from_file(&file_path).await?;

        let key = "defaultModificationId2";
        let err = json.get([
            By::key(key)
        ]);
        let tst = err.unwrap_err().downcast::<String>().unwrap();
        let eta = format!("{:?}: not found .{:?} at {}", file_path, key, serde_json::to_string_pretty(&json.value)?);
        assert_eq!(tst, eta); // assert_eq!(&err[0..80], &expected[0..80]);

        Ok(())
    }

    #[tokio::test]
    async fn test_json_3() -> Result<()> {
        test_helper::init();

        let file_path = std::path::Path::new("test_data/sample.json");
        let json: Json = Json::from_file(&file_path).await?;

        let value = json.get([By::key("breadcrumbs"), By::index(1), By::key("title")])?;
        let tst = value.as_str()?;
        let eta = "Транспорт";
        assert_eq!(tst, eta);

        Ok(())
    }

    #[tokio::test]
    async fn test_json_4() -> Result<()> {
        test_helper::init();

        let file_path = std::path::Path::new("test_data/sample.json");
        let json: Json = Json::from_file(&file_path).await?;

        let value = json.get([By::key("breadcrumbs")])?;
        let _tst = value.as_vec()?;

        Ok(())
    }
        
    #[tokio::test]
    async fn test_json_5() -> Result<()> {
        test_helper::init();

        let file_path = std::path::Path::new("test_data/sample.json");
        let json: Json = Json::from_file(&file_path).await?;

        let value = json.get([By::key("breadcrumbs"), By::index(1)])?;
        let _tst = value.as_map()?;

        Ok(())
    }

    #[tokio::test]
    async fn test_json_6() -> Result<()> {
        test_helper::init();

        let file_path = std::path::Path::new("test_data/sample.json");
        let json: Json = Json::from_file(&file_path).await?;

        let value = json.get([By::key("modifications"), By::key("items"), By::index(0), By::key("specification"), By::key("blocks"), By::index(0), By::key("params"), By::index(1), By::key("value")])?;
        let tst = value.as_str()?;
        assert_eq!(tst, "150");

        let tst = value.parse_as_u64()?;
        assert_eq!(tst, 150);

        let tst = value.parse_as_u8()?;
        assert_eq!(tst, 150);

        Ok(())
    }
        
    #[tokio::test]
    async fn test_json_7() -> Result<()> {
        test_helper::init();

        let file_path = std::path::Path::new("test_data/sample.json");
        let json: Json = Json::from_file(&file_path).await?;

        let value = json.get([By::key("modifications"), By::key("items"), By::index(0), By::key("specification"), By::key("blocks"), By::index(2), By::key("params"), By::index(2), By::key("value")])?;

        let tst = value.parse_as_u64()?;
        assert_eq!(tst, 330);

        let tst = value.parse_as_f64()?;
        assert_eq!(tst as u64, 330);

        let err = value.parse_as_u8();
        let tst = err.unwrap_err().downcast::<String>().unwrap();
        let eta = r#""test_data/sample.json"."modifications"."items"[0]."specification"."blocks"[2]."params"[2]."value" expected to be a u8 or String parseable to u8, but: "330""#.to_owned();
        assert_eq!(tst, eta); 

        Ok(())
    }

    #[tokio::test]
    async fn test_json_8() -> Result<()> {
        test_helper::init();

        let file_path = std::path::Path::new("test_data/sample.json");
        let json: Json = Json::from_file(&file_path).await?;

        let value = json.get([By::key("modifications"), By::key("items"), By::index(0), By::key("specification"), By::key("blocks"), By::index(0), By::key("params"), By::index(2), By::key("value")])?;
        let tst = value.parse_as_f64()?;
        assert_eq!(tst as u8, 2);

        let err = value.parse_as_u8();
        let tst = err.unwrap_err().downcast::<String>().unwrap();
        let eta = r#""test_data/sample.json"."modifications"."items"[0]."specification"."blocks"[0]."params"[2]."value" expected to be a u8 or String parseable to u8, but: "2.0""#.to_owned();
        assert_eq!(tst, eta); 

        Ok(())
    }

    #[tokio::test]
    async fn test_json_9() -> Result<()> {
        test_helper::init();

        let file_path = std::path::Path::new("test_data/sample.json");
        let json: Json = Json::from_file(&file_path).await?;

        let value = json.get([By::key("breadcrumbs"), By::index(1), By::key("title")])?;
        let err = value.parse_as_u64();
        let tst = err.unwrap_err().downcast::<String>().unwrap();
        let eta = r#""test_data/sample.json"."breadcrumbs"[1]."title" expected to be a u64 or String parseable to u64, but: "Транспорт""#.to_owned();
        assert_eq!(tst, eta); 

        Ok(())
    }

    #[tokio::test]
    async fn test_json_a() -> Result<()> {
        test_helper::init();

        let file_path = std::path::Path::new("test_data/sample.json");
        let json: Json = Json::from_file(&file_path).await?;

        let value = json.get([By::key("itemsBlockData"), By::key("searchButton")])?;
        let mut keys: Vec<&str> = Vec::new();
        for (key, val) in value.iter_map()? {
            keys.push(key);
            match key {
                "errorInfoText" => {
                    let bold = val.get([By::key("bold")])?;
                    assert_eq!(bold.as_str()?, "На Авито нет BMW 5 серия");

                    let err = bold.parse_as_u8();
                    let tst = err.unwrap_err().downcast::<String>().unwrap();
                    let eta = r#""test_data/sample.json"."itemsBlockData"."searchButton"."errorInfoText"."bold" expected to be a u8 or String parseable to u8, but: "На Авито нет BMW 5 серия""#.to_owned();
                    assert_eq!(tst, eta); 
                },
                _ => {},
            }
        }
        assert_eq!(keys, ["errorButtonText", "errorInfoText", "errorSearchLink", "infoText"]);

        let err = value.iter_vec();
        let tst = err.unwrap_err().downcast::<String>().unwrap();
        let eta = "\"test_data/sample.json\".\"itemsBlockData\".\"searchButton\" expected to be an Array, but: {\n  \"errorButtonText\": \"Выбрать другой автомобиль на Авито\",\n  \"errorInfoText\": {\n    \"bold\": \"На Авито нет BMW 5 серия\",\n    \"normal\": \", но есть много других машин\"\n  },\n  \"errorSearchLink\": \"/rossiya/avtomobili\",\n  \"infoText\": {\n    \"bold\": {\n      \"few\": \"объявления BMW 5 серия\",\n      \"many\": \"объявлений BMW 5 серия\",\n      \"one\": \"объявление BMW 5 серия\",\n      \"other\": \"объявлений BMW 5 серия\"\n    },\n    \"normal\": \"найдено в России\"\n  }\n}".to_owned();
        assert_eq!(tst, eta);

        Ok(())
    }

    #[tokio::test]
    async fn test_json_b() -> Result<()> {
        test_helper::init();

        let file_path = std::path::Path::new("test_data/sample.json");
        let json: Json = Json::from_file(&file_path).await?;

        let value = json.get([By::key("breadcrumbs")])?;
        let mut titles: Vec<String> = Vec::new();
        for val in value.iter_vec()? {
            let val = val.get([By::key("title")])?;
            let s = val.as_str()?;
            titles.push(s.to_owned());
        }
        assert_eq!(titles, ["Все объявления в России", "Транспорт", "Автомобили"]);

        Ok(())
    }

    #[tokio::test]
    async fn test_json_c() -> Result<()> {
        test_helper::init();

        let json = r#"
            { "some": "thing" }
        "#;
        let json: Json = Json::from_str(json, "json")?;
        let value = json.get([By::key("some")])?;
        assert_eq!(value.as_str()?, "thing");

        let err = value.parse_as_u8();
        let tst = err.unwrap_err().downcast::<String>().unwrap();
        let eta = r#""json"."some" expected to be a u8 or String parseable to u8, but: "thing""#.to_owned();
        assert_eq!(tst, eta); 

        Ok(())
    }

    use lazy_static::lazy_static;
    use regex::Regex;
    #[tokio::test]
    async fn test_json_d() -> Result<()> {
        test_helper::init();

        let file_path = std::path::Path::new("test_data/sample.json");
        let json: Json = Json::from_file(&file_path).await?;

        let value = json.get([By::key("modifications"), By::key("items"), By::index(0), By::key("specification"), By::key("blocks"), By::index(0), By::key("params"), By::index(4), By::key("name")])?;

        lazy_static! {
            static ref RE: Regex = Regex::new(r"[^\d]").unwrap();
        }
        let tst = value.parse_as_u64_after(|s| RE.replace_all(s, ""))?;
        assert_eq!(tst, 100);

        Ok(())
    }
}

