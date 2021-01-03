table! {
    secret_keys (id) {
        id -> Int4,
        secret_key -> Bytea,
    }
}

table! {
    secrets (id) {
        id -> Int4,
        key_id -> Int4,
        hash -> Bytea,
        nonce -> Bytea,
        secret -> Bytea,
    }
}

joinable!(secrets -> secret_keys (key_id));

allow_tables_to_appear_in_same_query!(
    secret_keys,
    secrets,
);
