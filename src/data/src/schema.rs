table! {
    albums (id) {
        id -> Integer,
        name -> Nullable<Varchar>,
        year -> Nullable<Varchar>,
        artist_id -> Integer,
        image -> Nullable<Varchar>,
        external_id -> Nullable<Varchar>,
    }
}

table! {
    artists (id) {
        id -> Integer,
        name -> Nullable<Varchar>,
        external_id -> Nullable<Varchar>,
    }
}

table! {
    songs (id) {
        id -> Integer,
        name -> Nullable<Varchar>,
        track_num -> Nullable<Integer>,
        duration -> Nullable<Integer>,
        album_id -> Nullable<Integer>,
        external_id -> Nullable<Varchar>,
    }
}

table! {
    tasks (id) {
        id -> Integer,
        name -> Nullable<Varchar>,
        status -> Integer,
        task_type -> Integer,
    }
}

joinable!(albums -> artists (artist_id));
joinable!(songs -> albums (album_id));

allow_tables_to_appear_in_same_query!(
    albums,
    artists,
    songs,
    tasks,
);
