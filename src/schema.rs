// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Unsigned<Bigint>,
        username -> Text,
        role -> Nullable<Text>,
    }
}
