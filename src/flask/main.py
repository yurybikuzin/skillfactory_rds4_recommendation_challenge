import sqlite3
from flask import Flask, render_template, request, jsonify, g

app = Flask(__name__)

# from flask import render_template
# from flask import request

# https://flask.palletsprojects.com/en/1.1.x/patterns/sqlite3/
# from flask import g

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

@app.route("/script.js")
def script():
    filter = Filter()
    return render_template('script.js', filter=filter)

# @app.route("/", methods=["GET", "POST"])
@app.route("/")
def index():
    # print(request.form)
    # print( jsonify(request.get_json(force=True)) )
    filter = Filter()
    # cur = get_db().cursor()
    # cur.execute('''
    #     select 
    #         count(*) 
    #     from item 
    # ''')
    # (count_item,) = cur.fetchone()

    # cur.execute('''
    #     select 
    #         grouped.category_id,
    #         grouped.count,
    #         dic_category.value
    #     from (
    #         select 
    #    		category_id,
    #    		count(*) as count
    #         from category
    #         group by category_id
    #     ) as grouped
    #     left join dic_category
    #         on grouped.category_id = dic_category.id
    #    	where grouped.count < 10000
    #    	order by grouped.count desc
    # ''')
    # list_category = list(map(lambda row: { "id": row[0], "count": row[1],"name": row[2] }, cur.fetchall()))
    # count_category = len(list_category)
    # return render_template('index.html', count_item=count_item, count_category=count_category, list_category=list_category)
    return render_template('index.html', filter=filter)

# @app.route("/", methods=["POST"])
# def index():
#     filter = Filter()
#     print( jsonify(request.get_json(force=True)) )
#     return render_template('index.html', filter=filter)


# @app.route('/cat/<category_id>')
# def cat(category_id):
#     cat = Cat(category_id)
#     return render_template('cat.html', cat=cat)


@app.route('/filter-main')
def filter():
    filter = Filter()
    return render_template('filter-main.html', filter=filter)

# class Cat:
#     _list_sort = None
#     def __init__(self, category_id):
#         cur = get_db().cursor()
#         self.filter = Filter(category_id)
#         self.category_id = category_id
#         cur.execute(f'''
#             select 
#                 {self.category_id} as category_id,
#                 grouped.count,
#                 dic_category.value
#             from (
#                 select 
#                     category_id,
#                     count(*) as count
#                 from category
#                 where category_id = {self.category_id}
#                 group by category_id
#             ) as grouped
#             left join dic_category
#                 on grouped.category_id = dic_category.id
#             where grouped.count < 10000
#             order by grouped.count desc
#         ''')
#         (self.id, self.count, self.name) = cur.fetchone()
#     def list_sort(self):
#         if self._list_sort is None:
#             self._list_sort = [ 
#                     {"name": "Price: Low to High", "id": 1},
#                     {"name": "Price: High to Low", "id": 2},
#                     {"name": "PriceAvg. Customer Review", "id": 3},
#                     ]
#         return self._list_sort

