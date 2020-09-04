-- Your SQL goes here
drop table if exists train;
create table train (
    id integer primary key not null,
    itemid integer not null,
    userid integer not null,
    rating integer not null,
    overall integer not null,
    verified integer not null,
    unix_review_time integer not null,
    reviewer_name_id integer,
    review_text text,
    summary_id integer,
    vote integer
);
create index itemid on train(itemid,userid);
create index userid on train(userid,itemid);
create index unix_review_time on train(unix_review_time);
create index vote on train(vote);

drop table if exists dic_reviewer_name;
create table dic_reviewer_name (
    id integer primary key not null,
    value text not null unique
);
drop table if exists dic_summary;
create table dic_summary (
    id integer primary key not null,
    value text not null unique
);
drop table if exists dic_image;
create table dic_image (
    id integer primary key not null,
    value text not null unique
);
drop table if exists image;
create table image (
    id integer primary key not null,
    train_id integer not null,
    image_id integer not null
);
drop table if exists item;
create table item (
    itemid integer primary key not null,
    brand_id integer,
    description_id integer,
    title_id integer,
    price integer,
    is_train integer not null
);
create index price on item(price);
drop table if exists dic_brand;
create table dic_brand (
    id integer primary key not null,
    value text not null unique
);
drop table if exists dic_description;
create table dic_description (
    id integer primary key not null,
    value text not null unique
);
drop table if exists dic_title;
create table dic_title (
    id integer primary key not null,
    value text not null unique
);
drop table if exists category;
create table category (
    id integer primary key not null,
    itemid integer not null,
    category_id integer not null,
    is_train integer not null
);
create index category_id on category(category_id);
drop table if exists dic_category;
create table dic_category (
    id integer primary key not null,
    value text not null unique
);
drop table if exists also_view;
create table also_view (
    id integer primary key not null,
    itemid integer not null,
    also_view_itemid integer not null,
    is_train integer not null
);
drop table if exists itemid_asin;
create table itemid_asin (
    itemid integer primary key not null,
    asin text non null
);
