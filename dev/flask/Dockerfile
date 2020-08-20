FROM python:3.8.5-slim-buster

RUN pip install flask watchdog 
RUN pip install simplejson python-dotenv blinker

WORKDIR /app
ENV FLASK_APP=main.py
ENV FLASK_ENV=development
CMD ["flask", "run", "--host=0.0.0.0"]

