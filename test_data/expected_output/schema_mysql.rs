#[derive(Queryable, Debug)]
pub struct UnitsOfMeasure {
    pub id: u32,
    pub volumetricUnitId: Option<u32>,
    pub weightUnitId: Option<u32>,
    pub number: Option<f64>,
    pub createdAt: Option<NaiveDateTime>,
    pub updatedAt: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug)]
pub struct SomethingType {
    pub id: u32,
    pub value: String,
    pub name: String,
    pub active: bool,
    pub createdAt: Option<NaiveDateTime>,
    pub updatedAt: Option<NaiveDateTime>,
    pub inbound: bool,
}

#[derive(Queryable, Debug)]
pub struct Something {
    pub id: u32,
    pub somethingId: Option<u32>,
    pub somethingInOunces: f64,
    pub total: f64,
    pub createdAt: Option<NaiveDateTime>,
    pub updatedAt: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug)]
pub struct TextType {
    pub id: i32,
    pub tinytext: String,
    pub nullableTinytext: Option<String>,
    pub mediumtext: String,
    pub nullableMediumtext: Option<String>,
    pub longtext: String,
    pub nullableLongtext: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct NumericType {
    pub id: i32,
    pub double: f64,
    pub nullableDouble: Option<f64>,
    pub tinyint: i8,
    pub nullableTinyint: Option<i8>,
    pub smallint: i16,
    pub nullableSmallint: Option<i16>,
    pub bigint: i64,
    pub nullableBigint: Option<i64>,
}

#[derive(Queryable, Debug)]
pub struct UnsignedType {
    pub id: i32,
    pub unsignedTinyint: u8,
    pub nullableUnsignedTinyint: Option<u8>,
    pub unsignedSmallint: u16,
    pub nullableUnsignedSmallint: Option<u16>,
    pub bigint: u64,
    pub nullableBigint: Option<u64>,
}
