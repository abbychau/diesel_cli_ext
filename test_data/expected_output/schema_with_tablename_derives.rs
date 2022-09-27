#[derive(Queryable, Debug, Identifiable)]
#[primary_key(key)]
#[diesel(table_name = "settings")]
pub struct Setting {
    pub key: String,
    pub value: serde_json::Value,
}
