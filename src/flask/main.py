from flask import Flask

app = Flask(__name__)

list_main_cat = ['Grocery', 'Amazon Home', 'Health & Personal Care', 'All Beauty',
   'Sports & Outdoors', 'Toys & Games', 'Pet Supplies',
   'Industrial & Scientific', 'Office Products', 
   'Tools & Home Improvement', 'Baby', 'Arts, Crafts & Sewing',
   'Musical Instruments', 'Home Audio & Theater', 'Software',
   'Cell Phones & Accessories', 'Camera & Photo']

# import pandas as pd
# import zipfile

# zf = zipfile.ZipFile('C:/Users/Desktop/THEZIPFILE.zip') 
# df = pd.read_csv(zf.open('intfile.csv'))

@app.route("/")
def index():
    return render_template('index.html', list_main_cat=list_main_cat)
    # message = "Hello World again!!!! "
    # return message

from flask import render_template

@app.route('/main_cat/<main_cat>')
def main_cat(main_cat):
    return render_template('main_cat.html', main_cat=main_cat)
