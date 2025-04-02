diesel::table! {
    #[allow(non_snake_case)]
    ItemTable (key) {
        key -> Text,
        value -> Binary,
    }
}
