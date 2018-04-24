# Diesel CLI Extension

Diesel CLI Extension is a tool-belt that aids Diesel CLI after it built schema.rs .

## How to use

First of all, `diesel print-schema > src/schema.rs` 

### To generate model structs:
`diesel_ext > src/models.rs`, `diesel_ext model > src/models.rs` (it is the default option)

Sample output:
``` rust
use chrono::NaiveDateTime;
use bigdecimal::BigDecimal;

#[derive(Queryable)]
pub struct Account {
    pub id : i32,
    pub client_id : i32,
    pub balance_jpy : BigDecimal,
    pub balance_btc : BigDecimal,
    pub balance_eth : BigDecimal,
    pub created_at : NaiveDateTime,
}

#[derive(Queryable)]
pub struct Client {
    pub id : i32,
    pub opened_at : NaiveDateTime,
    pub business_name : String,
    pub business_address : String,
    pub api_usage_count : i32,
    pub api_usage_second : i32,
    pub commission_fixed_currency : i16,
    pub commission_fixed_amount : f32,
    pub commission_fixed_bp : f32,
}

#[derive(Queryable)]
pub struct Contact {
    pub client_id : i32,
    pub name : String,
    pub position : String,
    pub phone : String,
    pub email : String,
    pub created_at : NaiveDateTime,
}

#[derive(Queryable)]
pub struct FiatInstitution {
    pub id : i32,
    pub client_id : i32,
    pub name : String,
    pub code : String,
    pub routing_code : String,
    pub account_name : String,
    pub account_code : String,
    pub link_code : String,
    pub created_at : NaiveDateTime,
}

#[derive(Queryable)]
pub struct Order {
    pub id : i64,
    pub account_id : i64,
    pub placed_at : NaiveDateTime,
    pub initial_process_at : NaiveDateTime,
    pub asset_from : i16,
    pub asset_to : i16,
    pub side : i16,
    pub type_ : i16,
    pub amount : BigDecimal,
    pub filled : BigDecimal,
    pub status : i16,
    pub closed_at : NaiveDateTime,
    pub commission_currency : i16,
    pub commission_fixed : f32,
    pub commission_bp : f32,
    pub tracking_code : String,
}
```

### To generate prelimitive proto file:
`diesel_ext proto > myproto.proto`

Sample output:
``` r
syntax = "proto3";


message Account {
    int32 id = 1;
    int32 client_id = 2;
    string balance_jpy = 3;
    string balance_btc = 4;
    string balance_eth = 5;
    string created_at = 6;
}
message Client {
    int32 id = 1;
    string opened_at = 2;
    string business_name = 3;
    string business_address = 4;
    int32 api_usage_count = 5;
    int32 api_usage_second = 6;
    int32 commission_fixed_currency = 7;
    float commission_fixed_amount = 8;
    float commission_fixed_bp = 9;
}
message Contact {
    int32 client_id = 1;
    string name = 2;
    string position = 3;
    string phone = 4;
    string email = 5;
    string created_at = 6;
}
message FiatInstitution {
    int32 id = 1;
    int32 client_id = 2;
    string name = 3;
    string code = 4;
    string routing_code = 5;
    string account_name = 6;
    string account_code = 7;
    string link_code = 8;
    string created_at = 9;
}
message Order {
    int64 id = 1;
    int64 account_id = 2;
    string placed_at = 3;
    string initial_process_at = 4;
    int32 asset_from = 5;
    int32 asset_to = 6;
    int32 side = 7;
    int32 type_ = 8;
    string amount = 9;
    string filled = 10;
    int32 status = 11;
    string closed_at = 12;
    int32 commission_currency = 13;
    float commission_fixed = 14;
    float commission_bp = 15;
    string tracking_code = 16;
}


message EnquireAccountRequest {
    int64 id =1;
}
message EnquireClientRequest {
    int64 id =1;
}
message EnquireContactRequest {
    int64 id =1;
}
message EnquireFiatInstitutionRequest {
    int64 id =1;
}
message EnquireOrderRequest {
    int64 id =1;
}


service MessageRpc {
    rpc getAccount (EnquireAccountRequest) returns (Account) { }
    rpc getClient (EnquireClientRequest) returns (Client) { }
    rpc getContact (EnquireContactRequest) returns (Contact) { }
    rpc getFiatInstitution (EnquireFiatInstitutionRequest) returns (FiatInstitution) { }
    rpc getOrder (EnquireOrderRequest) returns (Order) { }
}
```

### To generate proto conversions:
`diesel_ext from_proto > proto/src/conversion/from_proto.rs`, `diesel_ext into_proto > proto/src/conversion/into_proto.rs`

Sample output:

