
# Запуск Jupyter-ноутбука в docker-контейнере

После [установки Docker](https://docs.docker.com/engine/install/) (и выполнения, в случае Linux, [Post-installation steps for Linux](https://docs.docker.com/engine/install/linux-postinstall/)) надо в терминале перейти в корневую папку репозитория и выполнить команду:

В Windows Command Line (`cmd`):

```
docker run -m 4096m -p 8888:8888 -v %cd%:/home/jovyan/work bazawinner/dev-recommend-proj:7
```

В Windows Power Shell, macOS:

```
docker run -m 4096m -p 8888:8888 -v ${PWD}:/home/jovyan/work bazawinner/dev-recommend-proj:7
```

Про опцию `-m 4096m` [см. подробнее](https://stackoverflow.com/questions/43460770/docker-windows-container-memory-limit#:~:text=If%20you%20run%20docker%20containers,m%22%20option%20for%20docker%20run.)

В linux:

```
docker run -p 8888:8888 -v ${PWD}:/home/jovyan/work bazawinner/dev-recommend-proj:7
```

После выполнения команды появиться подобный вывод:

```
Executing the command: jupyter notebook
[I 10:53:34.456 NotebookApp] Writing notebook server cookie secret to /home/jovyan/.local/share/jupyter/runtime/notebook_cookie_secret
[I 10:53:35.006 NotebookApp] JupyterLab extension loaded from /opt/conda/lib/python3.7/site-packages/jupyterlab
[I 10:53:35.006 NotebookApp] JupyterLab application directory is /opt/conda/share/jupyter/lab
[I 10:53:35.008 NotebookApp] Serving notebooks from local directory: /home/jovyan
[I 10:53:35.008 NotebookApp] The Jupyter Notebook is running at:
[I 10:53:35.008 NotebookApp] http://a7d8a2ffdd6e:8888/?token=ebfc449f52d47aea7a98db8c7a323710cf615e98ee21bcfc
[I 10:53:35.008 NotebookApp]  or http://127.0.0.1:8888/?token=ebfc449f52d47aea7a98db8c7a323710cf615e98ee21bcfc
[I 10:53:35.009 NotebookApp] Use Control-C to stop this server and shut down all kernels (twice to skip confirmation).
[C 10:53:35.011 NotebookApp] 
    To access the notebook, open this file in a browser:
        file:///home/jovyan/.local/share/jupyter/runtime/nbserver-7-open.html
    Or copy and paste one of these URLs:
        http://a7d8a2ffdd6e:8888/?token=ebfc449f52d47aea7a98db8c7a323710cf615e98ee21bcfc
     or http://127.0.0.1:8888/?token=ebfc449f52d47aea7a98db8c7a323710cf615e98ee21bcfc
 ```

Url из последней строки вывода (в приведенном примере это http://127.0.0.1:8888/?token=ebfc449f52d47aea7a98db8c7a323710cf615e98ee21bcfc) надо вставить в адресную строку браузера

Должна появится подобная картинка:

![Изображение Jupyter-ноутбука](assets/jupyter-notebook.png "Изображение Jupyter-ноутбука")

Теперь достаточно кликнуть на папке work, чтобы увидеть содержимое корневой папки репозитория, включая доступные ноутбуки:

![Изображение Jupyter-ноутбука](assets/jupyter-notebook-work.png "Изображение Jupyter-ноутбука")

