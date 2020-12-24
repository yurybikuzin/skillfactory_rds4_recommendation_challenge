# Сборка production-образа и деплой


<!-- vim-markdown-toc Redcarpet -->

* [Сборка](#сборка)
* [Деплой](#деплой)
* [Процедура деплоя на heroku](#процедура-деплоя-на-heroku)
    * [Ссылки](#ссылки)
* [Процедура деплоя на собственный сервер](#процедура-деплоя-на-собственный-сервер)

<!-- vim-markdown-toc -->

## Сборка

Для сборки образа использовался скрипт [docker-image.sh](docker-image.sh)

```
./docker-image.sh prod flask
```

## Деплой 

Деплой производился дважды:
- на [heroku.com](https://heroku.com): https://evening-badlands-35661.herokuapp.com/ 
- и на собственный сервер: http://bikuzin18.baza-winner.ru:42420/

После деплоя на [heroku.com](https://heroku.com) выяснилось, что ресурсов (в первую очередь памяти - 512MB), предоставляемых [heroku.com](https://heroku.com) в бесплатном режиме, недостаточно для работы сервиса (а именно для работы с базой данных, только файл `flask.db` которой занимает 541MB. Поэтому, когда на сервер приходят запросы, связанные с фильтрацией/сортировкой, то время обработка таких запросов часто превышает время ожидания http-сервера `heroku`, и пользователь вместо ответа получает ошибку приложения на сервере)

Поэтому была предпринята попытка произвести деплой на собственный сервер. Тоже достаточно маломощный (1 ядро, 1GB памяти под Ubuntu18). Ключевое отличие этого деплоя от деплоя на heroku в том, что запросы к сервису поступают напрямую, минуя промежуточный http-сервер (nginx), и лимита времени на их выполнения нет. Поэтому пользователь, при достаточном терпении, может дождаться результата выполнения дажы самых "тяжелых" запросов, связанных с сортировкой/фильтрацией

Лучшим вариантом знакомства с production-сервисом будет разворачиваине его на локальной машине следующей командой:

```
docker rm recommend; docker run -it --name recommend -e PORT=9000 -p 42420:9000 bazawinner/prod-recommend-flask:9
```

Результат можно увидеть в браузере: http://localhost:42420

## Процедура деплоя на heroku

```
sudo snap install heroku --classic
heroku login
heroku container:login
mkdir ~/recommend_deploy
cd ~/recommend_deploy
heroku create # команда вернула evening-badlands-35661
export heroku_app=evening-badlands-35661
git init
heroku git:remote -a $heroku_app
docker tag bazawinner/prod-recommend-flask:9 registry.heroku.com/$heroku_app/web
heroku container:login
docker push registry.heroku.com/$heroku_app/web
heroku container:release web
heroku open
```

### Ссылки 
- https://devcenter.heroku.com/articles/container-registry-and-runtime
- https://help.heroku.com/PPBPA231/how-do-i-use-the-port-environment-variable-in-container-based-apps

## Процедура деплоя на собственный сервер

```
ssh bikuzin18
docker rm recommend; docker run -it --name recommend -e PORT=9000 -p 42420:9000 bazawinner/prod-recommend-flask:9
```

