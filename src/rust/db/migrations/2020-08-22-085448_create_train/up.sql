-- Your SQL goes here
drop table if exists train;
create table train (
    id integer primary key not null,
    userid integer not null,
    itemid integer not null,
    rating integer not null,
    overall integer not null,
    verified integer not null,
    unix_review_time integer not null,
    reviewer_name_id integer,
    summary_id integer,
    vote integer
);
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
    brand text,
    description text,
    title text,
    main_cat_id integer,
    price text integer,
    is_train integer not null
);
drop table if exists dic_main_cat;
create table dic_main_cat (
    id integer primary key not null,
    value text not null unique
);
drop table if exists category;
create table category (
    id integer primary key not null,
    itemid integer not null,
    category_id integer not null,
    is_train integer not null,
);
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
    is_train integer not null,
);
