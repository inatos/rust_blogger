table! {
    comment (id) {
        id -> Int4,
        content -> Varchar,
        post_id -> Int4,
        site_user_id -> Int4,
        parent_comment_id -> Nullable<Int4>,
        created -> Timestamptz,
    }
}

table! {
    post (id) {
        id -> Int4,
        title -> Varchar,
        label -> Text,
        post_group_id -> Int4,
        post_category_id -> Int4,
        content -> Text,
        published -> Bool,
        embedded -> Bool,
        created -> Timestamptz,
        updated -> Timestamptz,
    }
}

table! {
    post_category (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    post_group (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    site_user (id) {
        id -> Int4,
        site_user_status_id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created -> Timestamptz,
        updated -> Timestamptz,
    }
}

table! {
    site_user_status (id) {
        id -> Int4,
        name -> Varchar,
    }
}

joinable!(comment -> post (post_id));
joinable!(comment -> site_user (site_user_id));
joinable!(post -> post_category (post_category_id));
joinable!(post -> post_group (post_group_id));
joinable!(site_user -> site_user_status (site_user_status_id));

allow_tables_to_appear_in_same_query!(
    comment,
    post,
    post_category,
    post_group,
    site_user,
    site_user_status,
);
