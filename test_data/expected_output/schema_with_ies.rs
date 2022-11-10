#[derive(Queryable, Debug, Identifiable)]
#[diesel(primary_key(account_id))]
pub struct CarryOverProperty {
    pub account_id: Vec<u8>,
    pub debit: IpNetwork,
}
