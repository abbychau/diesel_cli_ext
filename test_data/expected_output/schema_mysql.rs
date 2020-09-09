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
