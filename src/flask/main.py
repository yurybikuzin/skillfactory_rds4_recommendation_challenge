import sqlite3
from flask import Flask, render_template, request, jsonify, g

app = Flask(__name__)

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
    filter = Filter()
    return render_template('index.html', filter=filter)

@app.route('/filter-main')
def filter():
    filter = Filter()
    return render_template('filter-main.html', filter=filter)

@app.route("/item/<itemid>")
def item(itemid):
    filter = Filter()
    [item] = list_item(itemid)
    return render_template('item.html', item=item, filter=filter)

import builtins
class Filter: 
    _list_sort = None
    _list_brand = None
    _list_cat = None
    _list_price = None
    _list_item = None
    _selected_list_brand = []
    _selected_list_brand_as_str = None
    _selected_list_price = []
    _selected_list_price_as_str = None
    _selected_list_cat = []
    _selected_list_cat_as_str = None
    _selected_list_cart = []
    _selected_list_cart_as_str = None
    _found = None
    _count = None
    _per_page = 20
    _where = None
    _sort = None
    def __init__(self):
        self.is_cart = request.args.get('is_cart')

        self._selected_start = request.args.get('start')
        if self._selected_start is not None:
            self._selected_start = int(self._selected_start)
        else:
            self._selected_start = 0
        self.start_prev = self._selected_start - self._per_page
        self.start_next = self._selected_start + self._per_page

        self._selected_sort = request.args.get('sort')
        if self._selected_sort is not None:
            self._selected_sort = int(self._selected_sort)

        price = request.args.get('price')
        if price is not None: 
            self._selected_list_price = price.split(",")
            self._selected_list_price_as_str = ",".join(self._selected_list_price)

        cat = request.args.get('cat')
        if cat is not None: 
            self._selected_list_cat = cat.split(",")
            self._selected_list_cat_as_str = ",".join(self._selected_list_cat)

        brand = request.args.get('brand')
        if brand is not None: 
            self._selected_list_brand = brand.split(",")
            self._selected_list_brand_as_str = ",".join(self._selected_list_brand)

        cart = request.args.get('cart')
        if cart is not None: 
            self._selected_list_cart = list(map(lambda x: int(x), cart.split(",")))
            self._selected_list_cart_as_str = ",".join(map(lambda x: str(x), self._selected_list_cart))
    def list_sort(self):
        if self._list_sort is None:
            self._list_sort = [ 
                    {"name": "Price: Low to High", "id": 0, 'sql': 'item.price asc'},
                    {"name": "Price: High to Low", "id": 1, 'sql': 'item.price desc'},
                    {"name": "Avg. Customer Review", "id": 2},
                    ]
        return self._list_sort
    def list_price(self):
        if self._list_price is None:
            self._list_price = [ 
                    {"name": "<no price>", "id": 0, 'sql': 'is null'},
                    {"name": "below $5", "id": 1, 'sql': '< 500'},
                    {"name": "$5 to $10", "id": 2, 'sql': 'between 500 and 1000'},
                    {"name": "$10 to $15", "id": 3, 'sql': 'between 1000 and 1500'},
                    {"name": "$15 to $20", "id": 4, 'sql': 'between 1500 and 2000'},
                    {"name": "$20 to $30", "id": 5, 'sql': 'between 2000 and 3000'},
                    {"name": "above $30", "id": 6, 'sql': '> 3000' },
                    ]
        return self._list_price
    def list_brand_len(self):
        return len(self.list_brand())
    def selected_list_cart_len(self):
        return len(self._selected_list_cart)
    def list_cat_len(self):
        return len(self.list_cat())
    def list_item(self):
        if self._list_item is None:
            if self.is_cart:
                if len(self._selected_list_cart) > 0:
                    self._list_item = list_item(self._selected_list_cart_as_str)
            else:
                where = self.where()
                sort = self.sort()
                self._list_item = list_item(where, limit=f'{self._selected_start},{self._per_page}', sort=sort)
        return self._list_item
    def is_bof(self): 
        return self.start_prev < 0
    def is_eof(self):
        return self.start_next >= self.found()
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
            self._list_brand = list(map(lambda row: { "id": -1 if row[0] is None else row[0], "count": row[1], "name": "<no brand>" if row[2] is None else row[2]}, builtins.filter(lambda row: row[2] is not None, cur.fetchall())))
        return self._list_brand
    def found(self):
        if self._found is None:
            where = self.where()
            sql = f'''
                select 
                    count(*)
                from (
                    select item.itemid
                    from item 
                    left join category 
                        on item.itemid = category.itemid
                    {'' if where is None else f'where {where}'}
                    group by item.itemid
                )
            '''
            print(sql)
            cur = get_db().cursor()
            cur.execute(sql)
            (found,) = cur.fetchone()
            self._found = found
        return self._found
    def sort(self):
        if self._sort is None:
            if self._selected_sort is not None:
                self._sort = self.list_sort()[int(self._selected_sort)]['sql']
        return self._sort
    def where(self):
        if self._where is None:
            where = []
            if self._selected_list_cat_as_str is not None:
                where.append(f'category.category_id in ({self._selected_list_cat_as_str})')
            if self._selected_list_brand_as_str is not None:
                where.append(f'item.brand_id in ({self._selected_list_brand_as_str})')
            if len(self._selected_list_price) > 0:
                list_price = self.list_price()
                where_price = []
                for i in self._selected_list_price:
                    where_price.append(f'price {list_price[int(i)]["sql"]}')
                where.append(f'({ " or ".join(where_price)})')
            if len(where) == 0:
                where = None
            else:
                where = " and ".join(where)
            self._where = where
        return self._where
    def count(self):
        if self._count is None:
            self._count = 0
            if len(self._selected_list_brand) > 0:
                self._count += 1
            if len(self._selected_list_cat) > 0:
                self._count += 1
            if len(self._selected_list_price) > 0:
                self._count += 1
        return self._count
    def as_str(self, is_cart=None, start=None, sort=None, price=None, cat=None, brand=None, cart=None):
        if is_cart is None:
            is_cart = self.is_cart
        if start is None:
            start = self._selected_start
        if sort is None:
            sort = self._selected_sort
        if price is None:
            price = self._selected_list_price_as_str
        if cat is None:
            cat = self._selected_list_cat_as_str
        if brand is None:
            brand = self._selected_list_brand_as_str
        if cart is None:
            cart = self._selected_list_cart_as_str
        params = []
        if is_cart:
            params.append( f"is_cart=1")
        if start is not None and start != "" and start != "0":
            params.append( f"start={start}")
        if sort is not None and sort != "":
            params.append( f"sort={sort}")
        if price is not None and price != "":
            params.append( f"price={price}")
        if cat is not None and cat != "":
            params.append( f"cat={cat}")
        if brand is not None and brand != "":
            params.append( f"brand={brand}")
        if cart is not None and cart != "":
            params.append( f"cart={cart}")
        if len(params) == 0:
            return ""
        else:
            return "?" + "&".join(params)
    def with_item_in_cart(self, id):
        if self._selected_list_cart_as_str is None:
            cart = f"{id}"
        elif id not in self._selected_list_cart:
            cart = f"{self._selected_list_cart_as_str},{id}"
        else:
            cart = self._selected_list_cart_as_str
        return self.as_str(cart=cart)
    def without_item_in_cart(self, id):
        cart = ''
        if self._selected_list_cart_as_str is not None:
            if id not in self._selected_list_cart:
                cart = self._selected_list_cart_as_str
            else:
                selected_list= list(builtins.filter(lambda x: (x != id), self._selected_list_cart))
                if len(selected_list) > 0:
                    cart = ",".join(map(lambda x: str(x), selected_list))
        return self.as_str(cart=cart)

def list_item(where, limit=None, sort=None):
    sql = f'''
        select 
            item.itemid,
            itemid_asin.asin,
            dic_title.value,
            item.price,
            dic_description.value,
            dic_brand.value
        from item 
        left join category 
            on item.itemid = category.itemid
        left join itemid_asin on
            item.itemid = itemid_asin.itemid
        left join dic_title on
            item.title_id = dic_title.id
        left join dic_description on
            item.description_id = dic_description.id
        left join dic_brand on
            item.brand_id = dic_brand.id
        {f'where item.itemid in ({where})' if limit is None else '' if where is None or where == '' else f'where {where}'}
        group by item.itemid
        { '' if sort is None else f'order by {sort}'}
        { '' if limit is None else f'limit {limit}'}
    '''
    print(sql)
    cur = get_db().cursor()
    cur.execute(sql)
    return list(map(lambda row: { 
        "itemid": row[0], 
        "asin": row[1], 
        "title": row[2], 
        "price": row[3], 
        "price_whole": None if row[3] is None else row[3] // 100,
        "price_fraction": None if row[3] is None else "{:02d}".format(row[3] - row[3] // 100 * 100),
        "description": '' if row[4] is None else row[4],
        "brand": row[5],
        }, cur.fetchall()))


