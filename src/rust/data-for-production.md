# Подготовка sqlite3-базы данных для production

Для подготовки базы данных (sqlite3) для production-сервиса была написана утилита на языке [Rust](rust.md)

## Подготовка данных для запуска утилиты

Необходимо [извлечь данные](json-to-csv.md) из `data/meta_Grocery_and_Gourmet_Food.json.zip`

## Запуск утилиты

В папке `src/rust`:

```
cargo run --release -p db
```

## Результат работы

Файл `flask.db` в папке `src/flask/data`

## Development

При разработке утилиты удобно было пользоваться скриптом [`db.bash`](db.bash)
