table! {
    accounts (id) {
        id -> Int4,
        client_id -> Int4,
        balance_jpy -> Numeric,
        balance_btc -> Numeric,
        balance_eth -> Numeric,
        created_at -> Timestamp,
    }
}

table! {
    clients (id) {
        id -> Int4,
        opened_at -> Timestamp,
        business_name -> Nullable<Text>,
        business_address -> Nullable<Text>,
        api_usage_count -> Int4,
        api_usage_second -> Int4,
        commission_fixed_currency -> Int2,
        commission_fixed_amount -> Float4,
        commission_fixed_bp -> Float4,
    }
}

table! {
    contacts (client_id) {
        client_id -> Int4,
        name -> Text,
        position -> Nullable<Text>,
        phone -> Nullable<Text>,
        email -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

table! {
    fiat_institutions (id) {
        id -> Int4,
        client_id -> Int4,
        name -> Text,
        code -> Nullable<Text>,
        routing_code -> Nullable<Text>,
        account_name -> Nullable<Text>,
        account_code -> Nullable<Text>,
        link_code -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

table! {
    orders (id) {
        id -> Int8,
        account_id -> Int8,
        placed_at -> Timestamp,
        initial_process_at -> Nullable<Timestamp>,
        asset_from -> Int2,
        asset_to -> Int2,
        side -> Int2,
        #[sql_name = "type"]
        type_ -> Int2,
        amount -> Numeric,
        filled -> Numeric,
        status -> Int2,
        closed_at -> Nullable<Timestamp>,
        commission_currency -> Int2,
        commission_fixed -> Nullable<Float4>,
        commission_bp -> Nullable<Float4>,
        tracking_code -> Nullable<Text>,
    }
}


allow_tables_to_appear_in_same_query!(
    accounts,
    clients,
    contacts,
    fiat_institutions,
    orders,
);