``` rust
use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use models;
use proto::_name_;
use std::str::FromStr;
use std::convert::From;

fn str2Ndt(str: &str) -> NaiveDateTime {
    NaiveDateTime::parse_from_str(str, "%Y-%m-%d %H:%M:%S").unwrap()
}



impl From<_name_::Account> for models::Account {
    fn from(i: _name_::Account) -> Self {
        models::Account{
            id: i.get_id(),
            client_id: i.get_client_id(),
            balance_jpy: i.get_balance_jpy().to_string(),
            balance_btc: i.get_balance_btc().to_string(),
            balance_eth: i.get_balance_eth().to_string(),
            created_at: i.get_created_at().to_string(),
        }
    }
}

impl From<_name_::Client> for models::Client {
    fn from(i: _name_::Client) -> Self {
        models::Client{
            id: i.get_id(),
            opened_at: i.get_opened_at().to_string(),
            business_name: i.get_business_name().to_string(),
            business_address: i.get_business_address().to_string(),
            api_usage_count: i.get_api_usage_count(),
            api_usage_second: i.get_api_usage_second(),
            commission_fixed_currency: i.get_commission_fixed_currency(),
            commission_fixed_amount: i.get_commission_fixed_amount(),
            commission_fixed_bp: i.get_commission_fixed_bp(),
        }
    }
}

impl From<_name_::Contact> for models::Contact {
    fn from(i: _name_::Contact) -> Self {
        models::Contact{
            client_id: i.get_client_id(),
            name: i.get_name().to_string(),
            position: i.get_position().to_string(),
            phone: i.get_phone().to_string(),
            email: i.get_email().to_string(),
            created_at: i.get_created_at().to_string(),
        }
    }
}

impl From<_name_::FiatInstitution> for models::FiatInstitution {
    fn from(i: _name_::FiatInstitution) -> Self {
        models::FiatInstitution{
            id: i.get_id(),
            client_id: i.get_client_id(),
            name: i.get_name().to_string(),
            code: i.get_code().to_string(),
            routing_code: i.get_routing_code().to_string(),
            account_name: i.get_account_name().to_string(),
            account_code: i.get_account_code().to_string(),
            link_code: i.get_link_code().to_string(),
            created_at: i.get_created_at().to_string(),
        }
    }
}

impl From<_name_::Order> for models::Order {
    fn from(i: _name_::Order) -> Self {
        models::Order{
            id: i.get_id(),
            account_id: i.get_account_id(),
            placed_at: i.get_placed_at().to_string(),
            initial_process_at: i.get_initial_process_at().to_string(),
            asset_from: i.get_asset_from(),
            asset_to: i.get_asset_to(),
            side: i.get_side(),
            type_: i.get_type_(),
            amount: i.get_amount().to_string(),
            filled: i.get_filled().to_string(),
            status: i.get_status(),
            closed_at: i.get_closed_at().to_string(),
            commission_currency: i.get_commission_currency(),
            commission_fixed: i.get_commission_fixed(),
            commission_bp: i.get_commission_bp(),
            tracking_code: i.get_tracking_code().to_string(),
        }
    }
}
```

``` rust
use bigdecimal::BigDecimal;
use models;
use proto::_name_;
use protobuf::RepeatedField;
use std::collections::HashMap;
use std::convert::From;



impl From<models::Account> for _name_::Account {
    fn from(i: models::Account) -> Self {
        let mut o = _name_::Account::new();
        o.set_id(i.id.into());
        o.set_client_id(i.client_id.into());
        o.set_balance_jpy(i.balance_jpy.to_string());
        o.set_balance_btc(i.balance_btc.to_string());
        o.set_balance_eth(i.balance_eth.to_string());
        o.set_created_at(i.created_at.to_string());
        o
    }
}

impl From<models::Client> for _name_::Client {
    fn from(i: models::Client) -> Self {
        let mut o = _name_::Client::new();
        o.set_id(i.id.into());
        o.set_opened_at(i.opened_at.to_string());
        o.set_business_name(i.business_name.to_string());
        o.set_business_address(i.business_address.to_string());
        o.set_api_usage_count(i.api_usage_count.into());
        o.set_api_usage_second(i.api_usage_second.into());
        o.set_commission_fixed_currency(i.commission_fixed_currency.into());
        o.set_commission_fixed_amount(i.commission_fixed_amount.into());
        o.set_commission_fixed_bp(i.commission_fixed_bp.into());
        o
    }
}

impl From<models::Contact> for _name_::Contact {
    fn from(i: models::Contact) -> Self {
        let mut o = _name_::Contact::new();
        o.set_client_id(i.client_id.into());
        o.set_name(i.name.to_string());
        o.set_position(i.position.to_string());
        o.set_phone(i.phone.to_string());
        o.set_email(i.email.to_string());
        o.set_created_at(i.created_at.to_string());
        o
    }
}

impl From<models::FiatInstitution> for _name_::FiatInstitution {
    fn from(i: models::FiatInstitution) -> Self {
        let mut o = _name_::FiatInstitution::new();
        o.set_id(i.id.into());
        o.set_client_id(i.client_id.into());
        o.set_name(i.name.to_string());
        o.set_code(i.code.to_string());
        o.set_routing_code(i.routing_code.to_string());
        o.set_account_name(i.account_name.to_string());
        o.set_account_code(i.account_code.to_string());
        o.set_link_code(i.link_code.to_string());
        o.set_created_at(i.created_at.to_string());
        o
    }
}

impl From<models::Order> for _name_::Order {
    fn from(i: models::Order) -> Self {
        let mut o = _name_::Order::new();
        o.set_id(i.id.into());
        o.set_account_id(i.account_id.into());
        o.set_placed_at(i.placed_at.to_string());
        o.set_initial_process_at(i.initial_process_at.to_string());
        o.set_asset_from(i.asset_from.into());
        o.set_asset_to(i.asset_to.into());
        o.set_side(i.side.into());
        o.set_type_(i.type_.into());
        o.set_amount(i.amount.to_string());
        o.set_filled(i.filled.to_string());
        o.set_status(i.status.into());
        o.set_closed_at(i.closed_at.to_string());
        o.set_commission_currency(i.commission_currency.into());
        o.set_commission_fixed(i.commission_fixed.into());
        o.set_commission_bp(i.commission_bp.into());
        o.set_tracking_code(i.tracking_code.to_string());
        o
    }
}
```