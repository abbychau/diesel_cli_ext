table! {
    carry_over_properties (account_id) {
        account_id -> Bytea,
        debit -> Inet,
    }
}


allow_tables_to_appear_in_same_query!(
    carry_over_balances
);