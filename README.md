![Title PNG "Skill Factory"](/assets/skillfactory_logo.png)
# Проект №4. Рекомендательные системы

<!-- vim-markdown-toc Redcarpet -->

* [Задача](#задача)
* [Production-сервис](#production-сервис)
    * [Как посмотреть?](#как-посмотреть)
        * [На heroku.com](#на-heroku-com)
        * [На собственном сервере](#на-собственном-сервере)
        * [На локальной машине](#на-локальной-машине)
    * [Описание](#описание)
    * [Детали реализации](#детали-реализации)
* [Инструкции](#инструкции)
* [Структура репозитория](#структура-репозитория)

<!-- vim-markdown-toc -->


## Задача

https://www.kaggle.com/c/recommendationsv4/overview


## Production-сервис

### Как посмотреть?

#### На heroku.com

https://evening-badlands-35661.herokuapp.com/

Некоторые страницы могут не отображаться из-за низкой производительности сервера

#### На собственном сервере

http://bikuzin18.baza-winner.ru:42420/ 

Работает стабильнее, чем на heroku, если, конечно, сервис не выключен на собственном сервере

#### На локальной машине

Лучшим вариантом знакомства с production-сервисом будет разворачиваине его на локальной машине следующей командой:

```
docker rm recommend; docker run -it --name recommend -e PORT=9000 -p 42420:9000 bazawinner/prod-recommend-flask:9
```

Результат можно увидеть в браузере: http://localhost:42420

### Описание

Целью production-сервиса является демонстрация применения модели, полученной в результате решения [задачи](https://www.kaggle.com/c/recommendationsv4/overview)

Поскольку модель для рекомендательной системы мы строим на основе отзывов пользователей на товары, то и рекомендовать мы будет товары пользователю production-сервиса, а именно: когда пользователь просматривает карточку товара, то мы дополнительно на этой карточке показываем раздел "We recommend" ([например](https://evening-badlands-35661.herokuapp.com/item/20)), основываясь на анализе имющихся отзывов пользователей

Рекомендации не являются персональными. Модель для рекомендаций формируется однажды, перед выпуском сервиса в production

### Детали реализации

- [Использованные технологии](production-stack.md)
- [Страницы веб-сервиса и маршруты переходов](production-site-map.md)
- [Описание архитектуры](production-architechture.md)
- [Известные недочеты](known-issues.md)

## Инструкции

- [Для участия в совместной работе](collaboraion.md)
- [Для запуска Jupyter-ноутбука в docker-контейнере](jupyter-in-docker.md)
- [Для извлечения данных из `data/meta_Grocery_and_Gourmet_Food.json.zip`](src/rust/json-to-csv.md)
- [Для подготовки данных для production-сервиса](src/rust/data-for-production.md)
- [Для сборки и деплоя production-сервиса](go-to-production.md)

## Структура репозитория

В папке [data](data) находятся zip-файлы из https://www.kaggle.com/c/recommendationsv4/data

В корневой папке находятся все ноутбуки:

- [baseline-logreg.ipynb](baseline-logreg.ipynb) - адаптированный ноутбук https://www.kaggle.com/dmitriykrylov/baseline-logreg с https://www.kaggle.com/c/recommendationsv4/notebooks. Этот ноутбук, предварительно скопировав, можно взять за основу своего ноутбука
- [lightfm-lightgbm.ipynb](lightfm-lightgbm.ipynb) - адаптированный ноутбук https://www.kaggle.com/abdualimov/lightfm-lightgbm с https://www.kaggle.com/c/recommendationsv4/notebooks
- [nn-collab-filter.ipynb](nn-collab-filter.ipynb) - адаптированный ноутбук https://www.kaggle.com/abdualimov/nn-collab-filter с https://www.kaggle.com/c/recommendationsv4/notebooks
- [yurybikuzin.ipynb](yurybikuzin.ipynb) - ноутбук [yury bikuzin](https://sfdatasciencecourse.slack.com/team/U016P0Y3CP7)

В папке [books](books) находится [ноутбук](books/u6-p4-books.ipynb) созданный согласно [тренировочному заданию](https://lms.skillfactory.ru/courses/course-v1:Skillfactory+DST-8+13NOV2019/courseware/e3fc9ede1c074eb5819ad1932307daa9/0b9aff51b88044b5af4f860441df0cae/6?activate_block_id=block-v1%3ASkillfactory%2BDST-8%2B13NOV2019%2Btype%40vertical%2Bblock%40d49f77c3903f46ee92322ecb6d7c7ac8)

В папках `dev` и `prod` находятся Dockerfile'ы для создания необходимых образов

В папке `src` находятся исходные коды production-сервиса (`src/flask`) и [rust-утилит](src/rust/README.md)
