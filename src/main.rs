use std::env;
use std::fs::File;
use std::io::prelude::*;
mod parse;

fn main() {
    //Read in
    let args: Vec<_> = env::args().collect();
    let action;
    if args.len() < 2 {
        action = "model";
    } else {
        action = &args[1];
    }

    let mut f = File::open("src/schema.rs")
        .expect("File not found. Please run in the directory with schema.rs.");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Something went wrong reading the file.");

    let (str_proto,str_request,str_rpc,str_model,str_from_proto,str_into_proto,type_ndt,type_bd) = parse::parse(contents,action);
    //Output
    match action {
        "proto" => {
            println!("syntax = \"proto3\";\n\n");
            //println!("package your_package_name;\n\n");
            println!("{}\n", str_proto);
            println!("{}\n", str_request);
            println!("service MessageRpc {{\n{}}}", str_rpc);
        },
        "model" => {
            if type_ndt {
                println!("use chrono::NaiveDateTime;");
            }
            if type_bd {
                println!("use bigdecimal::BigDecimal;");
            }
            println!("{}", str_model);
        },
        "from_proto"=> {
            println!("use bigdecimal::BigDecimal;
use chrono::{{DateTime, NaiveDateTime, TimeZone, Utc}};
use models;
use proto::_name_;
use std::str::FromStr;
use std::convert::From;

fn str2Ndt(str: &str) -> NaiveDateTime {{
    NaiveDateTime::parse_from_str(str, \"%Y-%m-%d %H:%M:%S\").unwrap()
}}

");
            println!("{}", str_from_proto);
        },
        "into_proto"=> {
            println!("use bigdecimal::BigDecimal;
use models;
use proto::_name_;
use protobuf::RepeatedField;
use std::collections::HashMap;
use std::convert::From;

");
            println!("{}", str_into_proto);
        },
        _=>{}
    }
}
