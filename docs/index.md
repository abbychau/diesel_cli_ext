# Diesel CLI Extension

Diesel CLI Extension is a tool-belt that aids Diesel CLI after it built schema.rs .

[![Build Status](https://travis-ci.org/abbychau/diesel_cli_ext.svg)](https://travis-ci.org/abbychau/diesel_cli_ext)
[![Crates.io](https://img.shields.io/crates/v/diesel_cli_ext.svg)](https://crates.io/crates/diesel_cli_ext)
<!-- [![Coverage Status](https://coveralls.io/repos/github/abbychau/diesel_cli_ext/badge.svg?branch=master)](https://coveralls.io/github/abbychau/diesel_cli_ext?branch=master) -->

It contains 4 functions at this moment.
1. Generate protobuf file.(`diesel_ext proto`)
2. Generate model rust structs.(`diesel_ext model`)
3. Generate conversion implementations.(`diesel_ext into_proto`, and `diesel_ext from_proto`)

## Installation
`cargo install diesel_cli_ext`

## How to use
First of all, `diesel print-schema > src/schema.rs` 

TL;DR: 

```
Usage: target/debug/diesel_ext FILE [options]

Common Options:
    -s, --schema-file PATH
                        Set schema file path
    -h, --help          Print this help menu

Model Options:
    -m, --model         Set as model output
    -M, --map "FROM_TYPE TO_TYPE"
                        Set type mappings (can be set multiple times) e.g.
                        --map "BigInt iccc"
    -I, --import-types "TYPE"
                        This field adds use statements to the top of every
                        table! declaration. (can be set multiple times) e.g.
                        --import_types "diesel::sql_types::*"
    -d, --derive DERIVES
                        set struct derives
    -t, --add-table-name 
                        Add #[table_name = x] before structs

Proto Options:
    -p, --proto         Set as proto output
    -i, --into_proto    Set as into_proto output
    -f, --from_proto    Set as from_proto output
    -c, --class_name CLASS_NAME
                        Set proto class name
```

(You can see it again by `diesel_ext --help`)

Output demostrations as below...


### To generate model structs:
e.g. `diesel_ext > src/db/db_models.rs` , `diesel_ext -m > src/models.rs`, `diesel_ext --model > src/models.rs` (it is the default option)

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
`diesel_ext -p > myproto.proto`, `diesel_ext --proto > myproto.proto`

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
`diesel_ext -f -c class_name > proto/src/conversion/from_proto.rs`, `diesel_ext -i -c class_name > proto/src/conversion/into_proto.rs`

(if you omit the second parameter, names will be displayed as `_name_` for your search and replace.)

Sample output(from):
``` rust
use models;
use proto::client_service;
use std::str::FromStr;
use std::convert::From;

impl From<class_name::CarryOverBalance> for models::CarryOverBalance {
    fn from(i: class_name::CarryOverBalance) -> Self {
        models::CarryOverBalance{
            account_id: i.get_account_id(),
            debit: i.get_debit().to_string(),
            description: i.get_description().to_string(),
        }
    }
}

impl From<class_name::Order> for models::Order {
    fn from(i: class_name::Order) -> Self {
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

impl From<models::CarryOverBalance> for class_name::CarryOverBalance {
    fn from(i: models::CarryOverBalance) -> Self {
        let mut o = class_name::CarryOverBalance::new();
        o.set_account_id(i.account_id.into());
        o.set_debit(i.debit.to_string());
        o.set_description(i.description.to_string());
        o
    }
}

impl From<models::Order> for class_name::Order {
    fn from(i: models::Order) -> Self {
        let mut o = class_name::Order::new();
        o.set_id1(i.id1.into());
        o.set_time(i.time.to_string());
        o.set_json(i.json.to_string());
        o
    }
}
```
