use getopts::Options;
use std::env;
use std::fs::File;
use std::io::prelude::*;

mod parse;

fn print_normal_dependencies(type_ndt: bool, type_bd: bool, type_ip: bool) {
    if type_ndt {
        println!("use chrono::NaiveDateTime;");
    }
    if type_bd {
        println!("use bigdecimal::BigDecimal;");
    }
    if type_ip {
        println!("use ipnetwork::IpNetwork;");
    }
}
fn print_conversion_dependencies() {
    //todo add selection for ndt and bd
    println!(
        "
use models;
use proto::client_service;
use std::str::FromStr;
use std::convert::From;"
    );
}
fn print_conversion_methods(type_ndt: bool, type_bd: bool) {
    //todo add selection for ndt and bd
    if type_ndt {
        println!(
            "
fn str2ndt(istr: &str) -> NaiveDateTime {{
    NaiveDateTime::parse_from_str(istr, \"%Y-%m-%d %H:%M:%S\").unwrap()
}}"
        );
    }

    if type_bd {
        println!(
            "
fn str2bd(istr: &str) -> BigDecimal{{
    BigDecimal::from_str(istr).unwrap()
}}"
        );
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    //Read in
    let args: Vec<String> = env::args().collect();
    let action;
    let mut derive: Option<String> = None;
    let mut class_name: String = "".to_string();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optopt("s", "schema-file", "set file path", "PATH");
    opts.optflag("h", "help", "Print this help menu");
    opts.optflag("m", "model", "model output");
    opts.optflag("i", "into_proto", "into_proto output");
    opts.optflag("f", "from_proto", "from_proto output");
    opts.optflag("c", "class_name", "proto class name");
    opts.optflag("d", "derive", "set struct derives");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    //print!("{:?}",matches.opt_defined("m"));
    
    if matches.opt_present("m") {
        action = "model";
        derive = matches.opt_str("d");
    } else if matches.opt_present("i") {
        action = "into_proto";
        class_name = matches.opt_str("c").unwrap_or("class_name".to_string());
    } else if matches.opt_present("f") {
        action = "from_proto";
        class_name = matches.opt_str("c").unwrap_or("class_name".to_string());
    } else {
        //default as m
        action = "model";
        derive = matches.opt_str("d");
    }
    
    let mut f = File::open(match matches.opt_str("s"){
        Some(file2)=>file2,
        None=>"schema.rs".to_string()
    }).expect("File not found. Please check the specified file path or run in the directory with schema.rs.");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Something went wrong reading the file.");

    let (
        str_proto,
        str_request,
        str_rpc,
        str_model,
        str_from_proto,
        str_into_proto,
        type_ndt,
        type_bd,
        type_ip,
    ) = parse::parse(contents, action, derive);
    //Output
    
    match action {
        "proto" => {
            println!("syntax = \"proto3\";\n\n");
            println!("{}\n", str_proto);
            println!("{}\n", str_request);
            println!("service MessageRpc {{\n{}}}", str_rpc);
        }
        "model" => {
            println!("// Generated by diesel_ext\n");
            println!("#![allow(unused)]");
            println!("#![allow(clippy::all)]\n");
            print_normal_dependencies(type_ndt, type_bd, type_ip);
            println!("{}", str_model);
        }
        "from_proto" => {
            print_conversion_dependencies();
            print_conversion_methods(type_ndt, type_bd);
            println!("{}", str_from_proto.replace("_name_", &class_name));
        }
        "into_proto" => {
            print_conversion_dependencies();
            println!("{}", str_into_proto.replace("_name_", &class_name));
        }
        _ => {
            print_usage(&program, opts);
        }
    }
}

