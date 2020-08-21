
# Обработка `meta_Grocery_and_Gourmet_Food.json`


<!-- vim-markdown-toc Redcarpet -->

* [Установка Rust](#установка-rust)
* [Подготовка данных для запуска утилиты](#подготовка-данных-для-запуска-утилиты)
    * [Подготовка `itemid_asin.csv`](#подготовка-itemid_asin-csv)
* [Запуск утилиты](#запуск-утилиты)
* [Результат работы](#результат-работы)

<!-- vim-markdown-toc -->

Для извлечения данных из `../data/meta_Grocery_and_Gourmet_Food.json.zip` была написана утилита на языке Rust

## Установка Rust

https://www.rust-lang.org/tools/install

## Подготовка данных для запуска утилиты

Утилита использует следующие файлы из папки `../data`:

- meta_Grocery_and_Gourmet_Food.json.zip

- itemid_asin.csv

### Подготовка `itemid_asin.csv`

```
train = pd.read_csv('data/train.csv.zip', low_memory=False)
test = pd.read_csv('data/test.csv.zip', low_memory=False)
itemid_asin = train[['itemid', 'asin']]
itemid_asin_test = test[['itemid', 'asin']]
itemid_asin = itemid_asin.drop_duplicates().reset_index(drop = True)
itemid_asin_test = itemid_asin_test.drop_duplicates().reset_index(drop = True)
itemid_asin_concat = pd.concat([itemid_asin, itemid_asin_test]).drop_duplicates().reset_index(drop = True)
itemid_asin_concat.to_csv('data/itemid_asin.csv')
```


## Запуск утилиты

В папке `src/rust`:

```
cargo run --release 
```

Вывод:

```
filepath: "../../data/itemid_asin.csv", asin2itemid: 41320, set_itemid: 41320
filepath: "../../data/itemid_asin_train.csv", asin2itemid: 41302, set_itemid: 41302
Reading 'meta_Grocery_and_Gourmet_Food.json.zip' . . .
unzip: 4.568s
00:00:00 [============================================================] 287208
24.142s, normalized_used/is_train: 42003/41985, category_used/is_train: 166917/166843, also_view_used/is_train: 178606/178530
write *.csv.zip: 3.222s
timing total: 31.951s
```

## Результат работы

Файлы в папке `../data`:

- normalized_used.csv.zip
- category_used.csv.zip
- also_view_used.csv.zip

*устаревшие* (неиспользуемые):

- json.csv.zip

- normalized.csv.zip
- category.csv.zip
- also_view.csv.zip

[Подробное описание](../data/README.md)

