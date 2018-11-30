table! {
    chunk (id) {
        id -> Int4,
        index_id -> Int4,
        name -> Text,
        size -> Int4,
        creation_time -> Text,
        accessed_time -> Text,
        tags -> Nullable<Array<Int4>>,
        stats_download_count -> Int4,
    }
}

table! {
    chunks (id) {
        id -> Int4,
        index -> Text,
        name -> Text,
        size -> Int4,
    }
}

table! {
    tag (id) {
        id -> Int4,
        name -> Text,
        creation_time -> Timestamptz,
        accessed_time -> Timestamptz,
    }
}

allow_tables_to_appear_in_same_query!(
    chunk,
    chunks,
    tag,
);
