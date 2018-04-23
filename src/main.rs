use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::env;

fn propercase(s: &str) -> String {
    let mut next_cap = true;
    let mut store: Vec<char> = Vec::new();
    let mut last_char: char = ' ';
    for c in s.chars() {
        if c == '_' {
            next_cap = true;
            continue;
        }
        if next_cap {
            store.push(c.to_ascii_uppercase());
            next_cap = false;
        }else{
            store.push(c);
        }
        last_char = c;
    }
    if last_char == 's' {store.pop();}
    store.into_iter().collect()
}

fn main() {
    //Read in
    let args: Vec<_> = env::args().collect();
    let action;
    if args.len() < 2 {
        action = "model";
    }else{
        action = &args[1];
    }
    
    let mut f = File::open("src/schema.rs").expect("File not found. Please run in the directory with schema.rs.");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Something went wrong reading the file.");

    //Parse
    let mut str_model : String = "".to_string();
    let mut str_proto : String = "".to_string();
    let mut str_rpc : String = "".to_string();
    let mut str_request : String = "".to_string();
    let mut closable : bool = false;
    let mut type_ndt : bool = false;
    let mut type_bd : bool = false;
    let mut count : u16 = 0;
    let mut struct_name : String = "".to_string();
    let lines = contents.split("\n");
    let model_type_dict:HashMap<&str,&str> = [
        ("Int2", "i16"),
        ("Int4", "i32"),
        ("Int8", "i64"),
        ("BigInt", "i64"),
        ("Numeric", "BigDecimal"),
        ("Text", "String"),
        ("Timestamp","NaiveDateTime"),
        ("Float4","f32"),
        ("Bool", "bool"),
        ("Varchar", "String")
    ].iter().cloned().collect();
    let proto_type_dict:HashMap<&str,&str> = [
        ("Int2", "int32"),
        ("Int4", "int32"),
        ("Int8", "int64"),
        ("BigInt", "int64"),
        ("Numeric", "string"),
        ("Text", "string"),
        ("Timestamp","string"),
        ("Float4","float"),
        ("Bool", "bool"),
        ("Varchar", "string")
    ].iter().cloned().collect();    
    for line in lines {
        let cmp = line.to_string();
        if cmp.contains("#[") {
            //do nothing
        }else if cmp.contains("table!") {
            str_model.push_str(&format!("\n#[derive(Queryable)]\n"));
            //str_proto.push_str(&format!("\n#[derive(Queryable)]\n"));
        }else if cmp.contains(") {") {
            let vec: Vec<&str> = line.split(" ").collect();
            struct_name = propercase(vec[4]);
            str_model.push_str(
                &format!("pub struct {} {{\n", struct_name )
            );
            str_proto.push_str(
                &format!("message {} {{\n", struct_name )
            );
            
            
        }else if cmp.contains("->") {
            let vec: Vec<&str> = line.split(" ").collect();
            let _type = vec[10].replace(",","");
            let dict = match _action.as_str() {
                "model" => &model_type_dict,
                _ => &proto_type_dict
            };
            let mut type_string = match dict.get(_type.replace("Nullable<","").replace(">","").trim()){
                Some(name)=>name,
                None=> panic!("{} is not recognized. Please free feel to expand the HashMap. This could provide good hints: https://kotiri.com/2018/01/31/postgresql-diesel-rust-types.html", _type)
            };
            if type_string == &"NaiveDateTime" {type_ndt = true;}
            if type_string == &"BigDecimal" {type_bd = true;}

            str_model.push_str(&format!("    pub {} : {},\n", 
                &vec[8],
                false => format!("{}",type_string)
            ));
            count += 1;
            if count == 1 {
                let request_name = &format!("Enquire{}Request",&struct_name);
                str_rpc.push_str(&format!("    rpc get{} ({}) returns ({}) {{ }}\n",&struct_name,&request_name,&struct_name));
                str_request.push_str(&format!("message {} {{\n    int64 id =1;\n}}\n",&request_name));
            }
            str_proto.push_str(&format!("    {} {} = {};\n", 
                
                type_string,
                &vec[8],
                count
            ));
            closable=true;

        }else if cmp.contains("}") {
            if closable {
                count=0;
                str_model.push_str(&format!("}}\n"));
                str_proto.push_str(&format!("}}\n"));

                closable = false;
            }
        }
    }

    //Output
    if action == "proto"{
        println!("syntax = \"proto3\";\n\n");
        //println!("package your_package_name;\n\n");
        println!("{}\n",str_proto);
        println!("{}\n",str_request);
        println!("service MessageRpc {{\n{}}}",str_rpc);

    }else{

        if type_ndt {
            println!("use chrono::NaiveDateTime;");
        }
        if type_bd {
            println!("use bigdecimal::BigDecimal;");
        }
        println!("{}",str_model);
    }
    
}
