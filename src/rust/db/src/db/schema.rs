table! {
    also_view (id) {
        id -> Integer,
        itemid -> Integer,
        also_view_itemid -> Integer,
        is_train -> Integer,
    }
}

table! {
    category (id) {
        id -> Integer,
        itemid -> Integer,
        category_id -> Integer,
        is_train -> Integer,
    }
}

table! {
    dic_brand (id) {
        id -> Integer,
        value -> Text,
    }
}

table! {
    dic_category (id) {
        id -> Integer,
        value -> Text,
    }
}

table! {
    dic_description (id) {
        id -> Integer,
        value -> Text,
    }
}

table! {
    dic_image (id) {
        id -> Integer,
        value -> Text,
    }
}

table! {
    dic_main_cat (id) {
        id -> Integer,
        value -> Text,
    }
}

table! {
    dic_reviewer_name (id) {
        id -> Integer,
        value -> Text,
    }
}

table! {
    dic_summary (id) {
        id -> Integer,
        value -> Text,
    }
}

table! {
    dic_title (id) {
        id -> Integer,
        value -> Text,
    }
}

table! {
    image (id) {
        id -> Integer,
        train_id -> Integer,
        image_id -> Integer,
    }
}

table! {
    item (itemid) {
        itemid -> Integer,
        brand_id -> Nullable<Integer>,
        description_id -> Nullable<Integer>,
        title_id -> Nullable<Integer>,
        price -> Nullable<Integer>,
        is_train -> Integer,
    }
}

table! {
    itemid_asin (itemid) {
        itemid -> Integer,
        asin -> Nullable<Text>,
    }
}

table! {
    train (id) {
        id -> Integer,
        itemid -> Integer,
        userid -> Integer,
        rating -> Integer,
        overall -> Integer,
        verified -> Integer,
        unix_review_time -> Integer,
        reviewer_name_id -> Nullable<Integer>,
        summary_id -> Nullable<Integer>,
        vote -> Nullable<Integer>,
    }
}

allow_tables_to_appear_in_same_query!(
    also_view,
    category,
    dic_brand,
    dic_category,
    dic_description,
    dic_image,
    dic_main_cat,
    dic_reviewer_name,
    dic_summary,
    dic_title,
    image,
    item,
    itemid_asin,
    train,
);
