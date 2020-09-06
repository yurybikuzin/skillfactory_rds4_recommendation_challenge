# Подготовка sqlite3-базы данных для production

Для подготовки базы данных (sqlite3) для production была написана утилита на языке [Rust](rust.md)

## Запуск утилиты

В папке `src/rust`:

```
cargo run --release -p db
```

## Результат работы

Файл `flask.db` в папке `src/flask/data`

## Development

При разработке утилиты удобно было пользоваться скриптом [`db.bash`](db.bash)
