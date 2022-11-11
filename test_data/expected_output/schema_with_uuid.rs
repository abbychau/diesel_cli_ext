#[derive(Queryable, Debug, Identifiable)]
#[diesel(primary_key(account_id))]
pub struct CarryOverBalance {
    pub id: Uuid,
    pub debit: BigDecimal,
    pub description: Option<String>,
}
