table! {
    carry_over_balances (account_id) {
        account_id -> BigInt,
        debit -> Nullable<Text>,
        credit -> Nullable<Text>,
    }
}

table! {
    orders (id1, id2) {
        id1 -> BigInt,
        id2 -> BigInt,
        json -> Text,
    }
}
table! {
    history (id) {
        id -> Int8,
        created -> Timestamp,
        key -> Int8,
        success -> Bool,
    }
}

table! {
    hmac_sha256_active (key) {
        key -> Int8,
        created -> Timestamp,
        secret -> Varchar,
        account_id -> Int8,
        nonce -> Int8,
    }
}

table! {
    hmac_sha256_removed (key) {
        key -> Int8,
        created -> Timestamp,
        removed -> Timestamp,
        account_id -> Int8,
        nonce -> Int8,
    }
}

allow_tables_to_appear_in_same_query!(
    history,
    hmac_sha256_active,
    hmac_sha256_removed,
);
allow_tables_to_appear_in_same_query!(
    carry_over_balances,
    orders,
);