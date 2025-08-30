use convert_case::{Case, Casing};
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
    pub type_jsonb: bool,
    pub diesel_macro_use: bool,
}

pub struct ParseArguments {
    pub contents: String,
    pub action: String,
    pub model_derives: Option<String>,
    pub add_table_name: bool,
    pub model_type_mapping: HashMap<String, String>,
    pub diesel_version: String,
    pub rust_styled_fields: bool,
    pub struct_name_override: HashMap<String, String>,
}

impl Default for ParseArguments {
    fn default() -> Self {
        Self {
            contents: Default::default(),
            action: Default::default(),
            model_derives: Default::default(),
            add_table_name: Default::default(),
            model_type_mapping: Default::default(),
            diesel_version: "2".into(),
            rust_styled_fields: Default::default(),
            struct_name_override: Default::default(),
        }
    }
}

pub fn parse(args: ParseArguments) -> ParseOutput {
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
        mut type_jsonb,
        mut diesel_macro_use,
    ) = (false, false, false, false, false, false, false, false, false);

    let mut count: u16 = 0;
    let mut struct_name: String = "".to_string();
    let content = args.contents.replace('\t', "    ");
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
        ("Jsonb", "serde_json::Value"),
        ("Uuid", "Uuid"),
        ("Char", "String"),
        ("Varchar", "String"),
        ("Bpchar", "String"),
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

    for (key, val) in args.model_type_mapping.iter() {
        model_type_dict.insert(key, val);
    }

    let mut is_schema = false;
    // let mut is_excluding = true;
    for line in lines {
        let cmp = line.trim().to_string();
        let vec: Vec<&str> = cmp.split(' ').collect();
        let indent_depth = if is_schema { 4 } else { 0 };

        if cmp.contains("#[") || cmp.contains("joinable!(") {
            //do nothing
        } else if cmp.contains("pub mod ") && cmp.trim() != "pub mod sql_types {" {
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
                    args.model_derives.as_deref().unwrap_or("Queryable, Debug")
                )
            ));
        } else if cmp.contains(") {") {
            // this line contains table name
            struct_name = vec[0].to_string();
            struct_name = match args.struct_name_override.get(&struct_name) {
                Some(struct_name) => struct_name.into(),
                None => propercase(&struct_name),
            };

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
            if vec.len() >= 3 {
                for c in &vec[1..vec.len() - 1] {
                    let pks = c.trim_matches(x);

                    pks_list.push(pks.to_string());
                }

                if pks_list.len() > 1 || pks_list[0] != "id" {
                    str_model = str_model.replace(
                        "{trace1}",
                        if args.model_derives.is_none()
                            || (args.model_derives.is_some()
                                && !args.model_derives.clone().unwrap().contains("Identifiable"))
                        {
                            ", Identifiable"
                        } else {
                            ""
                        },
                    );
                    if args.diesel_version == "2" {
                        str_model.push_str(&" ".repeat(indent_depth));
                        str_model.push_str("#[diesel(primary_key(");
                        str_model.push_str(&pks_list.join(", "));
                        str_model.push_str("))]\n");
                    } else {
                        str_model.push_str(&" ".repeat(indent_depth));
                        str_model.push_str("#[primary_key(");
                        str_model.push_str(&pks_list.join(", "));
                        str_model.push_str(")]\n");
                    }
                }
            }

            if args.add_table_name {
                if args.diesel_version == "2" {
                    // add #[diesel(table_name = "name")]
                    str_model.push_str(&format!(
                        "{}#[diesel(table_name = {})]\n",
                        " ".repeat(indent_depth),
                        vec[0].split('.').last().unwrap()
                    ));
                    diesel_macro_use = true;
                } else {
                    // add #[table_name = "name"]
                    str_model.push_str(&format!(
                        "{}#[table_name = \"{}\"]\n",
                        " ".repeat(indent_depth),
                        vec[0].split('.').last().unwrap()
                    ));
                }
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
            let _type = vec[2].replace(',', "");

            let dict = match args.action.as_str() {
                "model" => &model_type_dict,
                _ => &proto_type_dict,
            };
            let is_optional = _type.clone().trim().starts_with("Nullable<");
            let is_nullable_array = _type.clone().contains("Array<Nullable<");
            let vec_count = _type.clone().matches("Array").count();
            let b_position = _type.find('[').unwrap_or(_type.len());
            let mut single_type = _type.clone();
            single_type.truncate(b_position);
            let warning_for_longer_lifetime: String;
            let type_string: &str = match dict.get(
                single_type
                    .replace("Array<", "")
                    .replace("Nullable<", "")
                    .replace('>', "")
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
            if type_string == "jsonb" {
                type_jsonb = true;
            }

            let type_with_wrap = if is_nullable_array {
                format!(
                    "{}{}{}",
                    "Vec<Option<".repeat(vec_count),
                    type_string,
                    ">>".repeat(vec_count)
                )
            } else {
                format!(
                    "{}{}{}",
                    "Vec<".repeat(vec_count),
                    type_string,
                    ">".repeat(vec_count)
                )
            };

            if args.rust_styled_fields && !vec[0].is_case(Case::Snake) {
                let field_name = vec[0].to_case(Case::Snake);
                str_model.push_str(&format!(
                    "{}#[diesel(column_name = \"{}\")]\n{}pub {}: {},\n",
                    " ".repeat(indent_depth + 4),
                    &vec[0],
                    " ".repeat(indent_depth + 4),
                    field_name,
                    if is_optional {
                        format!("Option<{}>", type_with_wrap)
                    } else {
                        type_with_wrap
                    }
                ));
            } else {
                str_model.push_str(&format!(
                    "{}pub {}: {},\n",
                    " ".repeat(indent_depth + 4),
                    &vec[0],
                    if is_optional {
                        format!("Option<{}>", type_with_wrap)
                    } else {
                        type_with_wrap
                    }
                ));
            }
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

            str_proto.push_str(&format!("    {} {} = {};\n", type_string, &vec[0], count));
            str_from_proto.push_str(&format!(
                "            {}: i.get_{}(){},\n",
                &vec[0],
                &vec[0],
                match type_string {
                    "string" => ".to_string()",
                    "String" => ".to_string()",
                    "BigDecimal" => ".to_bigdecimal()",
                    _ => "",
                }
            ));
            str_into_proto.push_str(&format!(
                "        o.set_{}(i.{}{});\n",
                &vec[0],
                &vec[0],
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
        type_jsonb,
        diesel_macro_use,
    }
}

fn propercase(s: &str) -> String {
    let mut next_cap = true;
    let mut store: Vec<char> = Vec::new();
    for c in s.chars() {
        if c == '.' {
            store.clear();
            next_cap = true;
            continue;
        }
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
    use std::{collections::HashMap, io::prelude::*};

    use crate::parse::ParseArguments;
    use pretty_assertions::assert_eq;

    fn file_get_contents(fname: &str) -> String {
        let mut f = ::std::fs::File::open(fname)
            .expect("File not found. Please run in the directory with schema.rs.");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("Something went wrong reading the file.");

        contents.replace('\r', "")
    }

    #[test]
    fn build_normal() {
        let parse_output = super::parse(ParseArguments {
            action: "model".into(),
            contents: file_get_contents("test_data/schema.rs"),
            ..Default::default()
        });
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
        assert!(!parse_output.type_nd);
        assert!(parse_output.type_ndt);
        assert!(!parse_output.type_nt);
        assert!(parse_output.type_bd);
        assert!(!parse_output.type_ip);
        assert!(!parse_output.type_uuid);
        assert!(parse_output.type_tz);
    }

    #[test]
    fn build_with_localmodded() {
        let parse_output = super::parse(ParseArguments {
            contents: file_get_contents("test_data/schema_localmodded.rs"),
            action: "model".into(),
            ..Default::default()
        });
        println!("{}", parse_output.str_model);
        assert_eq!(
            parse_output.str_model,
            file_get_contents("test_data/expected_output/schema_localmodded.rs")
        );
        assert!(!parse_output.type_nd);
        assert!(!parse_output.type_ndt);
        assert!(!parse_output.type_nt);
        assert!(!parse_output.type_bd);
        assert!(!parse_output.type_ip);
        assert!(!parse_output.type_uuid);
        assert!(!parse_output.type_tz);
    }

    #[test]
    fn build_with_ip_bytea() {
        let parse_output = super::parse(ParseArguments {
            contents: file_get_contents("test_data/schema_with_ip_bytea.rs"),
            action: "model".into(),
            ..Default::default()
        });
        print!("{}", parse_output.str_model);
        assert_eq!(
            parse_output.str_model,
            file_get_contents("test_data/expected_output/schema_with_ip_bytea.rs")
        );
        assert!(!parse_output.type_nd);
        assert!(!parse_output.type_ndt);
        assert!(!parse_output.type_nt);
        assert!(!parse_output.type_bd);
        assert!(parse_output.type_ip);
        assert!(!parse_output.type_uuid);
        assert!(!parse_output.type_tz);
    }

    #[test]
    fn build_with_tab() {
        let parse_output = super::parse(ParseArguments {
            contents: file_get_contents("test_data/schema_with_tab.rs"),
            action: "model".into(),
            ..Default::default()
        });
        print!("{}", parse_output.str_model);

        assert_eq!(parse_output.str_model.chars().count(), 85);
    }

    #[test]
    fn build_with_time() {
        let parse_output = super::parse(ParseArguments {
            contents: file_get_contents("test_data/schema_with_time.rs"),
            action: "model".into(),
            ..Default::default()
        });
        print!("{}", parse_output.str_model);
        assert_eq!(parse_output.str_model.chars().count(), 88);
        assert!(parse_output.type_nt);
    }

    #[test]
    fn build_with_ies() {
        let parse_output = super::parse(ParseArguments {
            contents: file_get_contents("test_data/schema_with_ies.rs"),
            action: "model".into(),
            ..Default::default()
        });
        print!("{}", parse_output.str_model);
        assert_eq!(
            parse_output.str_model,
            file_get_contents("test_data/expected_output/schema_with_ies.rs")
        );
    }

    #[test]
    fn build_with_identifiable() {
        let parse_output = super::parse(ParseArguments {
            contents: file_get_contents("test_data/schema.rs"),
            action: "model".into(),
            ..Default::default()
        });
        print!("{}", parse_output.str_model);
    }

    #[test]
    fn build_with_uuid() {
        let parse_output = super::parse(ParseArguments {
            contents: file_get_contents("test_data/schema_uuid.rs"),
            action: "model".into(),
            ..Default::default()
        });
        assert!(parse_output.type_uuid);
        assert_eq!(
            parse_output.str_model,
            file_get_contents("test_data/expected_output/schema_with_uuid.rs")
        );
    }

    #[test]
    fn build_with_mysql() {
        let parse_output = super::parse(ParseArguments {
            contents: file_get_contents("test_data/schema_mysql.rs"),
            action: "model".into(),
            ..Default::default()
        });
        print!("a:{}", parse_output.str_model);
        assert_eq!(
            parse_output.str_model,
            file_get_contents("test_data/expected_output/schema_mysql.rs")
        );
    }

    #[test]
    fn build_with_jsonb() {
        let parse_output = super::parse(ParseArguments {
            contents: file_get_contents("test_data/schema_with_jsonb.rs"),
            action: "model".into(),
            ..Default::default()
        });
        print!("a:{}", parse_output.str_model);
        assert_eq!(
            parse_output.str_model,
            file_get_contents("test_data/expected_output/schema_jsonb.rs")
        );
    }

    #[test]
    fn build_with_tablename_derives() {
        let parse_output = super::parse(ParseArguments {
            contents: file_get_contents("test_data/schema_with_tablename_derives.rs"),
            action: "model".into(),
            add_table_name: true,
            ..Default::default()
        });
        print!("a:{}", parse_output.str_model);
        assert_eq!(
            parse_output.str_model,
            file_get_contents("test_data/expected_output/schema_with_tablename_derives.rs")
        );
    }

    #[test]
    fn build_with_rust_style_fields() {
        let parse_output = super::parse(ParseArguments {
            contents: file_get_contents("test_data/schema_with_rust_style_fields.rs"),
            action: "model".into(),
            rust_styled_fields: true,
            ..Default::default()
        });
        print!("a:{}", parse_output.str_model);
        assert_eq!(
            parse_output.str_model,
            file_get_contents("test_data/expected_output/schema_with_rust_style_fields.rs")
        );
    }

    #[test]
    fn build_with_struct_name_override() {
        let parse_output = super::parse(ParseArguments {
            contents: file_get_contents("test_data/schema_with_struct_name_override.rs"),
            action: "model".into(),
            add_table_name: true,
            struct_name_override: HashMap::from([(
                "my_table".to_string(),
                "MyOverriddenTable".to_string(),
            )]),
            ..Default::default()
        });
        print!("a:{}", parse_output.str_model);
        assert_eq!(
            parse_output.str_model,
            file_get_contents("test_data/expected_output/schema_with_struct_name_override.rs")
        );
    }
}
