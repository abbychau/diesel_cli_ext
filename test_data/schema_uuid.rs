table! {
    carry_over_balances (account_id) {
        id -> Uuid,
        debit -> Numeric,
        description -> Nullable<Text>,
    }
}
