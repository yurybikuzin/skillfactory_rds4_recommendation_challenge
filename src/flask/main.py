from flask import Flask

app = Flask(__name__)

from flask import render_template

# https://flask.palletsprojects.com/en/1.1.x/patterns/sqlite3/
import sqlite3
from flask import g

DATABASE = 'data/flask.db'

def get_db():
    db = getattr(g, '_database', None)
    if db is None:
        db = g._database = sqlite3.connect(DATABASE)
    return db

@app.teardown_appcontext
def close_connection(exception):
    db = getattr(g, '_database', None)
    if db is not None:
        db.close()

@app.route("/")
def index():
    cur = get_db().cursor()
    # cur.execute("select value from dic_main_cat")
    # list_main_cat = map(lambda row: row[0], cur.fetchall())
    cur.execute('''
        select 
            dic_main_cat.value, 
            count(*) 
        from item 
        left join dic_main_cat 
            on item.main_cat_id == dic_main_cat.id 
        where dic_main_cat.value = 'Grocery' 
        group by main_cat_id
    ''')
    (main_cat, count) = cur.fetchone()
    print(f"main_cat: {main_cat}, count: {count}");

    cur.execute('''
        select 
            itemid_asin.asin,
            dic_title.value,
            dic_brand.value
        from item 
        left join dic_title 
            on item.title_id == dic_title.id 
        left join dic_brand 
            on item.brand_id == dic_brand.id 
        left join itemid_asin
            on item.itemid == itemid_asin.itemid
        left join dic_main_cat 
            on item.main_cat_id == dic_main_cat.id 
        where dic_main_cat.value = 'Grocery' 
        limit 0,10
    ''')

    # list_items = cur.fetchall()
    list_items = map(lambda row: { "asin": row[0], "title": row[1], "brand": row[2] }, cur.fetchall())
    # dict_main_cat = cur.fetchall();
    print(list_items)
    # return render_template('index.html', list_main_cat=list_main_cat)
    return render_template('index.html', main_cat=main_cat, count=count, list_items=list_items)


# @app.route('/main_cat/<main_cat>')
# def main_cat(main_cat):
#     return render_template('main_cat.html', main_cat=main_cat)
