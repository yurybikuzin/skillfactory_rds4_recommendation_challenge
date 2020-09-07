# Архитектура production-сервиса

<!-- vim-markdown-toc Redcarpet -->

* [Общая информация](#общая-информация)
* [Реализации логики переходов](#реализации-логики-переходов)
* [Особенности](#особенности)
    * [PORT](#port)
    * [APP_ROOT](#app_root)

<!-- vim-markdown-toc -->

## Общая информация

Production-сервис - это [веб-сервис](production-site-map.md)

Оформлен как монолитный docker-контейнер (1.25GB), включающий в себя как логику (Flask) так и данные (sqlite3)

Это позволяет разворачивать его одной командой:

```
docker rm recommend; docker run -it --name recommend -e PORT=9000 -p 42420:9000 bazawinner/prod-recommend-flask:9
```

## Реализации логики [переходов](production-site-map.md)

Логика на Flask/Python описана в одном файле [src/flask/main.py](https://github.com/yurybikuzin/skillfactory_rds4_recommendation_challenge/blob/master/src/flask/main.py) размером 440 строк

Реализована обработка трех route'ов:

- корневой (`/`). Отвечает за список товаров для выбора, возможно отфильтрованный/отсортированный. Также отвечает за список выбранных товаров (в корзине).

- фильтры (`/filter-main`). Отвечает за формирование содержимого "выезжающей" страницы с фильрами/параметрами сортироваки списка товаров для выбора

- карточка товара (`/item/<itemid>`). Отвечает за формирование карточки товара, независимо от того, откуда мы в нее перешли (из спииска товаров для выбора, из корзины или из списка рекомендованных товаров). Также отвечает за формирование страницы отзывов на товар.

Все параметры состояния (параметр сортировки, параметры фильтрации, признак корзина/общий список, номер страницы списка товаров/отзывов) передаются от клиента к серверу как [query-параметры](https://launchschool.com/books/http/read/what_is_a_url) url'а

За чтение и обработку query-параметров на стороне сервера отвечает [class Filter](https://github.com/yurybikuzin/skillfactory_rds4_recommendation_challenge/blob/master/src/flask/main.py#L46)

## Особенности

### PORT

При запуске docker-контейнера необходимо передавать переменную среды `PORT`, на который "вешается" сервис внутри контейнера. Это сделано согласно [требованиям heroku.com](https://help.heroku.com/PPBPA231/how-do-i-use-the-port-environment-variable-in-container-based-apps)

### APP_ROOT

Дополнительно при запуске контейнера можно передать переменную среды `APP_ROOT` (например, `docker run -it --name recommend -e APP_ROOT=amazing -e PORT=9000 -p 42420:9000 bazawinner/prod-recommend-flask:9`), тогда веб-сервис будет доступен по адресу http://localhost:42420/amazing/












