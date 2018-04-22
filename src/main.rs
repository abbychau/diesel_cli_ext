use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;


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
    let mut f = File::open("src/schema.rs").expect("File not found. Please run in the directory with schema.rs.");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Something went wrong reading the file.");

    //Parse
    let mut output: String = "".to_string();
    let mut closable : bool = false;
    let mut type_ndt : bool = false;
    let mut type_bd : bool = false;

    let lines = contents.split("\n");
    let type_dict:HashMap<&str,&str> = [
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
    
    for line in lines {
        let cmp = line.to_string();
        if cmp.contains("#[") {
            //do nothing
        }else if cmp.contains("table!") {
            output.push_str(&format!("\n#[derive(Queryable)]\n"));

        }else if cmp.contains(") {") {
            let vec: Vec<&str> = line.split(" ").collect();
            output.push_str(
                &format!("pub struct {} {{\n", propercase(vec[4]) )
            );

        }else if cmp.contains("->") {
            let vec: Vec<&str> = line.split(" ").collect();
            let _type = vec[10].replace(",","");
            let mut type_string = match type_dict.get(_type.replace("Nullable<","").replace(">","").trim()){
                Some(name)=>name,
                None=> panic!("{} is not recognized. Please free feel to expand the HashMap. This could provide good hints: https://kotiri.com/2018/01/31/postgresql-diesel-rust-types.html", _type)
            };
            if type_string == &"NaiveDateTime" {type_ndt = true;}
            if type_string == &"BigDecimal" {type_bd = true;}

            output.push_str(&format!("    pub {} : {}", 
                &vec[8],
                match cmp.contains("Nullable<") {
                    true => format!("Option<{}>",type_string),
                    false => format!("{}",type_string)
                }
            ));
            output.push_str(",\n");
            closable=true;

        }else if cmp.contains("}") {
            if closable {
                output.push_str(&format!("}}\n"));
                closable = false;
            }
        }
    }

    //Output
    if type_ndt {
        println!("use chrono::NaiveDateTime;");
    }
    if type_bd {
        println!("use bigdecimal::BigDecimal;");
    }
    println!("{}",output);
}