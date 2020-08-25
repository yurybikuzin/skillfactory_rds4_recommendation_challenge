#!/usr/bin/env python3

import sqlite3

DATABASE = 'data/flask.db'
db = sqlite3.connect(DATABASE)
cur = db.cursor()

cur.execute('''
    create index category_id on category(category_id);
''')
