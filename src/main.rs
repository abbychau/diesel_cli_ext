use getopts::Options;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use toml::Value;

mod parse;

/// Derive a formatted message from a set of options.
pub fn custom_opts_usage(iopts: Options, brief: &str) -> String {
    iopts.usage_with_format(|opts| {
        let full_param = opts.collect::<Vec<String>>();
        format!(
            "{}\n\nCommon Options:\n{}\n\nModel Options:\n{}\n\nProto Options:\n{}\n",
            brief,
            full_param[0..2].join("\n"),
            full_param[2..6].join("\n"),
            full_param[6..9].join("\n"),
        )
    })
}

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
    print!("{}", custom_opts_usage(opts, &brief));
}

fn main() {
    //Read in
    let args: Vec<String> = env::args().collect();
    let action;
    let mut derive: Option<String> = None;
    let mut class_name: String = "".to_string();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optopt("s", "schema-file", "Set schema file path", "PATH");
    opts.optflag("h", "help", "Print this help menu");

    opts.optflag("m", "model", "Set as model output");
    opts.optmulti(
        "M",
        "map",
        "Set type mappings (can be set multiple times) e.g. --map \"BigInt iccc\"",
        "\"FROM_TYPE TO_TYPE\"",
    );
    opts.optopt("d", "derive", "set struct derives", "DERIVES");
    opts.optflag(
        "t",
        "add_table_name",
        "Add #[table_name = x] before structs",
    );

    opts.optflag("i", "into_proto", "Set as into_proto output");
    opts.optflag("f", "from_proto", "Set as from_proto output");
    opts.optopt("c", "class_name", "Set proto class name", "CLASS_NAME");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    //print!("{:?}",matches.opt_defined("m"));

    let mut type_mapping: HashMap<String, String> = HashMap::new();

    if matches.opt_present("M") {
        for x in matches.opt_strs("M") {
            let k: Vec<&str> = x.trim().split(' ').collect();
            type_mapping.insert(k[0].to_string(), k[1].to_string());
        }
    }

    if matches.opt_present("m") {
        action = "model";
        derive = matches.opt_str("d");
    } else if matches.opt_present("i") {
        action = "into_proto";
        class_name = matches
            .opt_str("c")
            .unwrap_or_else(|| "class_name".to_string());
    } else if matches.opt_present("f") {
        action = "from_proto";
        class_name = matches
            .opt_str("c")
            .unwrap_or_else(|| "class_name".to_string());
    } else {
        //default as m
        action = "model";
        derive = matches.opt_str("d");
    }

    let path = match matches.opt_str("s") {
        Some(file2) => file2,
        None => {
            if Path::new("diesel.toml").exists() {
                // println!("Found diesel.toml, using the file value inside.");
                let mut toml_f = File::open("diesel.toml").unwrap();
                let mut contents = String::new();
                toml_f
                    .read_to_string(&mut contents)
                    .expect("diesel.toml exists but not readable");
                let values = contents.parse::<Value>().unwrap();
                let file_path = values["print_schema"]["file"].to_string().replace("\"", "");
                if !Path::new(&file_path).exists() {
                    print!(
                        "Found diesel.toml and read a path: {}. However, this file does not exist.",
                        file_path
                    );
                    std::process::exit(1);
                }
                values["print_schema"]["file"].as_str().unwrap().to_string()
            } else {
                "schema.rs".to_string()
            }
        }
    };

    let mut f = File::open(path).expect("Schema file not found. Please check the specified file path or run in the directory with schema.rs / diesel.toml.");

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
    ) = parse::parse(
        contents,
        action,
        derive,
        matches.opt_present("t"),
        &mut type_mapping,
    );
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
