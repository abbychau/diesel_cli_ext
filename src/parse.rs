use std::collections::HashMap;
pub fn parse(
    contents: String,
    action: &str,
) -> (String, String, String, String, String, String, bool, bool) {
    //Parse
    let mut str_model: String = "".to_string();
    let mut str_proto: String = "".to_string();
    let mut str_from_proto: String = "".to_string();
    let mut str_into_proto: String = "".to_string();

    let mut str_rpc: String = "".to_string();
    let mut str_request: String = "".to_string();
    let mut closable: bool = false;
    let mut type_ndt: bool = false;
    let mut type_bd: bool = false;
    let mut count: u16 = 0;
    let mut struct_name: String = "".to_string();
    let lines = contents.split("\n");
    let model_type_dict: HashMap<&str, &str> = [
        ("Int2", "i16"),
        ("Int4", "i32"),
        ("Int8", "i64"),
        ("BigInt", "i64"),
        ("Numeric", "BigDecimal"),
        ("Text", "String"),
        ("Timestamp", "NaiveDateTime"),
        ("Float4", "f32"),
        ("Bool", "bool"),
        ("Json", "Json"),
        ("Jsonb", "Jsonb"),
        ("Varchar", "String"),
    ].iter()
        .cloned()
        .collect();
    let proto_type_dict: HashMap<&str, &str> = [
        ("Int2", "int32"),
        ("Int4", "int32"),
        ("Int8", "int64"),
        ("BigInt", "int64"),
        ("Numeric", "string"),
        ("Text", "string"),
        ("Timestamp", "string"),
        ("Float4", "float"),
        ("Bool", "bool"),
        ("Json", "string"),
        ("Jsonb", "string"),
        ("Varchar", "string"),
    ].iter()
        .cloned()
        .collect();
    for line in lines {
        let cmp = line.to_string();
        if cmp.contains("#[") {
            //do nothing
        } else if cmp.contains("table!") {
            str_model.push_str(&format!("\n#[derive(Queryable)]\n"));
        } else if cmp.contains(") {") {
            let vec: Vec<&str> = line.split(" ").collect();
            struct_name = propercase(vec[4]);
            str_model.push_str(&format!("pub struct {} {{\n", struct_name));
            str_proto.push_str(&format!("message {} {{\n", struct_name));

            str_into_proto.push_str(&format!(
                "\nimpl From<models::{}> for _name_::{} {{\n",
                struct_name, struct_name
            ));
            str_from_proto.push_str(&format!(
                "\nimpl From<_name_::{}> for models::{} {{\n",
                struct_name, struct_name
            ));

            str_into_proto.push_str(&format!(
                "    fn from(i: models::{}) -> Self {{\n",
                struct_name
            ));
            str_from_proto.push_str(&format!(
                "    fn from(i: _name_::{}) -> Self {{\n",
                struct_name
            ));

            str_from_proto.push_str(&format!("        models::{}{{\n", struct_name));
            str_into_proto.push_str(&format!(
                "        let mut o = _name_::{}::new();\n",
                struct_name
            ));
        } else if cmp.contains("->") {
            let vec: Vec<&str> = line.split(" ").collect();
            let _type = vec[10].replace(",", "");
            let dict = match action {
                "model" => &model_type_dict,
                _ => &proto_type_dict,
            };
            let mut type_string = match dict.get(_type.replace("Nullable<","").replace(">","").trim()){
                Some(name)=>name,
                None=> panic!("{} is not recognized. Please free feel to expand the HashMap. This could provide good hints: https://kotiri.com/2018/01/31/postgresql-diesel-rust-types.html", _type)
            };
            if type_string == &"NaiveDateTime" {
                type_ndt = true;
            }
            if type_string == &"BigDecimal" {
                type_bd = true;
            }

            str_model.push_str(&format!(
                "    pub {}: {},\n",
                &vec[8],
                format!("{}", type_string)
            ));
            count += 1;
            if count == 1 {
                let request_name = &format!("Enquire{}Request", &struct_name);
                str_rpc.push_str(&format!(
                    "    rpc get{} ({}) returns ({}) {{ }}\n",
                    &struct_name, &request_name, &struct_name
                ));
                str_request.push_str(&format!(
                    "message {} {{\n    int64 id =1;\n}}\n",
                    &request_name
                ));
            }
            str_proto.push_str(&format!("    {} {} = {};\n", type_string, &vec[8], count));
            str_from_proto.push_str(&format!(
                "            {}: i.get_{}(){},\n",
                &vec[8],
                &vec[8],
                match type_string {
                    &"string" => ".to_string()",
                    &"String" => ".to_string()",
                    &"BigDecimal" => ".to_bigdecimal()",
                    _ => "",
                }
            ));
            str_into_proto.push_str(&format!(
                "        o.set_{}(i.{}{});\n",
                &vec[8],
                &vec[8],
                match type_string {
                    &"string" => ".to_string()",
                    &"String" => ".to_string()",
                    _ => ".into()",
                }
            ));
            //str_into_proto
            closable = true;
        } else if cmp.contains("}") {
            if closable {
                count = 0;
                str_model.push_str(&format!("}}\n"));
                str_proto.push_str(&format!("}}\n"));

                str_from_proto.push_str(&format!("        }}\n"));
                str_from_proto.push_str(&format!("    }}\n"));

                str_into_proto.push_str(&format!("        o\n    }}\n"));

                str_from_proto.push_str(&format!("}}\n"));
                str_into_proto.push_str(&format!("}}\n"));
                closable = false;
            }
        }
    }
    (
        str_proto,
        str_request,
        str_rpc,
        str_model,
        str_from_proto,
        str_into_proto,
        type_ndt,
        type_bd,
    )
}

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
        } else {
            store.push(c);
        }
        last_char = c;
    }
    if last_char == 's' {
        store.pop();
    }
    store.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::prelude::*;

    fn get_contents() -> String {
        let mut f = ::std::fs::File::open("test_data/schema.rs")
            .expect("File not found. Please run in the directory with schema.rs.");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("Something went wrong reading the file.");
        contents
    }

    #[test]
    fn build_all() {
        let (
            str_proto,
            str_request,
            str_rpc,
            str_model,
            str_from_proto,
            str_into_proto,
            type_ndt,
            type_bd,
        ) = super::parse(get_contents(), "model");
        assert_eq!(str_proto.chars().count(), 191);
        assert_eq!(str_into_proto.chars().count(), 582);
        assert_eq!(str_from_proto.chars().count(), 556);
        assert_eq!(str_request.chars().count(), 109);
        assert_eq!(str_rpc.chars().count(), 151);
        assert_eq!(type_ndt, true);
        assert_eq!(type_bd, true);
    }

}
