diesel::table! {
    #[expect(non_snake_case, reason = "diesel table name is intentionally PascalCase")]
    ItemTable (key) {
        key -> Text,
        value -> Binary,
    }
}
