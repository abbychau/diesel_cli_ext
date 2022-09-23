#[derive(Queryable, Debug, Identifiable)]
#[primary_key(key)]
pub struct Setting {
    pub key: String,
    pub value: serde_json::Value,
}
