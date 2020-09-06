FROM python:3.8.5-slim-buster

RUN pip install flask watchdog 
RUN pip install simplejson python-dotenv blinker
RUN apt-get update && apt-get install -y gcc python3-dev && pip install nmslib

WORKDIR /app
ENV FLASK_APP=main.py
ENV FLASK_ENV=development
CMD ["flask", "run", "--host=0.0.0.0"]

