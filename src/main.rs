use getopts::Options;
use parse::ParseArguments;
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
            "{}\n\nCommon Options:\n{}\n\nModel Options:\n{}\n\nInsertable Options:\n{}\n\nProto Options:\n{}\n",
            brief,
            full_param[0..2].join("\n"),
            full_param[2..7].join("\n"),
            full_param[7..14].join("\n"),
            full_param[14..19].join("\n"),
        )
    })
}

fn print_normal_dependencies(parse_output: &parse::ParseOutput) {
    if parse_output.type_nd {
        println!("use chrono::NaiveDate;");
    }
    if parse_output.type_ndt {
        println!("use chrono::NaiveDateTime;");
    }
    if parse_output.type_nt {
        println!("use chrono::NaiveTime;");
    }
    if parse_output.type_bd {
        println!("use bigdecimal::BigDecimal;");
    }
    if parse_output.type_ip {
        println!("use ipnetwork::IpNetwork;");
    }
    if parse_output.type_uuid {
        println!("use uuid::Uuid;");
    }
    if parse_output.type_tz {
        println!("use chrono::DateTime;");
        println!("use chrono::offset::Utc;");
    }
    if parse_output.type_jsonb {
        println!("use serde_json::Value;");
    }
    if parse_output.diesel_macro_use {
        println!("use diesel::prelude::*;");
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
fn print_conversion_methods(parse_output: &parse::ParseOutput) {
    //todo add selection for ndt and bd
    if parse_output.type_ndt {
        println!(
            "
fn str2ndt(istr: &str) -> NaiveDateTime {{
    NaiveDateTime::parse_from_str(istr, \"%Y-%m-%d %H:%M:%S\").unwrap()
}}"
        );
    }

    if parse_output.type_bd {
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
    let mut model_derives: Option<String> = None;
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
    opts.optmulti(
        "I",
        "import-types",
        "This field adds use statements to the top of every table! declaration. (can be set multiple times) e.g. --import_types \"diesel::sql_types::*\"",
        "\"TYPE\"",
    );
    opts.optmulti(
        "",
        "derive-mod",
        "(NOT ready)This field adds derives for certain tables. (can be set multiple times) e.g. --derive-mod \"table_name +Debug\" --derive-mod \"table_name2 -Debug\"",
        "\"TABLENAME MODIFIER\"",
    );
    opts.optmulti(
        "n",
        "struct-name-override",
        "This field overrides the generated struct name for certain tables. (can be set multiple times) e.g. --struct-name-override \"foo bar\" --struct-name-override \"bar baz\"",
        "\"STRUCT NAME OVERRIDE\"",
    );
    opts.optopt("d", "derive", "set struct derives", "DERIVES");
    opts.optflag(
        "t",
        "add-table-name",
        "Add #[table_name = x] before structs",
    );
    opts.optflag(
        "r",
        "rust_styled_model_fields",
        "When creating models fields, will use rust styled names instead of database styled names",
    );
    
    // Insertable struct options
    opts.optflag(
        "g",
        "insertable",
        "Generate insertable structs for database inserts (omits auto-increment/default fields)",
    );
    opts.optmulti(
        "S",
        "skip-fields",
        "Fields to skip in insertable structs (can be set multiple times) e.g. --skip-fields \"id\" --skip-fields \"created_at\"",
        "\"FIELD_NAME\"",
    );
    opts.optmulti(
        "O",
        "optional-fields",
        "Fields to make optional in insertable structs even if NOT NULL (can be set multiple times) e.g. --optional-fields \"status\"",
        "\"FIELD_NAME\"",
    );
    opts.optopt(
        "P",
        "insertable-prefix",
        "Prefix for insertable struct names (default: New)",
        "PREFIX",
    );

    opts.optflag("p", "proto", "Set as proto output");
    opts.optflag("i", "into_proto", "Set as into_proto output");
    opts.optflag("f", "from_proto", "Set as from_proto output");
    opts.optopt("c", "class_name", "Set proto class name", "CLASS_NAME");
    opts.optopt(
        "v",
        "diesel_version",
        "Set diesel version (default:2)",
        "1 or 2",
    );

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!("{}", f),
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    //print!("{:?}",matches.opt_defined("m"));

    let rust_styled_fields = matches.opt_present("r");

    let mut model_type_mapping: HashMap<String, String> = HashMap::new();
    if matches.opt_present("M") {
        for x in matches.opt_strs("M") {
            let k: Vec<&str> = x.trim().split(' ').collect();
            model_type_mapping.insert(k[0].to_string(), k[1].to_string());
        }
    }

    let mut struct_name_override: HashMap<String, String> = HashMap::new();
    if matches.opt_present("n") {
        for x in matches.opt_strs("n") {
            let k: Vec<&str> = x.trim().split(' ').collect();
            struct_name_override.insert(k[0].to_string(), k[1].to_string());
        }
    }

    let diesel_version = matches.opt_str("v").unwrap_or("2".to_string());
    if diesel_version != "1" && diesel_version != "2" {
        panic!("diesel_version must be 1 or 2");
    }
    // Parse insertable struct options
    let skip_fields: Vec<String> = if matches.opt_present("S") {
        matches.opt_strs("S")
    } else {
        vec!["id".to_string(), "created_at".to_string(), "updated_at".to_string()]
    };
    
    let optional_fields: Vec<String> = if matches.opt_present("O") {
        matches.opt_strs("O")
    } else {
        vec![]
    };
    
    let insertable_prefix = matches.opt_str("P").unwrap_or("New".to_string());
    
    let action = if matches.opt_present("m") {
        model_derives = matches.opt_str("d");
        "model"
    } else if matches.opt_present("g") {
        model_derives = matches.opt_str("d");
        "insertable"
    } else if matches.opt_present("i") {
        class_name = matches
            .opt_str("c")
            .unwrap_or_else(|| "class_name".to_string());
        "into_proto"
    } else if matches.opt_present("f") {
        class_name = matches
            .opt_str("c")
            .unwrap_or_else(|| "class_name".to_string());
        "from_proto"
    } else if matches.opt_present("p") {
        "proto"
    } else {
        //default as m
        model_derives = matches.opt_str("d");
        "model"
    };

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
                let file_path = values["print_schema"]["file"].to_string().replace('\"', "");
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

    let parse_output = parse::parse(ParseArguments {
        contents,
        action: action.into(),
        model_derives,
        add_table_name: matches.opt_present("t"),
        model_type_mapping,
        diesel_version,
        rust_styled_fields,
        struct_name_override,
        skip_fields,
        optional_fields,
        insertable_prefix,
    });

    //imported types
    let mut import_type_string = String::new();
    if matches.opt_present("I") {
        for x in matches.opt_strs("I") {
            import_type_string.push_str("use ");
            import_type_string.push_str(&x);
            import_type_string.push_str(";\n");
        }
    }
    match action {
        "proto" => {
            println!("syntax = \"proto3\";\n\n");
            println!("{}\n", parse_output.str_proto);
            println!("{}\n", parse_output.str_request);
            println!("service MessageRpc {{\n{}}}", parse_output.str_rpc);
        }
        "model" => {
            println!("// Generated by diesel_ext\n");

            println!("#![allow(unused)]");
            println!("#![allow(clippy::all)]\n");

            println!("{}", import_type_string);
            print_normal_dependencies(&parse_output);
            println!("{}", parse_output.str_model);
        }
        "insertable" => {
            println!("// Generated by diesel_ext (insertable structs)\n");

            println!("#![allow(unused)]");
            println!("#![allow(clippy::all)]\n");

            println!("{}", import_type_string);
            print_normal_dependencies(&parse_output);
            if parse_output.diesel_macro_use {
                println!("use diesel::prelude::*;");
            }
            println!("{}", parse_output.str_insertable);
        }
        "from_proto" => {
            print_conversion_dependencies();
            print_conversion_methods(&parse_output);
            println!(
                "{}",
                parse_output.str_from_proto.replace("_name_", &class_name)
            );
        }
        "into_proto" => {
            print_conversion_dependencies();
            println!(
                "{}",
                parse_output.str_into_proto.replace("_name_", &class_name)
            );
        }
        _ => {
            print_usage(&program, opts);
        }
    }
}