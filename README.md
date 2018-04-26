# Diesel CLI Extension

Diesel CLI Extension is a tool-belt that aids Diesel CLI after it built schema.rs .

It contains 4 functions at this moment.
1. Generate protobuf file.(`diesel_ext proto`)
2. Generate model rust structs.(`diesel_ext model`)
3. Generate conversion implementations.(`diesel_ext into_proto`, and `diesel_ext from_proto`)

## How to use

First of all, `diesel print-schema > src/schema.rs` 

### To generate model structs:
e.g. `diesel_ext > src/db/db_models.rs` , `diesel_ext model > src/models.rs` (it is the default option)

Sample model output:
``` rust
use chrono::NaiveDateTime;
use bigdecimal::BigDecimal;

#[derive(Queryable)]
pub struct CarryOverBalance {
    pub account_id : i64,
    pub debit : BigDecimal,
    pub description : String,
}

#[derive(Queryable)]
pub struct Order {
    pub id1 : i64,
    pub time : NaiveDateTime,
    pub json : String,
}
```

### To generate prelimitive proto file:
`diesel_ext proto > myproto.proto`

Sample output:
``` r
syntax = "proto3";


message CarryOverBalance {
    int64 account_id = 1;
    string debit = 2;
    string description = 3;
}
message Order {
    int64 id1 = 1;
    string time = 2;
    string json = 3;
}


message EnquireCarryOverBalanceRequest {
    int64 id =1;
}
message EnquireOrderRequest {
    int64 id =1;
}


service MessageRpc {
    rpc getCarryOverBalance (EnquireCarryOverBalanceRequest) returns (CarryOverBalance) { }
    rpc getOrder (EnquireOrderRequest) returns (Order) { }
}
```

### To generate proto conversions:
`diesel_ext from_proto > proto/src/conversion/from_proto.rs`, `diesel_ext into_proto > proto/src/conversion/into_proto.rs`

Sample output(from):
``` rust
use models;
use proto::client_service;
use std::str::FromStr;
use std::convert::From;

impl From<_name_::CarryOverBalance> for models::CarryOverBalance {
    fn from(i: _name_::CarryOverBalance) -> Self {
        models::CarryOverBalance{
            account_id: i.get_account_id(),
            debit: i.get_debit().to_string(),
            description: i.get_description().to_string(),
        }
    }
}

impl From<_name_::Order> for models::Order {
    fn from(i: _name_::Order) -> Self {
        models::Order{
            id1: i.get_id1(),
            time: i.get_time().to_string(),
            json: i.get_json().to_string(),
        }
    }
}
```

into:
``` rust
use models;
use proto::client_service;
use std::str::FromStr;
use std::convert::From;

impl From<models::CarryOverBalance> for _name_::CarryOverBalance {
    fn from(i: models::CarryOverBalance) -> Self {
        let mut o = _name_::CarryOverBalance::new();
        o.set_account_id(i.account_id.into());
        o.set_debit(i.debit.to_string());
        o.set_description(i.description.to_string());
        o
    }
}

impl From<models::Order> for _name_::Order {
    fn from(i: models::Order) -> Self {
        let mut o = _name_::Order::new();
        o.set_id1(i.id1.into());
        o.set_time(i.time.to_string());
        o.set_json(i.json.to_string());
        o
    }
}
```