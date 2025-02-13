// @generated automatically by Diesel CLI.

diesel::table! {
    video_cards (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}
