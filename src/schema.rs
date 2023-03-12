// @generated automatically by Diesel CLI.

diesel::table! {
    text_chunk (id) {
        id -> Uuid,
        text_meta_id -> Uuid,
        num -> Int4,
        content -> Text,
    }
}
