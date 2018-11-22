table! {
    chunks (id) {
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
