#[derive(Queryable, Debug)]
pub struct UnitsOfMeasure {
    pub id: u32,
    #[diesel(column_name = "volumetricUnitId")]
    pub volumetric_unit_id: Option<u32>,
    #[diesel(column_name = "weightUnitId")]
    pub weight_unit_id: Option<u32>,
    pub number: Option<f64>,
    #[diesel(column_name = "createdAt")]
    pub created_at: Option<NaiveDateTime>,
    #[diesel(column_name = "updatedAt")]
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug)]
pub struct SomethingType {
    pub id: u32,
    pub value: String,
    pub name: String,
    pub active: bool,
    #[diesel(column_name = "createdAt")]
    pub created_at: Option<NaiveDateTime>,
    #[diesel(column_name = "updatedAt")]
    pub updated_at: Option<NaiveDateTime>,
    pub inbound: bool,
}

#[derive(Queryable, Debug)]
pub struct Something {
    pub id: u32,
    #[diesel(column_name = "somethingId")]
    pub something_id: Option<u32>,
    #[diesel(column_name = "somethingInOunces")]
    pub something_in_ounces: f64,
    pub total: f64,
    #[diesel(column_name = "createdAt")]
    pub created_at: Option<NaiveDateTime>,
    #[diesel(column_name = "updatedAt")]
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug)]
pub struct TextType {
    pub id: i32,
    pub tinytext: String,
    #[diesel(column_name = "nullableTinytext")]
    pub nullable_tinytext: Option<String>,
    pub mediumtext: String,
    #[diesel(column_name = "nullableMediumtext")]
    pub nullable_mediumtext: Option<String>,
    pub longtext: String,
    #[diesel(column_name = "nullableLongtext")]
    pub nullable_longtext: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct NumericType {
    pub id: i32,
    pub double: f64,
    #[diesel(column_name = "nullableDouble")]
    pub nullable_double: Option<f64>,
    pub tinyint: i8,
    #[diesel(column_name = "nullableTinyint")]
    pub nullable_tinyint: Option<i8>,
    pub smallint: i16,
    #[diesel(column_name = "nullableSmallint")]
    pub nullable_smallint: Option<i16>,
    pub bigint: i64,
    #[diesel(column_name = "nullableBigint")]
    pub nullable_bigint: Option<i64>,
}

#[derive(Queryable, Debug)]
pub struct UnsignedType {
    pub id: i32,
    #[diesel(column_name = "unsignedTinyint")]
    pub unsigned_tinyint: u8,
    #[diesel(column_name = "nullableUnsignedTinyint")]
    pub nullable_unsigned_tinyint: Option<u8>,
    #[diesel(column_name = "unsignedSmallint")]
    pub unsigned_smallint: u16,
    #[diesel(column_name = "nullableUnsignedSmallint")]
    pub nullable_unsigned_smallint: Option<u16>,
    pub bigint: u64,
    #[diesel(column_name = "nullableBigint")]
    pub nullable_bigint: Option<u64>,
}

#[derive(Queryable, Debug)]
pub struct BlobType {
    pub id: i32,
    pub blob: Vec<u8>,
    #[diesel(column_name = "nullableBlob")]
    pub nullable_blob: Option<Vec<u8>>,
    pub tinyblob: Vec<u8>,
    #[diesel(column_name = "nullableTinyblob")]
    pub nullable_tinyblob: Option<Vec<u8>>,
    pub mediumblob: Vec<u8>,
    #[diesel(column_name = "nullableMediumblob")]
    pub nullable_mediumblob: Option<Vec<u8>>,
    pub longblob: Vec<u8>,
    #[diesel(column_name = "nullableLongblob")]
    pub nullable_longblob: Option<Vec<u8>>,
}

#[derive(Queryable, Debug)]
pub struct BinaryAndChar {
    pub id: i32,
    pub char: String,
    pub varchar: String,
    pub binary: Vec<u8>,
    pub varbinary: Vec<u8>,
}

#[derive(Queryable, Debug)]
pub struct UppercaseField {
    #[diesel(column_name = "ID")]
    pub id: i32,
    #[diesel(column_name = "NAME")]
    pub name: String,
    #[diesel(column_name = "CREATED_AT")]
    pub created_at: Option<NaiveDateTime>,
    #[diesel(column_name = "UPDATED_AT")]
    pub updated_at: Option<NaiveDateTime>,
}
