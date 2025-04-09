// @generated automatically by Diesel CLI.

diesel::table! {
    session_cookies (id) {
        id -> Int4,
        username -> Text,
        val -> Text,
    }
}
