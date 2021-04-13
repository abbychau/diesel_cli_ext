use std::collections::HashMap;
use std::io::{stderr, Write};

pub struct ParseOutput {
    pub str_proto: String,
    pub str_request: String,
    pub str_rpc: String,
    pub str_model: String,
    pub str_from_proto: String,
    pub str_into_proto: String,
    pub type_nd: bool,
    pub type_ndt: bool,
    pub type_nt: bool,
    pub type_bd: bool,
    pub type_ip: bool,
    pub type_uuid: bool,
    pub type_tz: bool,
}

pub fn parse(
    contents: String,
    action: &str,
    model_derives: Option<String>,
    add_table_name: bool,
    model_type_mapping: &mut HashMap<String, String>,
) -> ParseOutput {
    //Parse
    let mut str_model: String = "".to_string();
    let mut str_proto: String = "".to_string();
    let mut str_from_proto: String = "".to_string();
    let mut str_into_proto: String = "".to_string();
    let mut str_rpc: String = "".to_string();
    let mut str_request: String = "".to_string();
    let mut closable: bool = false;
    let (
        mut type_nd,
        mut type_ndt,
        mut type_nt,
        mut type_bd,
        mut type_ip,
        mut type_uuid,
        mut type_tz,
    ) = (false, false, false, false, false, false, false);

    let mut count: u16 = 0;
    let mut struct_name: String = "".to_string();
    let content = contents.replace('\t', "    ");
    let lines = content.split('\n');
    let mut model_type_dict: HashMap<&str, &str> = [
        ("Int2", "i16"),
        ("SmallInt", "i16"), //sqlite
        ("Int4", "i32"),
        ("Integer", "i32"), //sqlite
        ("Unsigned<Integer", "u32"),
        ("Unsigned<Decimal", "f64"),
        ("Int8", "i64"),
        ("BigInt", "i64"),
        ("Numeric", "BigDecimal"),
        ("Decimal", "f64"),
        ("Text", "String"),
        ("Date", "NaiveDate"),
        ("Time", "NaiveTime"),
        ("Datetime", "NaiveDateTime"),
        ("Timestamp", "NaiveDateTime"),
        ("Timestamptz", "DateTime<Utc>"),
        ("Float4", "f32"),
        ("Float8", "f64"),
        ("Float", "f32"), //sqlite
        ("Bool", "bool"),
        ("Json", "Json"),
        ("Jsonb", "Jsonb"),
        ("Uuid", "Uuid"),
        ("Char", "String"),
        ("Varchar", "String"),
        ("Bytea", "Vec<u8>"),
        ("Binary", "Vec<u8>"),
        ("Varbinary", "Vec<u8>"),
        ("Blob", "Vec<u8>"),
        ("Tinyblob", "Vec<u8>"),
        ("Mediumblob", "Vec<u8>"),
        ("Longblob", "Vec<u8>"),
        ("Bit", "bool"),
        ("Inet", "IpNetwork"),
        ("Tinytext", "String"),
        ("Mediumtext", "String"),
        ("Longtext", "String"),
        ("Double", "f64"),
        ("Tinyint", "i8"),
        ("Unsigned<Tinyint", "u8"),
        ("Smallint", "i16"),
        ("Unsigned<Smallint", "u16"),
        ("Bigint", "i64"),
        ("Unsigned<Bigint", "u64"),
    ]
    .iter()
    .cloned()
    .collect();

    let proto_type_dict: HashMap<&str, &str> = [
        ("Int2", "int32"),
        ("Int4", "int32"),
        ("Int8", "int64"),
        ("BigInt", "int64"),
        ("Numeric", "string"),
        ("Text", "string"),
        ("Date", "NaiveDate"),
        ("Timestamp", "string"),
        ("Timestamptz", "string"),
        ("Float4", "float"),
        ("Float8", "double"),
        ("Bool", "bool"),
        ("Json", "string"),
        ("Jsonb", "string"),
        ("Varchar", "string"),
        ("Bytea", "bytes"),
        ("Inet", "string"),
        ("Uuid", "string"),
    ]
    .iter()
    .cloned()
    .collect();

    for (key, val) in model_type_mapping.iter() {
        model_type_dict.insert(&key, &val);
    }

    let mut is_schema = false;
    // let mut is_excluding = true;
    for line in lines {
        let cmp = line.to_string();
        let vec: Vec<&str> = line.split(' ').collect();
        let indent_depth = if is_schema { 4 } else { 0 };

        if cmp.contains("#[") || cmp.contains("joinable!(") {
            //do nothing
        } else if cmp.contains("pub mod ") {
            if is_schema {
                str_model.push_str("\n}\n\n");
            }
            str_model.push_str(&format!("pub mod {} {{\n", &vec[2]));
            is_schema = true;
        } else if cmp.contains("table!") {
            // add derives
            str_model.push_str(&format!(
                "\n{}#[derive({})]\n",
                " ".repeat(indent_depth),
                &format!(
                    "{}{{trace1}}",
                    model_derives.as_deref().unwrap_or("Queryable, Debug")
                )
            ));
        } else if cmp.contains(") {") {
            // this line contains table name
            struct_name = propercase(vec[4 + indent_depth]);
            if is_schema {
                struct_name = if struct_name.contains('.') {
                    let _v: Vec<&str> = struct_name.split('.').collect();
                    _v[1].to_string()
                } else {
                    struct_name
                }
            }
            let x: &[_] = &['(', ')', '{', '}', ',', ' '];
            let mut pks_list: Vec<String> = vec![];
            if vec.len() - 1 > 5 + indent_depth {
                for c in &vec[5 + indent_depth..vec.len() - 1] {
                    let pks = c.trim_matches(x);

                    pks_list.push(pks.to_string());
                }

                if pks_list.len() > 1 || pks_list[0] != "id" {
                    str_model = str_model.replace(
                        "{trace1}",
                        if model_derives.is_none()
                            || (model_derives.is_some()
                                && !model_derives.clone().unwrap().contains("Identifiable"))
                        {
                            ", Identifiable"
                        } else {
                            ""
                        },
                    );
                    str_model.push_str(&" ".repeat(indent_depth));
                    str_model.push_str("#[primary_key(");
                    str_model.push_str(&pks_list.join(", "));
                    str_model.push_str(")]\n");
                }
            }

            if add_table_name {
                // add #[table_name = "name"]
                str_model.push_str(&format!(
                    "{}#[table_name = \"{}\"]\n",
                    " ".repeat(indent_depth),
                    vec[4 + indent_depth]
                ));
            }

            str_model.push_str(&format!(
                "{}pub struct {} {{\n",
                " ".repeat(indent_depth),
                struct_name
            ));
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
            let _type = vec[10 + indent_depth].replace(",", "");

            let dict = match action {
                "model" => &model_type_dict,
                _ => &proto_type_dict,
            };
            let is_optional = _type.clone().contains("Nullable<");
            let vec_count = _type.clone().matches("Array").count();
            let b_position = _type.find('[').unwrap_or_else(|| _type.len());
            let mut single_type = _type.clone();
            single_type.truncate(b_position);
            let warning_for_longer_lifetime: String;
            let type_string: &str = match dict.get(
                single_type
                    .replace("Array<", "")
                    .replace("Nullable<", "")
                    .replace(">", "")
                    .trim(),
            ) {
                Some(name) => name,
                None => {
                    // Show a warning and return a placeholder.
                    stderr().write_all(&format!("{} is not recognized. Please feel free to expand the HashMap. This could provide \
                    good hints: https://kotiri.com/2018/01/31/postgresql-diesel-rust-types.html\n", _type).into_bytes()).unwrap();
                    warning_for_longer_lifetime = format!("/* TODO: unknown type {} */", _type);
                    &warning_for_longer_lifetime[..]
                }
            };
            if type_string == "NaiveDate" {
                type_nd = true;
            }
            if type_string == "NaiveDateTime" {
                type_ndt = true;
            }
            if type_string == "NaiveTime" {
                type_nt = true;
            }
            if type_string == "BigDecimal" {
                type_bd = true;
            }
            if type_string == "IpNetwork" {
                type_ip = true;
            }
            if type_string == "Uuid" {
                type_uuid = true;
            }
            if type_string == "DateTime<Utc>" {
                type_tz = true;
            }
            let type_with_vec_wrap = format!(
                "{}{}{}",
                "Vec<".repeat(vec_count),
                type_string,
                ">".repeat(vec_count)
            );
            str_model.push_str(&format!(
                "{}pub {}: {},\n",
                " ".repeat(indent_depth + 4),
                &vec[8 + indent_depth],
                if is_optional {
                    format!("Option<{}>", type_with_vec_wrap)
                } else {
                    type_with_vec_wrap
                }
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

            str_proto.push_str(&format!(
                "    {} {} = {};\n",
                type_string,
                &vec[8 + indent_depth],
                count
            ));
            str_from_proto.push_str(&format!(
                "            {}: i.get_{}(){},\n",
                &vec[8 + indent_depth],
                &vec[8 + indent_depth],
                match type_string {
                    "string" => ".to_string()",
                    "String" => ".to_string()",
                    "BigDecimal" => ".to_bigdecimal()",
                    _ => "",
                }
            ));
            str_into_proto.push_str(&format!(
                "        o.set_{}(i.{}{});\n",
                &vec[8 + indent_depth],
                &vec[8 + indent_depth],
                match type_string {
                    "string" => ".to_string()",
                    "String" => ".to_string()",
                    _ => ".into()",
                }
            ));
            //str_into_proto
            closable = true;
        } else if cmp.contains('}') && closable {
            count = 0;
            str_model.push_str(" ".repeat(indent_depth).as_str());
            str_model.push_str("}\n");
            str_proto.push_str("}\n");
            //" ".repeat(8)
            str_from_proto.push_str("        }\n");
            str_from_proto.push_str("    }\n");
            str_into_proto.push_str("        o\n    }\n");
            str_from_proto.push_str("}\n");
            str_into_proto.push_str("}\n");
            closable = false;
        }
    }

    if is_schema {
        str_model.push_str("\n}\n");
    }
    str_model = str_model.trim().replace("{trace1}", "");
    str_model.push('\n');
    ParseOutput {
        str_proto,
        str_request,
        str_rpc,
        str_model,
        str_from_proto,
        str_into_proto,
        type_nd,
        type_ndt,
        type_nt,
        type_bd,
        type_ip,
        type_uuid,
        type_tz,
    }
}

fn propercase(s: &str) -> String {
    let mut next_cap = true;
    let mut store: Vec<char> = Vec::new();
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
    }
    if store.last() == Some(&'s') {
        store.pop();
        if store.last() == Some(&'e') {
            store.pop();
            if store.last() == Some(&'i') {
                store.pop();
                store.push('y');
            } else {
                store.push('e');
            }
        }
    }
    store.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::io::prelude::*;

    fn file_get_contents(fname: &str) -> String {
        let mut f = ::std::fs::File::open(fname)
            .expect("File not found. Please run in the directory with schema.rs.");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("Something went wrong reading the file.");

        contents.replace("\r", "")
    }

    #[test]
    fn build_normal() {
        let parse_output = super::parse(
            file_get_contents("test_data/schema.rs"),
            "model",
            None,
            false,
            &mut HashMap::default(),
        );
        println!("str_proto shows as follow:\n{}", parse_output.str_proto);
        assert_eq!(parse_output.str_proto.chars().count(), 266);
        assert_eq!(parse_output.str_into_proto.chars().count(), 708);
        assert_eq!(parse_output.str_from_proto.chars().count(), 680);
        assert_eq!(parse_output.str_request.chars().count(), 109);
        assert_eq!(parse_output.str_rpc.chars().count(), 151);
        println!("str_model shows as follow:\n{}", parse_output.str_model);
        assert_eq!(
            parse_output.str_model,
            file_get_contents("test_data/expected_output/schema.rs")
        );
        assert_eq!(parse_output.type_nd, false);
        assert_eq!(parse_output.type_ndt, true);
        assert_eq!(parse_output.type_nt, false);
        assert_eq!(parse_output.type_bd, true);
        assert_eq!(parse_output.type_ip, false);
        assert_eq!(parse_output.type_uuid, false);
        assert_eq!(parse_output.type_tz, true);
    }

    #[test]
    fn build_with_localmodded() {
        let parse_output = super::parse(
            file_get_contents("test_data/schema_localmodded.rs"),
            "model",
            None,
            false,
            &mut HashMap::default(),
        );
        println!("{}", parse_output.str_model);
        assert_eq!(
            parse_output.str_model,
            file_get_contents("test_data/expected_output/schema_localmodded.rs")
        );
        assert_eq!(parse_output.type_nd, false);
        assert_eq!(parse_output.type_ndt, false);
        assert_eq!(parse_output.type_nt, false);
        assert_eq!(parse_output.type_bd, false);
        assert_eq!(parse_output.type_ip, false);
        assert_eq!(parse_output.type_uuid, false);
        assert_eq!(parse_output.type_tz, false);
    }

    #[test]
    fn build_with_ip_bytea() {
        let parse_output = super::parse(
            file_get_contents("test_data/schema_with_ip_bytea.rs"),
            "model",
            None,
            false,
            &mut HashMap::default(),
        );
        print!("{}", parse_output.str_model);
        assert_eq!(
            parse_output.str_model,
            file_get_contents("test_data/expected_output/schema_with_ip_bytea.rs")
        );
        assert_eq!(parse_output.type_nd, false);
        assert_eq!(parse_output.type_ndt, false);
        assert_eq!(parse_output.type_nt, false);
        assert_eq!(parse_output.type_bd, false);
        assert_eq!(parse_output.type_ip, true);
        assert_eq!(parse_output.type_uuid, false);
        assert_eq!(parse_output.type_tz, false);
    }

    #[test]
    fn build_with_tab() {
        let parse_output = super::parse(
            file_get_contents("test_data/schema_with_tab.rs"),
            "model",
            None,
            false,
            &mut HashMap::default(),
        );
        print!("{}", parse_output.str_model);

        assert_eq!(parse_output.str_model.chars().count(), 85);
    }

    #[test]
    fn build_with_time() {
        let parse_output = super::parse(
            file_get_contents("test_data/schema_with_time.rs"),
            "model",
            None,
            false,
            &mut HashMap::default(),
        );
        print!("{}", parse_output.str_model);
        assert_eq!(parse_output.str_model.chars().count(), 88);
        assert_eq!(parse_output.type_nt, true);
    }

    #[test]
    fn build_with_ies() {
        let parse_output = super::parse(
            file_get_contents("test_data/schema_with_ies.rs"),
            "model",
            None,
            false,
            &mut HashMap::default(),
        );
        print!("{}", parse_output.str_model);
        assert_eq!(
            parse_output.str_model,
            file_get_contents("test_data/expected_output/schema_with_ies.rs")
        );
    }

    #[test]
    fn build_with_identifiable() {
        let parse_output = super::parse(
            file_get_contents("test_data/schema.rs"),
            "model",
            Some("Identifiable".to_string()),
            false,
            &mut HashMap::default(),
        );
        print!("{}", parse_output.str_model);
    }

    #[test]
    fn build_with_uuid() {
        let parse_output = super::parse(
            file_get_contents("test_data/schema_uuid.rs"),
            "model",
            None,
            false,
            &mut HashMap::default(),
        );
        // print!("{}", parse_output.str_model);
        assert_eq!(parse_output.type_uuid, true);
        assert_eq!(parse_output.str_model.chars().count(), 183);
    }

    #[test]
    fn build_with_mysql() {
        let parse_output = super::parse(
            file_get_contents("test_data/schema_mysql.rs"),
            "model",
            None,
            false,
            &mut HashMap::default(),
        );
        print!("a:{}", parse_output.str_model);
        assert_eq!(
            parse_output.str_model,
            file_get_contents("test_data/expected_output/schema_mysql.rs")
        );
    }
}
