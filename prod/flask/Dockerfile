FROM python:3.8-slim

RUN true && \
    apt-get clean && \ 
    apt-get -y update && \ 
true

RUN true && \
    apt-get -y install curl python3-dev build-essential gcc && \ 
true

ADD ./prod/flask/requirements.txt /tmp/requirements.txt

# Install dependencies
RUN pip3 install --no-cache-dir -q -r /tmp/requirements.txt

# Add our code
WORKDIR /app
ADD src/flask/data data
ADD src/flask/templates templates
ADD src/flask/static static
COPY src/flask/main.py main.py
COPY prod/flask/uwsgi.ini uwsgi.ini

# USER www-data

# Run the app.  CMD is required to run on Heroku
CMD uwsgi --ini uwsgi.ini

