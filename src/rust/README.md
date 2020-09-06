
# Обработка `meta_Grocery_and_Gourmet_Food.json`

<!-- vim-markdown-toc Redcarpet -->

* [Установка Rust](#установка-rust)
* [Troubleshooting](#troubleshooting)
    * [`error: linking with cc` while `cargo run -p db`](#error-linking-with-cc-while-cargo-run-p-db)
* [Подготовка данных для flask](#подготовка-данных-для-flask)

<!-- vim-markdown-toc -->

Для извлечения данных из `../data/meta_Grocery_and_Gourmet_Food.json.zip` была написана утилита на языке Rust

## Установка Rust

https://www.rust-lang.org/tools/install

- json.csv.zip

- normalized.csv.zip
- category.csv.zip
- also_view.csv.zip

[Подробное описание](../data/README.md)

## Troubleshooting


### `error: linking with cc` while `cargo run -p db`

```
error: linking with `cc` failed: exit code: 1
```

https://github.com/rust-lang/rust/issues/25289 gives an answer:

```
sudo apt install -y gcc-multilib
```

but this solves problem:

```
sudo apt-get install -y libsqlite3-dev
```

## Подготовка данных для flask

```
./db.bash
```
