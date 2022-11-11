#[derive(Queryable, Debug, Identifiable)]
#[diesel(primary_key(key))]
pub struct Setting {
    pub key: String,
    pub value: serde_json::Value,
}
