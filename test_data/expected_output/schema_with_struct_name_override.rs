#[derive(Queryable, Debug)]
#[diesel(table_name = my_table)]
pub struct MyOverriddenTable {
    pub id: i32,
}
