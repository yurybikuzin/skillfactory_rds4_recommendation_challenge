
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
```


## Запуск утилиты

В папке `src`:

```
cargo run --release 
```

## Результат работы

Файлы в папке `../data`:

- json.csv.zip

- normalized.csv.zip
- category.csv.zip
- also_view.csv.zip

- normalized_used.csv.zip
- category_used.csv.zip
- also_view_used.csv.zip

[Подробное описание](../data/README.md)