import builtins
class Filter: 
    _list_sort = None
    _list_brand = None
    _list_cat = None
    _list_price = None
    selected_list_brand = []
    selected_list_brand_as_str = None
    selected_list_price = []
    selected_list_price_as_str = None
    _found = None
    _count = None
    def __init__(self):
        self.sort = request.form.get("sort")
        if self.sort is None:
            self.sort = request.args.get('sort')
        if self.sort is not None:
            self.sort = int(self.sort)
        # print("self.sort", self.sort)
        brand = request.form.getlist('brand')
        print('brand', brand)
        if len(brand) > 0:
            self.selected_list_brand = list(map(lambda x: int(x), brand))
            self.selected_list_brand_as_str = ",".join(map(lambda x: str(x), self.selected_list_brand))
        else:
            brand = request.args.get('brand')
            if brand is not None: 
                self.selected_list_brand = list(map(lambda x: int(x), brand.split(",")))
                self.selected_list_brand_as_str = ",".join(map(lambda x: str(x), self.selected_list_brand))
        cat = request.args.get('cat')
        if cat is not None: 
            self.selected_list_cat = list(map(lambda x: int(x), cat.split(",")))
            self.selected_list_cat_as_str = ",".join(map(lambda x: str(x), self.selected_list_cat))
        price = request.args.get('price')
        if price is not None: 
            self.selected_list_price = list(map(lambda x: int(x), price.split(",")))
            self.selected_list_price_as_str = ",".join(map(lambda x: str(x), self.selected_list_price))
    def list_sort(self):
        if self._list_sort is None:
            self._list_sort = [ 
                    {"name": "Price: Low to High", "id": 1},
                    {"name": "Price: High to Low", "id": 2},
                    {"name": "Avg. Customer Review", "id": 3},
                    ]
        return self._list_sort
    def list_price(self):
        if self._list_price is None:
            self._list_price = [ 
                    {"name": "All prices", "id": -1},
                    {"name": "<no price>", "id": 0},
                    {"name": "below $5", "id": 1},
                    {"name": "$5 to $10", "id": 2},
                    {"name": "$10 to $15", "id": 3},
                    {"name": "$15 to $20", "id": 4},
                    {"name": "$20 to $30", "id": 5},
                    {"name": "above $30", "id": 6},
                    ]
        return self._list_price
    def list_brand_len(self):
        return len(self.list_brand())
    def list_cat_len(self):
        return len(self.list_cat())
    def list_cat(self):
        if self._list_cat is None:
            cur = get_db().cursor()
            cur.execute('''
                select 
                    grouped.category_id,
                    grouped.count,
                    dic_category.value
                from (
                    select 
                        category_id,
                        count(*) as count
                    from category
                    group by category_id
                ) as grouped
                left join dic_category
                    on grouped.category_id = dic_category.id
                where grouped.count between 2 and 10000
                order by grouped.count desc, length(dic_category.value)
            ''')
            self._list_cat = list(map(lambda row: { "id": row[0], "count": row[1], "name": row[2]}, cur.fetchall()))
        return self._list_cat
    def list_brand(self):
        if self._list_brand is None:
            cur = get_db().cursor()
            cur.execute(f'''
                select 
                    item.brand_id,
                    count(*) as count,
                    dic_brand.value
                from item 
                left join dic_brand on
                    item.brand_id = dic_brand.id
                group by item.brand_id
                order by count desc
            ''')
            self._list_brand = list(map(lambda row: { "id": -1 if row[0] is None else row[0], "count": row[1], "name": "<no brand>" if row[2] is None else row[2]}, cur.fetchall()))
        return self._list_brand
    def found(self):
        if self._found is None:
            sql = f'''
                select 
                    count(*) as count
                from item 
            '''
                # left join category on
                #     item.itemid = category.itemid
                # where category.category_id = {self.category_id}
            # print(self.selected_list_brand_as_str)
            # if self.selected_list_brand_as_str is not None:
            #     sql = sql + f"and brand_id in ({self.selected_list_brand_as_str})"
            cur = get_db().cursor()
            cur.execute(sql)
            (found,) = cur.fetchone()
            self._found = found
        return self._found
    def count(self):
        if self._count is None:
            if len(self.selected_list_brand) == 0:
                self._count = 0
            else:
                self._count = 1
        return self._count
    def as_str(self, sort=None, brand=None, price=None):
        if sort is None:
            sort = self.sort
        if brand is None:
            brand = self.selected_list_brand_as_str
        if price is None:
            price = self.selected_list_price_as_str
        params = []
        if sort is not None and sort != "":
            params.append( f"sort={sort}")
        if brand is not None and brand != "":
            params.append( f"brand={brand}")
        if price is not None and price != "":
            params.append( f"price={price}")
        if len(params) == 0:
            return ""
        else:
            return "?" + "&".join(params)
    def with_sort(self, sort):
        return self.as_str(sort=sort)
    def without_sort(self, sort):
        return self.as_str(sort="")
    def with_brand(self, id):
        if self.selected_list_brand_as_str is None:
            brand = f"{id}"
        else:
            brand = f"{self.selected_list_brand_as_str},{id}"
        return self.as_str(brand=brand)
    def without_brand(self, id):
        brand = ""
        if len(self.selected_list_brand) > 0:
            selected_list_brand = list(builtins.filter(lambda x: (x != id), self.selected_list_brand))
            if len(selected_list_brand) > 0:
                brand = ",".join(map(lambda x: str(x), selected_list_brand))
        return self.as_str(brand=brand)
    def with_price(self, id):
        if id < 0:
            price = ""
        elif self.selected_list_price_as_str is None:
            price = f"{id}"
        else:
            price = f"{self.selected_list_price_as_str},{id}"
        return self.as_str(price=price)
    def without_price(self, id):
        price = ""
        if id < 0 and len(self.selected_list_price) == 0:
            price = None
        elif id >= 0 and len(self.selected_list_price) > 0:
            selected_list_price = list(builtins.filter(lambda x: (x != id), self.selected_list_price))
            if len(selected_list_price) > 0:
                price = ",".join(map(lambda x: str(x), selected_list_price))
        return self.as_str(price=price)

